use crate::{errors::StringError, labels::LoopLabel, AstLowerer};
use firefly_ast::value::{ElseStatement, IfStatement, Value as AstValue};
use firefly_hir::{resolve::SymbolTable, ty::{Ty, TyKind}, value::{ElseValue, IfValue, LiteralValue, Value as HirValue, ValueKind as HirValueKind, WhileValue}, Entity, Id};
use firefly_span::{Span, Spanned};
use itertools::Itertools;

impl AstLowerer {
    pub fn lower_value(&mut self, value: &Spanned<AstValue>, parent: Id<Entity>, symbol_table: &mut SymbolTable) -> HirValue  {
        let span = value.span;
        let (kind, ty) = match &value.item {
            AstValue::Tuple(items) => {
                let items = items.iter()
                    .map(|item| self.lower_value(item, parent, symbol_table))
                    .collect_vec();

                let types = items.iter()
                    .map(|item| item.ty.clone())
                    .collect_vec();

                let tuple_kind = HirValueKind::Tuple(items);
                let tuple_type = Ty::new(TyKind::Tuple(types), span);

                (tuple_kind, tuple_type)
            },
            AstValue::IntegerLiteral(num) => {
                // Remove the underscores
                let santized_num = num.item.replace("_", "");
                
                let int_kind = HirValueKind::Literal(LiteralValue::Integer(santized_num));
                let int_type = Ty::new(TyKind::Integer, span);

                (int_kind, int_type)
            }

            AstValue::StringLiteral(string) => {
                let sanitized_str = self.sanitize_string(&string.item, span);

                let str_kind = HirValueKind::Literal(LiteralValue::String(sanitized_str));
                let str_type = Ty::new(TyKind::String, span);

                (str_kind, str_type)
            }

            AstValue::Call(function, args) => {
                let function_value = self.lower_value(function, parent, symbol_table);

                let TyKind::Func(_, return_ty) = &function_value.ty.kind else {
                    panic!("error: called a non-callable value")
                };
                let return_ty = return_ty.as_ref().clone();

                let args = args.iter()
                               .map(|arg| self.lower_value(arg, parent, symbol_table))
                               .collect_vec();

                let invoke = HirValueKind::Invoke(Box::new(function_value), args);

                (invoke, return_ty)
            }

            AstValue::Path(path) => match self.resolve_value(path, parent, symbol_table) {
                Some(value) => { return value },
                None => (HirValueKind::Unit, Ty::new(TyKind::Unit, span))
            }

            AstValue::Return(return_value) => {
                let return_value = if let Some(return_value) = return_value {
                    self.lower_value(return_value, parent, symbol_table)
                } else {
                    let span = value.span;
                    HirValue::new(HirValueKind::Unit, Ty::new(TyKind::Unit, span), span)
                };

                (HirValueKind::Return(Box::new(return_value)), Ty::new(TyKind::Never, value.span))
            }

            AstValue::If(if_statement) => {
                let if_value = self.lower_if_statement(&if_statement, parent, symbol_table);

                (HirValueKind::If(Box::new(if_value)), Ty::new(TyKind::Unit, value.span))
            }

            AstValue::While(while_statement) => {
                let label = while_statement.label.as_ref().map(|label| self.lower_name(label));
                let condition = self.lower_value(&while_statement.condition, parent, symbol_table);

                self.label_stack.push(label.clone(), while_statement.body.id);
                let body = self.lower_code_block(&while_statement.body, parent, symbol_table);
                self.label_stack.pop();

                let while_value = WhileValue {
                    label,
                    condition,
                    body,
                };

                (HirValueKind::While(Box::new(while_value)), Ty::new(TyKind::Unit, value.span))
            }

            AstValue::Break(label) => {
                let found_label =
                    if let Some(label) = label {
                        self.label_stack.find(&label.item)
                    }
                    else {
                        self.label_stack.last()
                    };

                match found_label {
                    Some(LoopLabel { code_block, .. }) => {
                        (HirValueKind::Break(*code_block), Ty::new(TyKind::Never, value.span))
                    }
                    None => {
                        println!("error: label not found");
                        (HirValueKind::Unit, Ty::new(TyKind::Never, value.span))
                    }
                }
            }

            AstValue::Continue(label) => {
                let found_label =
                    if let Some(label) = label {
                        self.label_stack.find(&label.item)
                    }
                    else {
                        self.label_stack.last()
                    };

                match found_label {
                    Some(LoopLabel { code_block, .. }) => {
                        (HirValueKind::Continue(*code_block), Ty::new(TyKind::Never, value.span))
                    }
                    None => {
                        println!("error: label not found");
                        (HirValueKind::Unit, Ty::new(TyKind::Never, value.span))
                    }
                }
            }

            AstValue::Error => unreachable!()
        };

        HirValue::new(kind, ty, span)
    }

    fn lower_if_statement(&mut self, if_stmt: &IfStatement, parent: Id<Entity>, symbol_table: &mut SymbolTable) -> IfValue {
        let condition = self.lower_value(&if_stmt.condition, parent, symbol_table);

        let positive = self.lower_code_block(&if_stmt.positive, parent, symbol_table);
        let negative = if_stmt.negative.as_ref().map(|negative| match negative {
            ElseStatement::Else(code_block) => {
                ElseValue::Else(self.lower_code_block(code_block, parent, symbol_table))
            }
            ElseStatement::ElseIf(negative) => {
                ElseValue::ElseIf(Box::new(self.lower_if_statement(&negative, parent, symbol_table)))
            }
        });

        IfValue { condition, positive, negative }
    }

    fn sanitize_string(&self, s: &str, span: Span) -> String {
        let is_raw = s.starts_with("raw");
        let s = if is_raw { &s[3..] } else { s };

        let num_of_quotes = s.chars().take_while(|&c| c == '"').count();

        let mut inner = s[num_of_quotes..s.len() - num_of_quotes].to_string();

        if !is_raw {
            inner = self.unescape_string(&inner, span);
        }

        if num_of_quotes >= 3 {
            inner = self.unindent_string(&inner);
        }

        return inner.to_string();
    }

    fn unindent_string(&self, mut s: &str) -> String {
        let mut unindent = "";

        let last_newline = s.rfind('\n');

        if let Some(last_newline) = last_newline {
            let last_line = &s[last_newline + 1..];

            if last_line.chars().all(char::is_whitespace) {
                s = &s[..last_newline];
            }

            unindent = last_line;
        }

        let first_newline = s.find('\n');

        if let Some(first_newline) = first_newline {
            // If theres only whitespace before the first newline, remove it
            let first_line = &s[..first_newline];

            if first_line.chars().all(char::is_whitespace) {
                s = &s[first_newline + 1..];
            }
        }

        if unindent == "" {
            return s.to_string();
        }

        let unindented = s.lines().map(|line| {
            if line.starts_with(unindent) {
                &line[unindent.len()..]
            } else {
                line
            }
        }).join("\n");

        return unindented;
    }

    fn unescape_string(&self, s: &str, span: Span) -> String {
        let mut unescaped = String::with_capacity(s.len());

        let mut remaining = s.chars();

        while let Some(next) = remaining.next() {
            match next {
                '\\' => {
                    let c = match remaining.next().unwrap() {
                        '\\' => '\\',
                        '\"' => '\"',
                        '\'' => '\'',
                        '0' => '\0',
                        'a' => '\x07',
                        'b' => '\x08',
                        'e' => '\x1B',
                        'f' => '\x0C',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        'v' => '\x0B',

                        'x' => {
                            // take characters until we reach a non-hex character
                            let mut hex = String::new();

                            let n_hex_digits =
                                remaining.clone()
                                         .take(4)
                                         .take_while(char::is_ascii_hexdigit)
                                         .count();

                            for _ in 0..n_hex_digits {
                                hex.push(remaining.next().unwrap());
                            }

                            if n_hex_digits == 0 {
                                self.emit(StringError::NoHexSequence(span));
                                continue;
                            }

                            let Some(c) = char::from_u32(hex.parse().unwrap()) else {
                                self.emit(StringError::InvalidHexSequence(hex, span));
                                continue;
                            };

                            c
                        }

                        _ => {
                            self.emit(StringError::InvalidEscapeSequence(span));
                            continue;
                        }
                    };

                    unescaped.push(c)
                }
                c => unescaped.push(c)
            }
        }

        return unescaped;
    }

}
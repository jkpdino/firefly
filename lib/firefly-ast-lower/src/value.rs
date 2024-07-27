use crate::AstLowerer;
use firefly_ast::value::Value as AstValue;
use firefly_hir::{resolve::SymbolTable, ty::{Ty, TyKind}, value::{LiteralValue, Value as HirValue, ValueKind as HirValueKind}, Entity, Id};
use firefly_span::Spanned;
use itertools::Itertools;

impl AstLowerer {
    pub fn lower_value(&mut self, value: &Spanned<AstValue>, parent: Id<Entity>, symbol_table: &SymbolTable) -> HirValue  {
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
                let sanitized_str = self.sanitize_string(&string.item);

                println!("\"{sanitized_str}\"");

                let str_kind = HirValueKind::Literal(LiteralValue::String(sanitized_str));
                let str_type = Ty::new(TyKind::String, span);

                (str_kind, str_type)
            }

            AstValue::Call(_, _) => todo!(),

            AstValue::Path(path) => match self.resolve_value(path, parent, symbol_table) {
                Some(value) => { return value },
                None => (HirValueKind::Unit, Ty::new(TyKind::Unit, span))
            }
        };

        HirValue::new(kind, ty, span)
    }

    fn sanitize_string(&self, s: &str) -> String {
        let is_raw = s.starts_with("raw");
        let s = if is_raw { &s[3..] } else { s };

        let num_of_quotes = s.chars().take_while(|&c| c == '"').count();

        let mut inner = s[num_of_quotes..s.len() - num_of_quotes].to_string();

        if !is_raw {
            inner = self.unescape_string(&inner);
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

    fn unescape_string(&self, s: &str) -> String {
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
                                print!("error: invalid hex escape sequence");
                                continue;
                            }

                            let Some(c) = char::from_u32(hex.parse().unwrap()) else {
                                print!("error: invalid hex escape sequence");
                                continue;
                            };

                            c
                        }

                        _ => {
                            println!("error: invalid escape sequence");
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
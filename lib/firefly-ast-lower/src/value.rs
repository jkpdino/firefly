use crate::{
    errors::{StringError, TypeError, ValueError},
    labels::LoopLabel,
    resolve_condition::CallableResolveCondition,
    AstLowerer,
};
use firefly_ast::{
    operator::InfixOperator,
    value::{ElseStatement, IfStatement, Value as AstValue},
    PathSegment,
};
use firefly_hir::{
    resolve::SymbolTable,
    ty::{Ty, TyKind},
    value::{
        ElseValue, IfValue, LiteralValue, Value as HirValue, ValueKind as HirValueKind, WhileValue,
    },
    Entity, Id,
};
use firefly_span::{Span, Spanned};
use itertools::Itertools;

#[derive(Copy, Clone)]
pub struct LowerValueContext {
    is_in_operator: bool,
}

impl AstLowerer {
    pub fn lower_value(
        &mut self,
        value: &Spanned<AstValue>,
        parent: Id<Entity>,
        symbol_table: &mut SymbolTable,
        context: LowerValueContext,
    ) -> HirValue {
        let span = value.span;
        let (kind, ty) = match &value.item {
            AstValue::Tuple(items) if items.is_empty() => {
                (HirValueKind::Unit, Ty::new(TyKind::Unit, span))
            }
            AstValue::Tuple(items) if items.len() == 1 => {
                let HirValue { kind, ty, .. } =
                    self.lower_value(&items[0], parent, symbol_table, context.reset());

                (kind, ty)
            }
            AstValue::Tuple(items) => {
                let items = items
                    .iter()
                    .map(|item| self.lower_value(item, parent, symbol_table, context.reset()))
                    .collect_vec();

                let types = items.iter().map(|item| item.ty.clone()).collect_vec();

                let tuple_kind = HirValueKind::Tuple(items);
                let tuple_type = Ty::new(TyKind::Tuple(types), span);

                (tuple_kind, tuple_type)
            }
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

            AstValue::FloatLiteral(num) => {
                let sanitized_num = num.item.replace("_", "");

                let float_kind = HirValueKind::Literal(LiteralValue::Float(sanitized_num));
                let float_type = Ty::new(TyKind::Float, span);

                (float_kind, float_type)
            }

            AstValue::Call(function, args) => {
                let labels = args.iter().map(|arg| arg.label.clone()).collect_vec();

                let function_value =
                    self.lower_func_value(function, parent, symbol_table, labels, context.reset());

                let TyKind::Func(_, return_ty) = &function_value.ty.kind else {
                    self.emit(TypeError::CantCall(function_value.span));

                    return HirValue::default();
                };
                let return_ty = return_ty.as_ref().clone();

                let args = args
                    .iter()
                    .map(|arg| self.lower_value(&arg.value, parent, symbol_table, context.reset()))
                    .collect_vec();

                let invoke = HirValueKind::Invoke(Box::new(function_value), args);

                (invoke, return_ty)
            }

            AstValue::Path(path) => match self.resolve_value(path, parent, symbol_table) {
                Some(value) => return value,
                None => (HirValueKind::Unit, Ty::new(TyKind::Unit, span)),
            },

            AstValue::Member(parent_val, member) => {
                let parent_val =
                    self.lower_value(parent_val, parent, symbol_table, context.reset());

                if let Some(member) =
                    self.resolve_instance_member(parent_val, member.clone(), parent)
                {
                    return member;
                }

                return HirValue::default();
            }

            AstValue::TupleMember(parent_val, index) => {
                let parent_val =
                    self.lower_value(parent_val, parent, symbol_table, context.reset());

                let index_num: usize = index
                    .item
                    .parse()
                    .expect("internal compiler error: digits aren't a number");

                let TyKind::Tuple(items) = &parent_val.ty.kind else {
                    // error
                    return HirValue::default();
                };

                if index_num >= items.len() {
                    // error
                    return HirValue::default();
                }

                let ty = items[index_num].clone();

                (
                    HirValueKind::TupleMember(Box::new(parent_val), index_num),
                    ty,
                )
            }

            AstValue::Return(return_value) => {
                let return_value = if let Some(return_value) = return_value {
                    self.lower_value(return_value, parent, symbol_table, context.reset())
                } else {
                    let span = value.span;
                    HirValue::new(HirValueKind::Unit, Ty::new(TyKind::Unit, span), span)
                };

                (
                    HirValueKind::Return(Box::new(return_value)),
                    Ty::new(TyKind::Never, value.span),
                )
            }

            AstValue::If(if_statement) => {
                let if_value =
                    self.lower_if_statement(&if_statement, parent, symbol_table, context);

                (
                    HirValueKind::If(Box::new(if_value)),
                    Ty::new(TyKind::Unit, value.span),
                )
            }

            AstValue::While(while_statement) => {
                let label = while_statement
                    .label
                    .as_ref()
                    .map(|label| self.lower_name(label));
                let condition = self.lower_value(
                    &while_statement.condition,
                    parent,
                    symbol_table,
                    context.reset(),
                );

                self.label_stack
                    .push(label.clone(), while_statement.body.id);
                let body = self.lower_code_block(&while_statement.body, parent, symbol_table);
                self.label_stack.pop();

                let while_value = WhileValue {
                    label,
                    condition,
                    body,
                };

                (
                    HirValueKind::While(Box::new(while_value)),
                    Ty::new(TyKind::Unit, value.span),
                )
            }

            AstValue::Break(label) => {
                let found_label = if let Some(label) = label {
                    self.label_stack.find(&label.item)
                } else {
                    self.label_stack.last()
                };

                match found_label {
                    Some(LoopLabel { code_block, .. }) => (
                        HirValueKind::Break(*code_block),
                        Ty::new(TyKind::Never, value.span),
                    ),
                    None => {
                        if let Some(label) = label {
                            self.emit(ValueError::UndefinedBreakLabel(label.clone()));
                        } else {
                            self.emit(ValueError::BreakOutsideLoop(value.span));
                        }
                        (HirValueKind::Unit, Ty::new(TyKind::Never, value.span))
                    }
                }
            }

            AstValue::Continue(label) => {
                let found_label = if let Some(label) = label {
                    self.label_stack.find(&label.item)
                } else {
                    self.label_stack.last()
                };

                match found_label {
                    Some(LoopLabel { code_block, .. }) => (
                        HirValueKind::Continue(*code_block),
                        Ty::new(TyKind::Never, value.span),
                    ),
                    None => {
                        if let Some(label) = label {
                            self.emit(ValueError::UndefinedContinueLabel(label.clone()));
                        } else {
                            self.emit(ValueError::ContinueOutsideLoop(value.span));
                        }
                        (HirValueKind::Unit, Ty::new(TyKind::Never, value.span))
                    }
                }
            }

            AstValue::Assign(place, assignee) => {
                let place = self.lower_value(place, parent, symbol_table, context.reset());
                let assignee = self.lower_value(assignee, parent, symbol_table, context.reset());

                if !place.is_mutable() {
                    self.emit(ValueError::NotMutable(place.span));
                }

                (
                    HirValueKind::Assign(Box::new(place), Box::new(assignee)),
                    Ty::new(TyKind::Unit, value.span),
                )
            }

            AstValue::Prefix(op, value) => {
                let unit = self.lower_value(value, parent, symbol_table, context.reset());

                if let TyKind::Integer = unit.ty.kind {
                    return self.get_integer_prefix_operator(&op, unit, span).unwrap();
                } else if let TyKind::Float = unit.ty.kind {
                    return self.get_float_prefix_operator(&op, unit, span).unwrap();
                } else if let TyKind::Bool = unit.ty.kind {
                    return self.get_boolean_prefix_operator(&op, unit, span).unwrap();
                } else if let Some(operator_func) = self.resolve_instance_member(
                    unit,
                    PathSegment::new(Spanned::new(op.get_verb().into(), value.span)),
                    parent,
                ) {
                    let return_type = match &operator_func.ty.kind {
                        TyKind::Func(_, return_type) => return_type.as_ref().clone(),
                        _ => operator_func.ty.clone(),
                    };

                    (
                        HirValueKind::Invoke(Box::new(operator_func), vec![]),
                        return_type,
                    )
                } else {
                    todo!()
                }
            }

            AstValue::Infix(lhs, op, rhs) => {
                let (lhs, op, rhs) = self.reorganize(lhs, op, rhs, context.is_in_operator);

                let left = self.lower_value(&lhs, parent, symbol_table, context.in_operator());
                let right = self.lower_value(&rhs, parent, symbol_table, context.in_operator());

                if let InfixOperator::Assign = op {
                    return HirValue::new(
                        HirValueKind::Assign(Box::new(left), Box::new(right)),
                        Ty::new(TyKind::Unit, span),
                        span,
                    );
                } else if let TyKind::Integer = left.ty.kind {
                    return self.get_integer_operator(&op, left, right, span).unwrap();
                } else if let TyKind::Float = left.ty.kind {
                    return self.get_float_operator(&op, left, right, span).unwrap();
                } else if let TyKind::Bool = left.ty.kind {
                    return self.get_boolean_operator(&op, left, right, span).unwrap();
                } else if let Some(operator_func) = self.resolve_instance_member(
                    left,
                    PathSegment::new(Spanned::new(op.get_verb().into(), span)),
                    parent,
                ) {
                    let return_type = match &operator_func.ty.kind {
                        TyKind::Func(_, return_type) => return_type.as_ref().clone(),
                        _ => operator_func.ty.clone(),
                    };

                    return HirValue::new(
                        HirValueKind::Invoke(Box::new(operator_func), vec![right]),
                        return_type,
                        span,
                    );
                } else {
                    todo!()
                }
            }

            AstValue::Error => unreachable!(),
        };

        HirValue::new(kind, ty, span)
    }

    fn lower_func_value(
        &mut self,
        value: &Spanned<AstValue>,
        parent: Id<Entity>,
        symbol_table: &mut SymbolTable,
        labels: Vec<Option<Spanned<String>>>,
        context: LowerValueContext,
    ) -> HirValue {
        let span = value.span;

        let condition = CallableResolveCondition { labels };

        match &value.item {
            AstValue::Path(path) => {
                match self.resolve_value_with(path, parent, symbol_table, condition) {
                    Some(value) => return value,
                    None => {
                        return HirValue::new(
                            HirValueKind::Unit,
                            Ty::new(TyKind::Unit, span),
                            span,
                        );
                    }
                }
            }

            AstValue::Member(parent_val, member) => {
                let parent_val =
                    self.lower_value(parent_val, parent, symbol_table, context.reset());

                if let Some(member) =
                    self.resolve_instance_member_with(parent_val, member.clone(), parent, condition)
                {
                    return member;
                }

                return HirValue::default();
            }

            _ => {
                return self.lower_value(value, parent, symbol_table, context);
            }
        }
    }

    fn reorganize(
        &self,
        left: &Spanned<AstValue>,
        op: &InfixOperator,
        right: &Spanned<AstValue>,
        is_in_operator: bool,
    ) -> (Spanned<AstValue>, InfixOperator, Spanned<AstValue>) {
        if is_in_operator {
            return (left.clone(), op.clone(), right.clone());
        }

        match &left.item {
            AstValue::Infix(inner_lhs, inner_op, inner_rhs) => {
                let (inner_lhs, inner_op, inner_rhs) =
                    self.reorganize(&inner_lhs, inner_op, &inner_rhs, is_in_operator);

                if inner_op.precedence() >= op.precedence() {
                    //println!("{inner_lhs:?} {inner_op:?} {inner_rhs:?}");
                    return (inner_lhs, inner_op, inner_rhs);
                } else {
                    let new_left = inner_lhs;
                    let new_right = Spanned::new(
                        AstValue::Infix(Box::new(inner_rhs), op.clone(), Box::new(right.clone())),
                        Span::DUMMY,
                    );

                    //println!("{new_left:?} {inner_op:?} {new_right:?}");

                    return (new_left, inner_op, new_right);
                }
            }

            _ => {
                //println!("{left:?} {op:?} {right:?}");
                return (left.clone(), op.clone(), right.clone());
            }
        }
    }

    fn lower_if_statement(
        &mut self,
        if_stmt: &IfStatement,
        parent: Id<Entity>,
        symbol_table: &mut SymbolTable,
        context: LowerValueContext,
    ) -> IfValue {
        let condition = self.lower_value(&if_stmt.condition, parent, symbol_table, context);

        let positive = self.lower_code_block(&if_stmt.positive, parent, symbol_table);
        let negative = if_stmt.negative.as_ref().map(|negative| match negative {
            ElseStatement::Else(code_block) => {
                ElseValue::Else(self.lower_code_block(code_block, parent, symbol_table))
            }
            ElseStatement::ElseIf(negative) => ElseValue::ElseIf(Box::new(
                self.lower_if_statement(&negative, parent, symbol_table, context),
            )),
        });

        IfValue {
            condition,
            positive,
            negative,
        }
    }

    fn sanitize_string(&self, s: &str, span: Span) -> String {
        let is_raw = s.starts_with("raw");
        let s = if is_raw { &s[3..] } else { s };

        let num_of_quotes = s.chars().take_while(|&c| c == '"').count();
        let is_empty = num_of_quotes == s.len();

        if is_empty {
            return "".to_string();
        }

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

        let unindented = s
            .lines()
            .map(|line| {
                if line.starts_with(unindent) {
                    &line[unindent.len()..]
                } else {
                    line
                }
            })
            .join("\n");

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

                            let n_hex_digits = remaining
                                .clone()
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
                c => unescaped.push(c),
            }
        }

        return unescaped;
    }
}

impl LowerValueContext {
    pub fn in_operator(self) -> Self {
        Self {
            is_in_operator: true,
            ..self
        }
    }

    pub fn reset(self) -> Self {
        Self {
            is_in_operator: false,
            ..self
        }
    }
}

impl Default for LowerValueContext {
    fn default() -> Self {
        Self {
            is_in_operator: false,
        }
    }
}

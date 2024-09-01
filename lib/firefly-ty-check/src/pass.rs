use firefly_errors::emitter::Emitter;
use firefly_hir::{func::{Callable, Func}, stmt::{CodeBlock, Stmt, StmtKind}, value::{ElseValue, Value, ValueKind}, HirContext, Id};

use crate::{context::TypecheckContext, Typecheck};

// rewrite this entire module

pub fn type_check_context(context: &HirContext, emitter: &Emitter) {
    let type_context = TypecheckContext::new(context, emitter);

    context.entities()
           .filter_map(|id| context.cast_id::<Func>(id))
           .for_each(|function|
    {
        let Some(Callable { return_ty, .. }) = context.try_get(function) else {
            return;
        };

        let Some(code_block) = context.children(function.as_base()).iter().find_map(|child| context.cast_id::<CodeBlock>(*child)) else {
            return;
        };

        let typechecker = type_context.function_checker(return_ty);
        type_check(&typechecker, code_block, context);
    });
}

fn type_check(checker: &Typecheck, code_block: Id<CodeBlock>, context: &HirContext) {
    let code_block = context.get(code_block);

    for stmt in &code_block.stmts {
        type_check_stmt(checker, stmt, context);
    }

    if let Some(yields) = &code_block.yields {
        type_check_value(checker, yields, context);
    }
}

fn type_check_stmt(checker: &Typecheck, stmt: &Stmt, context: &HirContext) {
    checker.typecheck_statement(stmt);

    match &stmt.kind {
        StmtKind::Value(value) | StmtKind::Bind(_, _, value) => {
            type_check_value(checker, value, context);
        }
    }
}

fn type_check_value(checker: &Typecheck, value: &Value, context: &HirContext) {
    checker.typecheck_value(value);

    match &value.kind {
        ValueKind::Tuple(items) => {
            for item in items {
                type_check_value(checker, item, context);
            }
        }
        ValueKind::TupleMember(parent, _) => {
            type_check_value(checker, parent, context)
        }
        ValueKind::FieldOf(parent, _) => {
            type_check_value(checker, parent, context)
        }
        ValueKind::Assign(lhs, rhs) => {
            type_check_value(checker, lhs, context);
            type_check_value(checker, rhs, context)
        }
        ValueKind::InstanceFunc(instance, _) => {
            type_check_value(checker, instance, context)
        }
        ValueKind::Return(value) => {
            type_check_value(checker, value, context)
        }
        ValueKind::If(if_value) => {
            let mut last_if_value = Some(if_value);

            while let Some(if_value) = last_if_value.take() {
                type_check_value(checker, &if_value.condition, context);
                type_check(checker, if_value.positive, context);

                match &if_value.negative {
                    Some(ElseValue::ElseIf(else_if_value)) => {
                        last_if_value = Some(else_if_value);
                    }
                    Some(ElseValue::Else(id)) => {
                        type_check(checker, *id, context)
                    }
                    None => {}
                }
            }
        }
        ValueKind::While(while_value) => {
            type_check_value(checker, &while_value.condition, context);
            type_check(checker, while_value.body, context)
        }
        ValueKind::Invoke(func, args) => {
            type_check_value(checker, &func, context);
            for arg in args {
                type_check_value(checker, arg, context)
            }
        }

        _ => { }
    }
}
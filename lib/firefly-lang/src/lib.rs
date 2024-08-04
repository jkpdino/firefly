use firefly_hir::{
    items::{Constant, Module, TypeAlias}, resolve::{Import, Symbol}, ty::{Ty, TyKind}, value::{HasValue, LiteralValue, Value, ValueKind}, AccessComponent, BaseComponent, Component, HirContext, Id, Name, Visibility
};
use firefly_span::Span;

pub fn create_lang_module(context: &mut HirContext) {
    let root = context.root();

    let typealias = |kind: TyKind| TypeAlias {
        id: Id::default(),
        ty: Ty::new_unspanned(kind),
    };

    let lang_id = create("lang", Module { id: Id::default() }, root, context);

    create("int", typealias(TyKind::Integer), lang_id, context);
    create("string", typealias(TyKind::String), lang_id, context);
    create("bool", typealias(TyKind::Bool), lang_id, context);
    create("float", typealias(TyKind::Float), lang_id, context);

    const INT_OPERATORS: &[&str] = &[
        "add", "sub", "mul", "div", "rem",
        "left_shift", "right_shift",
        "bitand", "bitor", "bitxor",
    ];

    const INT_COMPARES: &[&str] = &[
        "eq_int", "neq_int",
        "gt_int", "geq_int",
        "lt_int", "leq_int"
    ];

    const BOOL_OPERATORS: &[&str] = &[
        "and", "or", "xor",
        "eq_bool", "neq_bool"
    ];

    for name in INT_OPERATORS {
        create_func(name, &[TyKind::Integer, TyKind::Integer], TyKind::Integer, lang_id, context)
    }

    create_func("bitnot", &[TyKind::Integer], TyKind::Integer, lang_id, context);

    for name in INT_COMPARES {
        create_func(name, &[TyKind::Integer, TyKind::Integer], TyKind::Bool, lang_id, context)
    }

    create_func("parse_int", &[TyKind::String], TyKind::Integer, lang_id, context);
    create_func("format_int", &[TyKind::Integer], TyKind::String, lang_id, context);

    for name in BOOL_OPERATORS {
        create_func(name, &[TyKind::Bool, TyKind::Bool], TyKind::Bool, lang_id, context);
    }

    create_func("not", &[TyKind::Bool], TyKind::Bool, lang_id, context);
    create_func("parse_bool", &[TyKind::String], TyKind::Bool, lang_id, context);
    create_func("format_bool", &[TyKind::Bool], TyKind::String, lang_id, context);

    create_func("print", &[TyKind::String], TyKind::Unit, lang_id, context);
    create_func("concat", &[TyKind::String, TyKind::String], TyKind::String, lang_id, context);
    create_func("len", &[TyKind::String], TyKind::Integer, lang_id, context);
    create_func("eq_str", &[TyKind::String, TyKind::String], TyKind::Bool, lang_id, context);
    create_func("neq_str", &[TyKind::String, TyKind::String], TyKind::Bool, lang_id, context);

    create_literal("true", ValueKind::Literal(LiteralValue::Boolean(true)), TyKind::Bool, lang_id, context);
    create_literal("false", ValueKind::Literal(LiteralValue::Boolean(false)), TyKind::Bool, lang_id, context);

    let root = context.root();
    let import_id = context.create(Import::import(Default::default(), lang_id.as_base()));
    context.link(root, import_id);
}

fn create<Base: BaseComponent>(
    name: &str,
    base: Base,
    parent: Id<impl Component>,
    context: &mut HirContext,
) -> Id<Base>
where
    HirContext: AccessComponent<Base>,
    HirContext: AccessComponent<Symbol>,
{
    let id = context.create(base);

    let symbol = Symbol {
        name: Name {
            name: name.to_string(),
            span: Span::default(),
        },
        visibility: Visibility::Public,
        is_static: true,
    };

    context.add_component::<Symbol>(id.as_base(), symbol);
    context.link(parent, id);

    id
}

fn create_literal(
    name: &'static str,
    value_kind: ValueKind,
    ty_kind: TyKind,
    parent: Id<impl Component>,
    context: &mut HirContext)
{
    let ty = Ty::new_unspanned(ty_kind);

    let value = Value::new(
        value_kind,
        ty,
        Span::default()
    );

    context.create_with_parent(parent, (
        Constant::default(),
        Symbol {
            name: Name::internal(name),
            visibility: Visibility::Public,
            is_static: true,
        },
        HasValue {
            value
        }
    ));
}

fn create_func(
    name: &'static str,
    params: &[TyKind],
    return_ty: TyKind,
    parent: Id<impl Component>,
    context: &mut HirContext
) {
    let params = params.iter().map(|p| Ty::new_unspanned(p.clone())).collect();
    let return_ty = Box::new(Ty::new_unspanned(return_ty));

    let ty = Ty::new_unspanned(TyKind::Func(params, return_ty));

    let value = Value::new(
        ValueKind::BuiltinFunc(name),
        ty,
        Span::default()
    );

    context.create_with_parent(parent, (
        Constant::default(),
        Symbol {
            name: Name::internal(name),
            visibility: Visibility::Public,
            is_static: true,
        },
        HasValue {
            value
        }
    ));
}
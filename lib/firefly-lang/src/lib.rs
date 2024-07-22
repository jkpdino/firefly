use firefly_hir::{
    items::{Module, TypeAlias},
    resolve::Symbol,
    ty::{Ty, TyKind},
    AccessComponent, BaseComponent, Component, HirContext, Id, Name, Visibility,
};
use firefly_span::Span;

pub fn create_lang_module(context: &mut HirContext) {
    let root = context.root();

    let typealias = |kind: TyKind| TypeAlias {
        id: Id::default(),
        ty: Ty {
            kind,
            span: Default::default(),
        },
    };

    let lang_id = create("lang", Module { id: Id::default() }, root, context);

    let int = create("int", typealias(TyKind::Integer), lang_id, context);
    let string = create("string", typealias(TyKind::String), lang_id, context);
    let bool = create("bool", typealias(TyKind::Bool), lang_id, context);
    let float = create("float", typealias(TyKind::Float), lang_id, context);
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
    };

    context.add_component::<Symbol>(id.as_base(), symbol);
    context.link(parent, id);

    id
}

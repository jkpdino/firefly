use crate::{func::Func, HirContext, Id};

#[test]
fn test_base_tree() {
    let mut context = HirContext::new();

    let root = context.root();

    let func1 = Id::default();
    let func2 = Id::default();

    context.create(Func { id: func1 });
    context.create(Func { id: func2 });

    context.link(root, func1);
    context.link(root, func2);
}

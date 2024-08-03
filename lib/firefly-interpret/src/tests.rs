use firefly_span::Span;

use crate::{builder::Builder, ir::{ty::{Ty, TyKind}, value::{BinaryIntrinsic, Comparison, ConstantValue, Immediate, ImmediateKind}, VirContext}, util::Id};

#[test]
pub fn basic() {
    let mut context = VirContext::new();
    let mut builder = Builder::new(&mut context);

    let func_id = 
        builder.context_mut()
            .create_function(
                "main",
                vec![ Ty::new(TyKind::Integer) ],
                Ty::new(TyKind::Void));

    builder.select_func(func_id);
    builder.append_basic_block();

    let local0 = builder.get_local(Id::new(0)).place_unspanned();
    let local1 = builder.build_local(Ty::new(TyKind::Bool)).place_unspanned();
    let local2 = builder.build_local(Ty::new(TyKind::Integer));
    let local3 = builder.build_local(Ty::new(TyKind::Integer));

    let compare = Immediate {
        kind: Box::new(ImmediateKind::Binary(BinaryIntrinsic::Compare(Comparison::LessThanOrEqual),
            local0.move_out(),
            Immediate {
                kind: Box::new(ImmediateKind::Constant(ConstantValue::Integer(1))),
                ty: Ty::new(TyKind::Integer),
                span: Default::default()
            }
        )),
        ty: Ty::new(TyKind::Bool),
        span: Span::default(),
    };
    builder.build_assign(local1, compare);




    println!("{context}");
}
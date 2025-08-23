use firefly_mir::{
    ty::{Ty, TyKind},
    MirContext,
};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InnerValue {
    Integer(u64),
    String(String),
    Boolean(bool),
    Float(f64),

    Struct(Vec<Value>),

    Void,

    Undefined,
}

pub type Value = Box<InnerValue>;

pub fn default_for_ty(ty: &Ty, context: &MirContext) -> Value {
    let inner = match &*ty.kind {
        TyKind::Integer => InnerValue::Integer(0),
        TyKind::String => InnerValue::String(String::new()),
        TyKind::Bool => InnerValue::Boolean(false),
        TyKind::Float => InnerValue::Float(0.0),
        TyKind::Void => InnerValue::Void,
        TyKind::Struct(id) => {
            let struct_def = context.get_struct(*id);

            InnerValue::Struct(
                struct_def
                    .fields
                    .iter()
                    .map(|ty| default_for_ty(ty, context))
                    .collect_vec(),
            )
        }
        TyKind::Tuple(items) => InnerValue::Struct(
            items
                .iter()
                .map(|ty| default_for_ty(ty, context))
                .collect_vec(),
        ),
        TyKind::Func(items, ty) => InnerValue::Undefined,
    };

    return Box::new(inner);
}

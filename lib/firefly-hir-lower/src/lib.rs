mod items;
mod ty;

use std::collections::HashMap;

use firefly_hir::{items::StructDef as HirStruct, HirContext, Id as HirId, func::Func as HirFunc};
use firefly_interpret::{ir::{code::Function as VirFunc, ty::struct_def::StructDef as VirStruct, VirContext}, util::Id as VirId};

pub struct HirLowerer<'a> {
    vir: VirContext,
    hir: &'a HirContext,

    struct_map: HashMap<HirId<HirStruct>, VirId<VirStruct>>,
    func_map: HashMap<HirId<HirFunc>, VirId<VirFunc>>
}

pub fn lower<'a>(hir: &'a HirContext) -> VirContext {
    let vir = VirContext::new();

    let mut lowerer = HirLowerer {
        vir,
        hir,

        struct_map: HashMap::new(),
        func_map: HashMap::new(),
    };

    lowerer.hir.entities()
               .filter_map(|entity| lowerer.hir.cast_id(entity))
               .for_each(|item|
    {
        lowerer.create_struct(item);
    });

    lowerer.hir.entities()
               .filter_map(|entity| lowerer.hir.cast_id(entity))
               .for_each(|item|
    {
        lowerer.create_func(item);
    });

    return lowerer.vir;
}
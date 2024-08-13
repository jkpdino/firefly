mod items;
mod ty;
mod value;
mod code;

use std::collections::HashMap;

use firefly_hir::{func::Func as HirFunc, items::{Field, Global as HirGlobal, StructDef as HirStruct}, stmt::{CodeBlock, Local as HirLocal}, HirContext, Id as HirId};
use firefly_interpret::{builder::Builder, ir::{code::{Function as VirFunc, Global as VirGlobal, Local as VirLocal}, ty::struct_def::StructDef as VirStruct, VirContext}, util::Id as VirId};
use value::loops::LoopMarker;

pub struct HirLowerer<'a> {
    vir: Builder<'a>,
    hir: &'a HirContext,

    struct_map: HashMap<HirId<HirStruct>, VirId<VirStruct>>,
    func_map: HashMap<HirId<HirFunc>, VirId<VirFunc>>,
    local_map: HashMap<HirId<HirLocal>, VirId<VirLocal>>,
    global_map: HashMap<HirId<HirGlobal>, VirId<VirGlobal>>,
    field_map: HashMap<HirId<Field>, usize>,
    loop_map: HashMap<HirId<CodeBlock>, LoopMarker>
}

pub fn lower<'a>(hir: &'a HirContext, vir: &'a mut VirContext) {
    let vir = Builder::new(vir);

    let mut lowerer = HirLowerer {
        vir,
        hir,

        loop_map:   HashMap::new(),
        func_map:   HashMap::new(),
        local_map:  HashMap::new(),
        field_map:  HashMap::new(),
        global_map: HashMap::new(),
        struct_map: HashMap::new(),
    };

    lowerer.hir.entities()
               .filter_map(|entity| lowerer.hir.cast_id(entity))
               .for_each(|item| lowerer.create_struct(item));

    lowerer.hir.entities()
               .filter_map(|entity| lowerer.hir.cast_id(entity))
               .for_each(|item| lowerer.create_func(item));

    lowerer.hir.entities()
               .filter_map(|entity| lowerer.hir.cast_id(entity))
               .for_each(|item| lowerer.create_global(item));

    // todo: initialize the globals

    lowerer.hir.entities()
               .filter_map(|entity| lowerer.hir.cast_id(entity))
               .for_each(|item| lowerer.lower_struct(item));

    lowerer.hir.entities()
               .filter_map(|entity| lowerer.hir.cast_id(entity))
               .for_each(|item| lowerer.lower_func(item));
}
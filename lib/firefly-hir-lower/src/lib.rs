mod items;
mod ty;
mod value;
mod code;

use std::collections::HashMap;

use firefly_hir::{func::Func as HirFunc, items::{Field, Global as HirGlobal, StructDef as HirStruct}, stmt::{CodeBlock, Local as HirLocal}, HirContext, Id as HirId};
use firefly_mir::{builder::Builder, code::{Function as MirFunc, Global as MirGlobal, Local as MirLocal}, ty::struct_def::StructDef as MirStruct, MirContext, Id as MirId};
use value::loops::LoopMarker;

pub struct HirLowerer<'a> {
    mir: Builder<'a>,
    hir: &'a HirContext,

    struct_map: HashMap<HirId<HirStruct>, MirId<MirStruct>>,
    func_map: HashMap<HirId<HirFunc>, MirId<MirFunc>>,
    local_map: HashMap<HirId<HirLocal>, MirId<MirLocal>>,
    global_map: HashMap<HirId<HirGlobal>, MirId<MirGlobal>>,
    field_map: HashMap<HirId<Field>, usize>,
    loop_map: HashMap<HirId<CodeBlock>, LoopMarker>
}

pub fn lower<'a>(hir: &'a HirContext, mir: &'a mut MirContext) {
    let mir = Builder::new(mir);

    let mut lowerer = HirLowerer {
        mir,
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
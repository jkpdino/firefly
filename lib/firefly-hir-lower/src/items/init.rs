use firefly_hir::{
    func::{Callable, Init},
    items::mangle::MangledName,
    stmt::CodeBlock,
    value::HasSelf,
    Id,
};
use itertools::Itertools;

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn create_init(&mut self, init: Id<Init>) {
        let MangledName { symbol } = self
            .hir
            .try_get_computed(init)
            .cloned()
            .expect("internal compiler error: initializer doesn't have a mangled name");

        let Callable {
            params, return_ty, ..
        } = self
            .hir
            .try_get(init)
            .expect("internal compiler error: initializer doesn't have a signature");

        // create the function
        let mir_params = params.iter().map(|p| self.lower_ty(&p.ty)).collect_vec();
        
        // Init functions return the initialized struct, not Unit
        let return_ty = if let Some(HasSelf { ty, .. }) = self.hir.try_get::<HasSelf>(init) {
            self.lower_ty(ty)
        } else {
            self.lower_ty(return_ty)
        };

        // Init functions don't take self as a parameter, they create and return self
        // (commenting out the self parameter addition)
        // if let Some(HasSelf { ty, .. }) = self.hir.try_get::<HasSelf>(init) {
        //     let ty = self.lower_ty(ty);
        //     mir_params.insert(0, ty);
        // }

        let mir_id = self
            .mir
            .context_mut()
            .create_function(&symbol, mir_params, return_ty);

        // Create locals for parameters first (so they get indices 0, 1, 2...)
        for param in params {
            let ty = self.lower_ty(&param.ty);
            let mir_local = self.mir.context_mut().create_local(mir_id, ty);
            self.local_map.insert(param.id, mir_local.id());
        }

        // create a local for self in the function (after parameters)
        if let Some(HasSelf { local, ty }) = self.hir.try_get::<HasSelf>(init) {
            let ty = self.lower_ty(ty);
            let mir_local = self.mir.context_mut().create_local(mir_id, ty);
            self.local_map.insert(*local, mir_local.id());
        }

        self.init_map.insert(init, mir_id);
    }

    pub fn lower_init(&mut self, init: Id<Init>) {
        let mir_id = *self.init_map.get(&init).unwrap();

        self.mir.select_func(mir_id);

        let Some(code_block) = self
            .hir
            .children(init.as_base())
            .iter()
            .find_map(|child| self.hir.cast_id::<CodeBlock>(*child))
        else {
            return;
        };

        // Lower the code
        let bb0 = self.mir.append_basic_block();
        self.mir.select_basic_block(bb0);

        // Initialize self with default values first
        if let Some(HasSelf { local, ty }) = self.hir.try_get::<HasSelf>(init) {
            use firefly_mir::value::{Immediate, ImmediateKind};
            
            let ty = self.lower_ty(ty);
            let self_local = self.local_map[local];
            let self_place = self.mir.get_local(self_local).place_unspanned();
            
            // Create default struct initialization with proper empty fields
            use firefly_mir::value::{ConstantValue};
            
            let default_fields = if let firefly_mir::ty::TyKind::Struct(struct_id) = &*ty.kind {
                let struct_def = self.mir.context_mut().get_struct(*struct_id);
                struct_def.fields.iter().map(|field_ty| {
                    // Create default values for each field type
                    match &*field_ty.kind {
                        firefly_mir::ty::TyKind::String => Immediate {
                            kind: Box::new(ImmediateKind::Constant(ConstantValue::String(String::new()))),
                            ty: field_ty.clone(),
                            span: Default::default(),
                        },
                        firefly_mir::ty::TyKind::Integer => Immediate {
                            kind: Box::new(ImmediateKind::Constant(ConstantValue::Integer(0))),
                            ty: field_ty.clone(),
                            span: Default::default(),
                        },
                        firefly_mir::ty::TyKind::Bool => Immediate {
                            kind: Box::new(ImmediateKind::Constant(ConstantValue::Bool(false))),
                            ty: field_ty.clone(),
                            span: Default::default(),
                        },
                        firefly_mir::ty::TyKind::Float => Immediate {
                            kind: Box::new(ImmediateKind::Constant(ConstantValue::Float(0.0))),
                            ty: field_ty.clone(),
                            span: Default::default(),
                        },
                        _ => Immediate {
                            kind: Box::new(ImmediateKind::Void), // Fallback for other types
                            ty: field_ty.clone(),
                            span: Default::default(),
                        }
                    }
                }).collect()
            } else {
                vec![]
            };
            
            let default_init = Immediate {
                kind: Box::new(ImmediateKind::Struct(default_fields)),
                ty: ty.clone(),
                span: Default::default(),
            };
            
            self.mir.build_assign(self_place, default_init);
        }

        // Use lower_code_block_inner instead of lower_code_block_func to avoid
        // automatic return terminator addition
        let _code_result = self.lower_code_block_inner(code_block);

        // Add explicit return of self for init functions
        if let Some(HasSelf { local, .. }) = self.hir.try_get::<HasSelf>(init) {
            use firefly_mir::code::Terminator;
            
            let self_local = self.local_map[local];
            let self_place = self.mir.get_local(self_local).place_unspanned();
            let return_value = self_place.move_out();
            
            self.mir.build_terminator(Terminator::returns(return_value));
        } else {
            // Fallback to void return if no self
            use firefly_mir::code::Terminator;
            self.mir.build_terminator(Terminator::returns_void());
        }
    }
}

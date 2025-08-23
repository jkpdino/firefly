use firefly_hir::{stmt::{CodeBlock, Stmt, StmtKind}, Id};
use firefly_mir::{code::Terminator, value::{Immediate, ImmediateKind}};

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn lower_code_block_func(&mut self, code_block: Id<CodeBlock>) {
        let imm = self.lower_code_block_inner(code_block);

        if self.mir.is_terminated() {
            return;
        }

        if let Some(imm) = imm {
            self.mir.build_terminator(Terminator::returns(imm));
        }
        else {
            self.mir.build_terminator(Terminator::returns_void());
        }
    }

    pub fn lower_code_block(&mut self, code_block: Id<CodeBlock>) {
        let imm = self.lower_code_block_inner(code_block);

        if let Some(imm) = imm {
            self.mir.build_eval(imm);
        }
    }
    pub fn lower_code_block_inner(&mut self, code_block: Id<CodeBlock>) -> Option<Immediate> {
        let code_block = self.hir.get(code_block).clone();

        for stmt in &code_block.stmts {
            self.lower_stmt(stmt);
        }

        if let Some(yields) = &code_block.yields {
            let yields = self.lower_immediate(&yields);

            if let ImmediateKind::Void = yields.kind.as_ref() {
                return None
            }

            return Some(yields)
        }

        return None
    }
    pub fn lower_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Value(value) => {
                use firefly_hir::value::ValueKind;
                
                // Check if this is a method call that needs to assign back to receiver
                if let ValueKind::Invoke(func, _args) = &value.kind {
                    if let ValueKind::InstanceFunc(receiver, _method_id) = &func.kind {
                        // This is a method call - we need to assign the result back to the receiver
                        let receiver_place = self.lower_place(receiver);
                        let method_result = self.lower_immediate(value);
                        
                        // Assign the method result back to the receiver
                        self.mir.build_assign(receiver_place, method_result);
                        return;
                    }
                }

                let imm = self.lower_immediate(value);

                if let ImmediateKind::Void = imm.kind.as_ref() {
                    return;
                }

                self.mir.build_eval(imm);
            }
            StmtKind::Bind(_, local_id, ty, value) => {
                let ty = self.lower_ty(ty);
                let local = self.mir.build_local(ty);
                self.local_map.insert(*local_id, local.id());
                
                let local_place = local.place_unspanned();
                let imm = self.lower_immediate(value);

                self.mir.build_assign(local_place, imm);
            }
        }
    }
}
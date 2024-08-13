use firefly_hir::{stmt::{CodeBlock, Stmt, StmtKind}, Id};
use firefly_mir::value::ImmediateKind;

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn lower_code_block(&mut self, code_block: Id<CodeBlock>) {
        let code_block = self.hir.get(code_block);

        for stmt in &code_block.stmts {
            self.lower_stmt(stmt);
        }
    }
    pub fn lower_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Value(value) => {
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
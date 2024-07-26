use crate::AstLowerer;
use firefly_ast::stmt::{CodeBlock as AstCodeBlock, Stmt as AstStmt};
use firefly_hir::{resolve::SymbolTable, stmt::{CodeBlock as HirCodeBlock, Stmt as HirStmt, StmtKind as HirStmtKind}, Entity, Id};
use firefly_span::Spanned;
use itertools::Itertools;

impl AstLowerer {
    pub fn lower_code_block(&mut self, code_block: &AstCodeBlock, parent: Id<Entity>, symbol_table: &mut SymbolTable) {
        symbol_table.push_scope();

        let code_block_id = Id::<HirCodeBlock>::default();

        let stmts = code_block.stmts.iter()
            .map(|stmt| self.lower_stmt(stmt, code_block_id, symbol_table))
            .collect_vec();

        self.context.create(HirCodeBlock {
            id: code_block_id,
            stmts,
            span: Default::default(),
        });
        self.context.link(parent, code_block_id);

        symbol_table.pop_scope();
    }

    pub fn lower_stmt(&mut self, stmt: &Spanned<AstStmt>, parent: Id<HirCodeBlock>, symbol_table: &mut SymbolTable) -> HirStmt {
        match &stmt.item {
            AstStmt::Value(value) => {
                let value = self.lower_value(&value, parent.as_base(), symbol_table);

                HirStmt::new(
                    HirStmtKind::Value(value),
                    stmt.span
                )
            }

            AstStmt::Bind(name, ty, value) => {
                let name = self.lower_name(name);
                let ty = self.lower_ty(&ty, parent.as_base(), symbol_table);
                let value = self.lower_value(&value, parent.as_base(), symbol_table);

                // Create a local so we can reference the symbol
                self.create_local(parent.as_base(), &name, &ty);

                // Now return a statement
                HirStmt::new(
                    HirStmtKind::Bind(name, ty, value),
                    stmt.span
                )
            }
        }
    }
}
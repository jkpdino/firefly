use crate::{value::LowerValueContext, AstLowerer};
use firefly_ast::stmt::{CodeBlock as AstCodeBlock, Stmt as AstStmt};
use firefly_hir::{
    resolve::{Symbol, SymbolTable},
    stmt::{CodeBlock as HirCodeBlock, Stmt as HirStmt, StmtKind as HirStmtKind},
    Entity, Id,
};
use firefly_span::Spanned;
use itertools::Itertools;

impl AstLowerer {
    pub fn lower_code_block(
        &mut self,
        code_block: &AstCodeBlock,
        parent: Id<Entity>,
        symbol_table: &mut SymbolTable,
    ) -> Id<HirCodeBlock> {
        self.context.link(parent, code_block.id);

        symbol_table.push_scope();

        let (mut stmts, mut yields) = (&code_block.stmts[..], code_block.yields.as_ref());

        if code_block.yields.is_none() {
            if let Some(last) = code_block.stmts.last() {
                if let AstStmt::Value(new_yields, false) = &last.item {
                    stmts = &stmts[..stmts.len() - 1];
                    yields = Some(new_yields);
                }
            }
        }

        let stmts = stmts
            .iter()
            .filter_map(|stmt| self.lower_stmt(stmt, code_block.id, symbol_table))
            .collect_vec();

        let yields = yields.map(|yields| {
            self.lower_value(
                yields,
                code_block.id.as_base(),
                symbol_table,
                Default::default(),
            )
        });

        self.context.create(HirCodeBlock {
            id: code_block.id,
            stmts,
            yields,
            span: Default::default(),
        });

        symbol_table.pop_scope();

        return code_block.id;
    }

    pub fn lower_stmt(
        &mut self,
        stmt: &Spanned<AstStmt>,
        parent: Id<HirCodeBlock>,
        symbol_table: &mut SymbolTable,
    ) -> Option<HirStmt> {
        let stmt = match &stmt.item {
            AstStmt::Value(value, _) => {
                let value =
                    self.lower_value(&value, parent.as_base(), symbol_table, Default::default());

                HirStmt::new(HirStmtKind::Value(value), stmt.span)
            }

            AstStmt::Bind(name, ty, value) => {
                let name = self.lower_name(name);
                let value = self.lower_value(&value, parent.as_base(), symbol_table);
                let ty = ty
                    .as_ref()
                    .map(|ty| self.lower_ty(&ty, parent.as_base(), symbol_table))
                    .unwrap_or_else(|| value.ty.clone());

                // Create a local so we can reference the symbol
                let local = self.create_local(parent.as_base(), &name, &ty);
                let local_symbol = self
                    .context
                    .cast_id::<Symbol>(local)
                    .expect("internal compiler error: local doesn't have a symbol");

                symbol_table.insert(name.name.clone(), local_symbol);

                // Now return a statement
                HirStmt::new(HirStmtKind::Bind(name, local, ty, value), stmt.span)
            }

            AstStmt::Error => return None,
            AstStmt::Semicolon => return None,
        };

        Some(stmt)
    }
}

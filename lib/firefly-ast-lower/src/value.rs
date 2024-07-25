use crate::AstLowerer;
use firefly_ast::value::Value as AstValue;
use firefly_hir::{resolve::SymbolTable, ty::{Ty, TyKind}, value::{LiteralValue, Value as HirValue, ValueKind as HirValueKind}};
use firefly_span::Spanned;
use itertools::Itertools;

impl AstLowerer {
    pub fn lower_value(&mut self, value: &Spanned<AstValue>, symbol_table: &SymbolTable) -> HirValue  {
        let span = value.span;
        let (kind, ty) = match &value.item {
            AstValue::Tuple(items) => {
                let items = items.iter()
                    .map(|item| self.lower_value(item, symbol_table))
                    .collect_vec();

                let types = items.iter()
                    .map(|item| item.ty.clone())
                    .collect_vec();

                let tuple_kind = HirValueKind::Tuple(items);
                let tuple_type = Ty::new(TyKind::Tuple(types), span);

                (tuple_kind, tuple_type)
            },
            AstValue::IntegerLiteral(num) => {
                // Remove the underscores
                let santized_num = num.item.replace("_", "");
                
                let int_kind = HirValueKind::Literal(LiteralValue::Integer(santized_num));
                let int_type = Ty::new(TyKind::Integer, span);

                (int_kind, int_type)
            }

            AstValue::Call(_, _) => todo!(),

            AstValue::Path(path) => match self.resolve_value(path, symbol_table) {
                Some(value) => { return value },
                None => (HirValueKind::Unit, Ty::new(TyKind::Unit, span))
            }
        };

        HirValue::new(kind, ty, span)
    }
}
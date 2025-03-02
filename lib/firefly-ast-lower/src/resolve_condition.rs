use firefly_hir::{func::Callable, resolve::Symbol, HirContext, Id};
use firefly_span::Spanned;
use itertools::Itertools;

pub trait ResolveCondition {
    fn matches(&self, symbol: Id<Symbol>, context: &HirContext) -> bool;

    fn format_for_error(&self) -> String;
}

pub struct CallableResolveCondition {
    pub labels: Vec<Option<Spanned<String>>>,
}

impl ResolveCondition for CallableResolveCondition {
    fn matches(&self, id: Id<Symbol>, context: &HirContext) -> bool {
        let Some(symbol) = context.try_get::<Callable>(id) else {
            return false;
        };

        if symbol.labels.len() != self.labels.len() {
            return false;
        }

        for (label, expected_label) in symbol.labels.iter().zip(self.labels.iter()) {
            match (label, expected_label) {
                (Some(label), Some(expected_label)) if label.name == expected_label.item => {}
                (None, None) => {}
                _ => return false,
            }
        }

        return true;
    }

    fn format_for_error(&self) -> String {
        return format!(
            "func ({})",
            self.labels
                .iter()
                .map(|label| label.as_ref().map_or("", |label| label.item.as_str()))
                .join(", ")
        );
    }
}

pub struct UnconditionalResolveCondition;

impl ResolveCondition for UnconditionalResolveCondition {
    fn matches(&self, _: Id<Symbol>, _: &HirContext) -> bool {
        true
    }

    fn format_for_error(&self) -> String {
        unreachable!()
    }
}

use std::sync::Arc;

use firefly_ast::item::Item;
use firefly_span::SourceFile;

use crate::context::Context;

use super::ParallelPass;

pub struct ParsePass;

impl ParallelPass for ParsePass {
    type Input = Arc<SourceFile>;
    type Output = Vec<Item>;

    fn process(&self, input: Self::Input, context: &mut Context) -> Self::Output {
        let Ok(items) = firefly_parser::parse(input.source_text(), input.start_pos, context.emitter) else {
            return vec![];
        };

        return items;
    }
}
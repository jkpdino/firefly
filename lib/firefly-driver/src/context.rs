use std::sync::Arc;

use firefly_ast_lower::AstLowerer;
use firefly_errors::emitter::Emitter;
use firefly_span::SourceMap;

pub struct Context<'a> {
    pub source_map: &'a Arc<SourceMap>,
    pub emitter: &'a Emitter,
    pub ast_lowerer: &'a mut AstLowerer
}
use std::{path::Path, sync::Arc};

use clap::Parser;
use context::Context;
use firefly_ast_lower::AstLowerer;
use firefly_errors::emitter::{Destination, Emitter};
use firefly_span::{SourceFile, SourceMap};
use pipeline::Pipeline;

pub mod pass;
mod pipeline;
mod context;
mod args;

pub struct Driver {
    source_map: Arc<SourceMap>,
    emitter: Emitter,
    ast_lowerer: AstLowerer,

    print_hir: bool,
}

impl Driver {
    pub fn new() -> Driver {
        let source_map = SourceMap::new();
        let emitter = Emitter::new(Destination::stderr(), &source_map);
        let ast_lowerer = AstLowerer::new();

        Driver { source_map, emitter, ast_lowerer, print_hir: false }
    }

    pub fn parse_args(&mut self) {
        let args = args::Args::parse();

        for input in &args.files {
            self.load_file(&input)
        }

        self.print_hir = args.print_hir;
    }

    pub fn load_file(&self, path: &str) {
        if let Err(err) = self.source_map.load_file(&Path::new(path)) {
            println!("{}", err);
        }
    }

    pub fn run_pipeline<T: Pipeline<Input = Vec<Arc<SourceFile>>>>(&mut self, pipeline: T) {
        let mut context = Context {
            source_map: &self.source_map,
            emitter: &self.emitter,
            ast_lowerer: &mut self.ast_lowerer,
        };

        pipeline.run(self.source_map.files(), &mut context);
    }

    pub fn output(&self) {
        if self.print_hir {
            println!("{}", self.ast_lowerer.context().display())
        }
    }
}
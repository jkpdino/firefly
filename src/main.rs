mod args;

use std::path::Path;

use args::Args;
use clap::Parser;
use firefly_ast_lower::AstLowerer;
use firefly_errors::emitter::Destination;
use itertools::Itertools;

fn main() {
    let args = Args::parse();

    // Load files into the source map
    let source_map = firefly_span::SourceMap::new();
    for file in &args.files {
        source_map.load_file(&Path::new(file)).unwrap();
    }

    let mut emitter = firefly_errors::emitter::Emitter::new(Destination::stderr(), &source_map);

    // Parse files
    let parsed_ast = source_map.files().iter()
        .filter_map(|file| firefly_parser::parse(file.source_text(), file.start_pos, &mut emitter).ok())
        .collect_vec();


    // Lower AST to HIR
    let mut ast_lowerer = AstLowerer::new();

    for ast in &parsed_ast {
        ast_lowerer.link_pass(ast);
    }

    for ast in &parsed_ast {
        ast_lowerer.lower_items(ast);
    }
    if args.print_hir {
        println!("{}", ast_lowerer.context().display());
    }
}

/*
chain:
    Parse (SourceFile -> AST)
    Lower (AST -> HIR)

*/

use std::path::Path;

use firefly_ast_lower::AstLowerer;

fn main() {
    let source_map = firefly_span::SourceMap::new();
    source_map.load_file(&Path::new("tests/All.fly")).unwrap();

    let mut ast_lowerer = AstLowerer::new();

    for file in source_map.files() {
        let items = firefly_parser::parse(file.source_text(), file.start_pos).unwrap();

        ast_lowerer.link_pass(&items);
        ast_lowerer.lower(&items);
    }

    println!("{}", ast_lowerer.context().display());
}

/*
chain:
    Parse (SourceFile -> AST)
    Lower (AST -> HIR)

*/

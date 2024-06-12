use std::path::Path;

mod source;

fn main() {
    let source_map = firefly_span::SourceMap::new();
    source_map.load_file(&Path::new("tests/All.bnk")).unwrap();

    for file in source_map.files() {
        let functions = firefly_parser::parse(file.source_text(), file.start_pos).unwrap();
        
        println!("{functions:#?}");
    }
}

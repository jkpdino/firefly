use firefly_errors::emitter::{Destination, Emitter};
use libtest_mimic::Failed;

use super::Test;

pub fn run_test(test: &Test) -> Result<(), Failed> {
  let source_map = firefly_span::SourceMap::new();

  let Ok(source_file) = source_map.load_file(&test.path) else {
    return Err(Failed::new(format!("Failed to load source file {}", test.path.display())));
  };

  let emitter = Emitter::new(Destination::stderr(), &source_map);

  let Ok(parsed) = firefly_parser::parse(source_file.source_text(), source_file.start_pos, source_map.clone()) else {
    return Err(Failed::new(format!("Failed to parse source file {}", test.path.display())));
  };

  Ok(())
}
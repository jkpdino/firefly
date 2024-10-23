use std::sync::Arc;

use firefly_ast_lower::AstLowerer;
use firefly_errors::emitter::{Destination, Emitter};
use firefly_hir_lower::lower;
use firefly_interpret::ExecutionEngine;
use firefly_mir::MirContext;
use itertools::Itertools;
use libtest_mimic::Failed;

use super::Test;

pub fn run_test(test: &Test) -> Result<(), Failed> {
  // relative to test.path
  let parent_path = test.path.parent().unwrap();

  let mut imported_paths = test.directives.include_files.iter().map(|file| parent_path.join(file)).collect_vec();
  imported_paths.push(test.path.clone());

  let source_map = firefly_span::SourceMap::new();
  let emitter = Arc::new(Emitter::new(Destination::stderr(), &source_map));

  let mut ast_lowerer = AstLowerer::new(emitter.clone());

  let res: Result<Vec<Vec<_>>, _> = imported_paths.iter().map(|path| {
    let Ok(source_file) = source_map.load_file(&path) else {
      return Err(format!("Failed to load source file {}", test.path.display()));
    };

    let Ok(parsed) = firefly_parser::parse(source_file.source_text(), source_file.start_pos, &emitter) else {
      return Err(format!("Failed to parse source file {}", test.path.display()));
    };

    return Ok(parsed)
  }).collect();

  let items = match res {
    Ok(items) => items,
    Err(err) => return Err(err.into()),
  };

  for item in &items {
    ast_lowerer.link_pass(item);
  }

  for item in &items {
    ast_lowerer.lower_item_defs(item);
  }

  for item in &items {
    ast_lowerer.lower_item_codes(item);
  }


  let mut mir_context = MirContext::new();
  lower(ast_lowerer.context_mut(), &mut mir_context);

  let mut execution_engine = ExecutionEngine::new(&mir_context);
  execution_engine.execute();

  Ok(())
}
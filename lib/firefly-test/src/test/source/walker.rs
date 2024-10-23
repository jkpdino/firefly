use walkdir::WalkDir;
use std::path::Path;

use crate::test::{suite::TestSuite, Test};

use super::TestSource;

pub struct TestWalker<'a> {
	pub root: &'a Path
}

impl TestSource for TestWalker<'_> {
	fn gather(&self) -> TestSuite {
		let tests =
      WalkDir::new(self.root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| self.extract_test(entry.path()))
        .enumerate()
        .map(|(index, test)| Test { index, ..test })
        .collect();

    TestSuite::new(tests)
	}
}

impl TestWalker<'_> {
	fn extract_test(&self, path: &Path) -> Option<Test> {
		let ext = path.extension()?;

		if ext != "fly" {
			return None;
		}

		let relative_path = path.strip_prefix(self.root).ok()?;
		let without_ext = relative_path.to_str()?.replace(".fly", "");
		let name = without_ext.replace("/", ".");

		Some(Test::new(0, path.to_path_buf(), name))
	}
}
use std::{fs, io::Read, path::Path};

///
/// A trait for loading source files
///
#[allow(unused)]
pub trait SourceLoader {
	fn exists(&self, path: &Path) -> bool;

	fn read_file(&self, path: &Path) -> std::io::Result<String>;
}

///
/// Reads files from the file system
///
pub struct FileLoader;

impl SourceLoader for FileLoader {
	fn exists(&self, path: &Path) -> bool {
		path.exists()
	}

	fn read_file(&self, path: &Path) -> std::io::Result<String> {
		let mut file = fs::File::open(path)?;

		let mut buf = String::new();
		file.read_to_string(&mut buf)?;
		return Ok(buf);
	}
}

///
/// Loads a test file
///
pub struct TestLoader(pub &'static str);

impl SourceLoader for TestLoader {
	fn exists(&self, _path: &Path) -> bool {
		true
	}

	fn read_file(&self, _path: &Path) -> std::io::Result<String> {
		Ok(self.0.to_owned())
	}
}

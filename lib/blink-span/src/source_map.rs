use std::{
	collections::{BTreeMap, HashMap},
	fmt::{Debug, Display},
	iter,
	path::Path,
	sync::{
		atomic::{AtomicUsize, Ordering},
		Arc, RwLock,
	},
};

use crate::{
	loader::{FileLoader, SourceLoader},
	BytePos, CharPos, FileAndLineInfos, LineInfo, Span,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileName(pub String);

///
/// The line indices of a file. This is used to map a `BytePos` to a line number
///
pub struct FileLines(BTreeMap<BytePos, usize>, BytePos);

pub struct SourceFile {
	/// The name of the input file
	pub file_name: FileName,

	/// The source code of the file
	pub src: Option<Arc<String>>,

	/// The starting position of the file
	pub start_pos: BytePos,

	/// The ending position of the file
	pub end_pos: BytePos,

	/// The lines of the file
	pub lines: FileLines,
}

pub struct SourceMap {
	/// The space already allocated
	mapped_space: AtomicUsize,

	/// A list of the files loaded
	files: RwLock<SourceMapFiles>,

	///The SourceLoader to use
	source_loader: Box<dyn SourceLoader>,
}

pub struct SourceMapFiles {
	files: Vec<Arc<SourceFile>>,

	map: HashMap<FileName, Arc<SourceFile>>,

	positions: BTreeMap<BytePos, Arc<SourceFile>>,
}

impl SourceFile {
	pub fn new(file_name: FileName, source: String, start_pos: BytePos) -> SourceFile {
		let end_pos = BytePos(start_pos.0 + source.bytes().len());
		let lines = FileLines::new(&source, start_pos, end_pos);

		SourceFile {
			file_name,
			src: Some(Arc::new(source)),
			start_pos,
			end_pos,
			lines,
		}
	}

	pub fn source_text(&self) -> &str {
		self.src.as_ref().unwrap()
	}

	pub fn get_line_text(&self, line: usize) -> Option<&str> {
		let start = self
			.lines
			.0
			.iter()
			.find(|(_, l)| **l == line)
			.map(|(pos, _)| pos.0)?
			- self.start_pos.0;

		let end = self
			.lines
			.0
			.iter()
			.find(|(_, l)| **l == line + 1)
			.map(|(pos, _)| pos.0)?
			- self.start_pos.0;

		Some(&self.src.as_ref()?[start..end])
	}
}

impl SourceMap {
	pub fn new() -> Arc<SourceMap> {
		Arc::new(Self {
			mapped_space: AtomicUsize::new(1),
			files: RwLock::new(SourceMapFiles {
				files: Vec::new(),
				map: HashMap::new(),
				positions: BTreeMap::new(),
			}),
			source_loader: Box::new(FileLoader),
		})
	}

	pub fn load_file(&self, path: &Path) -> std::io::Result<Arc<SourceFile>> {
		let file_name = FileName(path.file_name().unwrap().to_string_lossy().into());
		let text = self.source_loader.read_file(path)?;

		self.new_file(file_name, text)
	}

	pub fn allocate_space(&self, size: usize) -> BytePos {
		loop {
			let current = self.mapped_space.load(Ordering::Relaxed);
			let next = current
				.checked_add(size)
				.and_then(|next| next.checked_add(1))
				.expect("internal error: ran out of space");

			if self
				.mapped_space
				.compare_exchange(
					current,
					next,
					Ordering::Relaxed,
					Ordering::Relaxed,
				)
				.is_ok()
			{
				return BytePos(current);
			}
		}
	}

	pub fn new_file(&self, file_name: FileName, text: String) -> std::io::Result<Arc<SourceFile>> {
		let file_size = text.bytes().len();
		let start_pos = self.allocate_space(file_size);

		let source_file = Arc::new(SourceFile::new(
			file_name.clone(),
			text,
			start_pos,
		));

		let mut files = self.files.write().unwrap();

		files.files.push(source_file.clone());
		files.map.insert(file_name, source_file.clone());
		files.positions.insert(start_pos, source_file.clone());

		Ok(source_file)
	}

	pub fn files(&self) -> Vec<Arc<SourceFile>> {
		self.files.read().unwrap().files.clone()
	}

	pub fn line_info(&self, span: Span) -> Option<FileAndLineInfos> {
		let file_lock = self.files.read().ok()?;

		let (file, end_file) = (
			file_lock.get_file_for_pos(span.lo)?,
			file_lock.get_file_for_pos(span.hi)?,
		);

		// Check that the span comes from one file
		if !Arc::ptr_eq(&file, &end_file) {
			return None;
		}

		let mut lines = Vec::new();

		let mut base = span.lo;

		while base < span.hi {
			let (line, column_number) = (
				file.lines.get_line(base),
				file.lines.get_column(base),
			);

			// Get the beginning of the next line and the end of this one
			let line_beginning = file.lines.start_of_last_line(base);
			let next_beginning = file.lines.start_of_next_line(base);
			let end = next_beginning - 1;

			// Get the length of the line
			let end_of_line = CharPos(end.0.min(span.hi.0) - line_beginning.0);

			lines.push(LineInfo {
				line,
				start: column_number,
				end: end_of_line,
			});

			// Set base to the beginning of the next line
			base = next_beginning;
		}

		return Some(FileAndLineInfos { file, lines });
	}
}

impl SourceMapFiles {
	fn get_file_for_pos(&self, pos: BytePos) -> Option<Arc<SourceFile>> {
		self.positions
			.range(..pos)
			.next_back()
			.map(|(_, v)| v.clone())
	}
}

impl FileLines {
	pub fn new(s: &str, base: BytePos, end: BytePos) -> FileLines {
		// maps newlines to their character index
		// and then maps that byte position
		// to the line number
		let newlines = s
			.char_indices()
			.filter(|(_, c)| is_newline(*c))
			.map(|(i, _)| BytePos(base.0 + i))
			.enumerate()
			.map(|(i, n)| (n + 1, i + 2));

		let beginning = iter::once((base, 1));

		FileLines(beginning.chain(newlines).collect(), end)
	}

	///
	/// Returns the line number of a `BytePos`
	///
	pub fn get_line(&self, pos: BytePos) -> usize {
		let line_number = self
			.0
			.range(..=pos)
			.next_back()
			.map(|(_, v)| *v)
			.unwrap_or(usize::MAX);

		return line_number;
	}

	///
	/// Returns the column number of a `BytePos`
	///
	pub fn get_column(&self, pos: BytePos) -> CharPos {
		let line_start = self
			.0
			.range(..pos)
			.next_back()
			.map(|(k, _)| k.0)
			.unwrap_or(usize::MAX);

		return CharPos(pos.0 - line_start);
	}

	///
	/// Returns the position of the last line
	///
	pub fn start_of_last_line(&self, pos: BytePos) -> BytePos {
		self.0
			.range(..pos)
			.next_back()
			.map(|(k, _)| *k)
			.unwrap_or(self.1)
	}

	///
	/// Returns the position of the next line
	///
	pub fn start_of_next_line(&self, pos: BytePos) -> BytePos {
		self.0
			.range((pos + 1)..)
			.next()
			.map(|(k, _)| *k)
			.unwrap_or(self.1)
	}
}

impl Display for FileName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Debug for SourceFile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "SourceFile({})", self.file_name)
	}
}

pub fn is_newline(c: char) -> bool {
	c == '\n'
}

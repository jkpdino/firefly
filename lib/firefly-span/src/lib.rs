#[cfg(test)]
mod test;

mod loader;
mod source_map;
mod span;

use std::{
	fmt::Display,
	ops::{Add, Sub},
	sync::Arc,
};

pub use source_map::{FileName, SourceFile, SourceMap};
pub use span::{Span, Spanned};

///
/// Denotes a byte position within the current file. This position
/// can be within any of the files loaded into the source map.
///
/// Note: This type is not yet optimized for size
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytePos(pub usize);

///
/// Denotes a character position from the start of the line. This may be
/// very different than the byte offset.
///
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharPos(usize);

///
/// A source code location used for pretty printing
///
pub struct LinePos {
	/// The file the location originates from
	pub file: SourceFile,
	/// The (1-based) line number
	pub line: usize,
	/// The (0-based) column number
	pub col: CharPos,
}

///
/// A selection of code within a line
///
pub struct LineInfo {
	pub line: usize,

	pub start: CharPos,
	pub end: CharPos,
}

pub struct FileAndLineInfos {
	pub file: Arc<SourceFile>,
	pub lines: Vec<LineInfo>,
}

impl Add<usize> for BytePos {
	type Output = BytePos;

	fn add(self, rhs: usize) -> Self::Output {
		BytePos(self.0 + rhs)
	}
}

impl Sub<usize> for BytePos {
	type Output = BytePos;

	fn sub(self, rhs: usize) -> Self::Output {
		BytePos(self.0 - rhs)
	}
}

impl Display for CharPos {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl CharPos {
	pub fn n(&self) -> usize {
		self.0
	}
}

impl Default for BytePos {
	fn default() -> Self {
		Self(Default::default())
	}
}

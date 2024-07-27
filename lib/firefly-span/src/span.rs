use crate::BytePos;

///
/// A region of source code, used for error reporting.
///
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
	pub(crate) lo: BytePos,
	pub(crate) hi: BytePos,
}

///
/// Associates a span with a value
///
#[derive(Clone)]
pub struct Spanned<T> {
	pub item: T,
	pub span: Span,
}

impl Span {
	pub const DUMMY: Span = Span {
		lo: BytePos(0),
		hi: BytePos(0),
	};

	pub fn new(lo: BytePos, hi: BytePos) -> Span {
		Span { lo, hi }
	}

	pub fn intersects(self, other: Span) -> bool {
		self.lo < other.hi && other.lo < self.hi
	}

	pub fn contains(self, other: Self) -> bool {
		other.lo >= self.lo && other.hi <= self.hi
	}

	/// Returns a span enclosing `self` and `end`
	///
	/// Note that this is an ordered operation, that is, `start.to(end)` and
	/// `end.to(start)` both return the same thing
	///
	///
	/// ```text
	///     ----             ---
	///     self lorem ipsum end
	///     ^^^^^^^^^^^^^^^^^^^^
	/// ```
	pub fn to(self, end: Span) -> Span {
		use std::cmp;

		Span::new(
			cmp::min(self.lo, end.lo),
			cmp::max(self.hi, end.hi),
		)
	}

	/// Returns a span between the end of `self` and the beginning of `end`
	///
	/// ```text
	///     ----             ---
	///     self lorem ipsum end
	///         ^^^^^^^^^^^^^
	/// ```
	pub fn between(self, end: Span) -> Span {
		Span::new(self.hi, end.lo)
	}
}

impl<T> Spanned<T> {
	pub fn new(item: T, span: Span) -> Spanned<T> {
		Spanned { item, span }
	}
}

impl Default for Span {
	fn default() -> Self {
		Span::DUMMY
	}
}

impl std::fmt::Debug for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.lo.0)
	}
}

impl<T> std::fmt::Debug for Spanned<T>
where
	T: std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.item.fmt(f)
	}
}
use std::borrow::Cow;

type FluentId = Cow<'static, str>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticMessage {
	///
	/// A non-translatable message
	///
	Str(String),

	///
	/// A message that has already been translated
	///
	Translated(String),

	///
	/// A message that can be translated using Fluent
	///
	Fluent(FluentId),
}


impl DiagnosticMessage {
	pub fn from_str(s: &str) -> Self {
		Self::Str(s.to_string())
	}
}
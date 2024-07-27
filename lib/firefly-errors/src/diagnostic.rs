use std::{fmt::Display, panic::Location};

use firefly_error_messages::DiagnosticMessage;
use firefly_span::Span;
use termcolor::Color;

pub struct DiagnosticId {
	error: String,
}

pub enum Level {
	Hint,
	Warning,
	Info,
	Error,
	Fatal,
}

pub struct Diagnostic {
	///
	/// The seriousness of this diagnostic
	///
	pub level: Level,

	///
	///
	///
	pub message: DiagnosticMessage,

	///
	/// The error code of this diagnostic
	///
	pub code: Option<DiagnosticId>,

	///
	/// The location that threw this diagnostic
	/// Very useful for debugging
	///
	pub source_location: DiagnosticLocation,

	///
	/// todo: Very temporary
	///
	pub source: Vec<Span>,
}

pub struct DiagnosticLocation {
	pub file: &'static str,
	pub line: u32,
	pub col: u32,
}

impl DiagnosticLocation {
	#[track_caller]
	pub fn caller() -> Self {
		let location = Location::caller();
		DiagnosticLocation {
			file: location.file(),
			line: location.line(),
			col: location.column(),
		}
	}

	fn new(location: &'static Location) -> Self {
		DiagnosticLocation {
			file: location.file(),
			line: location.line(),
			col: location.column(),
		}
	}
}

impl Diagnostic {
	#[track_caller]
	pub fn new(level: Level, message: DiagnosticMessage) -> Self {
		Diagnostic {
			level,
			message,
			code: None,
			source_location: DiagnosticLocation::new(Location::caller()),
			source: Vec::new(),
		}
	}

	pub fn with_error_code(mut self, code: DiagnosticId) -> Self {
		self.code = Some(code);
		self
	}

	pub fn with_source(mut self, source: Span) -> Self {
		self.source.push(source);
		self
	}
}

impl DiagnosticId {
	pub fn new(code: impl ToString) -> DiagnosticId {
		DiagnosticId {
			error: code.to_string(),
		}
	}
}

impl Display for DiagnosticLocation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}:{}:{}",
			self.file, self.line, self.col
		)
	}
}

impl Level {
	pub fn color(&self) -> Color {
		match self {
			Level::Hint => Color::Green,
			Level::Warning => Color::Yellow,
			Level::Info => Color::Blue,
			Level::Error => Color::Red,
			Level::Fatal => Color::Magenta,
		}
	}
}

impl Display for Level {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Level::Hint => write!(f, "hint"),
			Level::Warning => write!(f, "warning"),
			Level::Info => write!(f, "info"),
			Level::Error => write!(f, "error"),
			Level::Fatal => write!(f, "fatal"),
		}
	}
}

impl Display for DiagnosticId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.error)
	}
}

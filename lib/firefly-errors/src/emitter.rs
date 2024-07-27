use std::sync::Arc;

use firefly_span::{SourceMap, Span};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};

use crate::diagnostic::Diagnostic;
use std::io::Write;

pub enum Destination {
	Terminal(StandardStream),
}

pub struct Emitter {
	destination: Destination,

	source_map: Option<Arc<SourceMap>>,
}

impl Emitter {
	pub fn new(destination: Destination, source_map: &Arc<SourceMap>) -> Emitter {
		Emitter {
			destination,
			source_map: Some(source_map.clone()),
		}
	}

	fn set_color(&self, stream: &mut StandardStreamLock, color: Option<Color>, is_bold: bool) {
		let mut spec = ColorSpec::new();
		spec.set_fg(color).set_bold(is_bold);

		stream.set_color(&spec).unwrap();
	}

	pub fn emit(&self, diagnostic: Diagnostic) -> std::io::Result<()> {
		// right now,
		let mut output = match &self.destination {
			Destination::Terminal(stream) => stream.lock(),
		};

		self.set_color(
			&mut output,
			Some(diagnostic.level.color()),
			true,
		);
		write!(&mut output, "{}", diagnostic.level)?;
		if let Some(error_code) = diagnostic.code {
			write!(&mut output, "[{}]", error_code)?;
		}

		self.set_color(&mut output, None, true);
		let text = match diagnostic.message {
			firefly_error_messages::DiagnosticMessage::Str(s) => s,
			firefly_error_messages::DiagnosticMessage::Translated(s) => s,
			firefly_error_messages::DiagnosticMessage::Fluent(id) => id.into_owned(),
		};
		writeln!(output, ": {text}")?;

		for span in diagnostic.source {
			self.write_span(&mut output, span)?;
		}

		// As of right now, get the color for the level
		// Output [fatal], [error], [warning], [info], or [hint]
		// Output the message
		// And output the location

		Ok(())
	}

	fn write_span(&self, stream: &mut StandardStreamLock, span: Span) -> std::io::Result<()> {
		let Some(source_map) = &self.source_map else {
			return Ok(());
		};

		let Some(file_and_line_infos) = source_map.line_info(span) else {
			return Ok(());
		};

		self.set_color(stream, Some(Color::Blue), true);
		write!(stream, "  --> ")?;

		let line_desc = if let Some(first_line) = file_and_line_infos.lines.first() {
			format!(
				":{}:{}",
				first_line.line, first_line.start
			)
		} else {
			"".to_string()
		};

		self.set_color(stream, None, false);
		writeln!(
			stream,
			"{}{line_desc}",
			file_and_line_infos.file.file_name
		)?;

		if !file_and_line_infos.lines.is_empty() {
			self.set_color(stream, Some(Color::Blue), true);
			writeln!(stream, "   |")?;
			for line in &file_and_line_infos.lines {
				self.set_color(stream, Some(Color::Blue), true);
				write!(stream, "{:2} |", line.line)?;

				self.set_color(stream, None, false);
				let code_line = file_and_line_infos
					.file
					.get_line_text(line.line)
					.map(|s| s.trim_end())
					.unwrap_or("");

				const TAB_WIDTH: usize = 4;

				let number_of_tabs_before_start = code_line
					.chars()
					.take(line.start.n())
					.filter(|c| *c == '\t')
					.count();
				let number_of_tabs_before_end = code_line
					.chars()
					.take(line.end.n())
					.filter(|c| *c == '\t')
					.count();

				let length_of_selection = line.end.n() - line.start.n()
					+ (number_of_tabs_before_end - number_of_tabs_before_start) * (TAB_WIDTH - 1);
				let selector = "^".repeat(length_of_selection);

				let length_before_selection =
					line.start.n() + number_of_tabs_before_start * (TAB_WIDTH - 1);
				let padding = " ".repeat(length_before_selection);

				let code_line_without_tabs = code_line.replace("\t", &" ".repeat(TAB_WIDTH));
				writeln!(stream, " {code_line_without_tabs}")?;

				self.set_color(stream, Some(Color::Blue), true);
				writeln!(stream, "   | {padding}{selector}")?;
				self.set_color(stream, None, false);
			}
		}

		Ok(())
	}
}

impl Destination {
	pub fn stderr() -> Self {
		Destination::Terminal(StandardStream::stderr(
			ColorChoice::Auto,
		))
	}
}

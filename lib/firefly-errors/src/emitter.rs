use std::{collections::HashMap, sync::{atomic::AtomicBool, Arc}};

use firefly_error_messages::DiagnosticMessage;
use firefly_span::{FileAndLineInfos, SourceFile, SourceMap, Span};
use itertools::Itertools;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};

use crate::{annotation::{Annotation, AnnotationKind}, diagnostic::{Diagnostic, Level}};
use std::io::Write;

pub enum Destination {
	Terminal(StandardStream),
}

pub struct Emitter {
	destination: Destination,

	triggered: AtomicBool,

	source_map: Option<Arc<SourceMap>>,
}

const TAB_WIDTH: usize = 4;

impl Emitter {
	pub fn new(destination: Destination, source_map: &Arc<SourceMap>) -> Emitter {
		Emitter {
			destination,
			triggered: AtomicBool::new(false),
			source_map: Some(source_map.clone()),
		}
	}

	fn set_color(&self, stream: &mut StandardStreamLock, color: Option<Color>, is_bold: bool) {
		let mut spec = ColorSpec::new();
		spec.set_fg(color).set_bold(is_bold);

		stream.set_color(&spec).unwrap();
	}

	pub fn has_triggered(&self) -> bool {
		self.triggered.load(std::sync::atomic::Ordering::Relaxed)
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
		let text = self.stringify_message(&diagnostic.message);
		writeln!(output, ": {text}")?;

		self.write_annotations(&mut output, &diagnostic.annotations, diagnostic.level)?;

		if let Level::Error = diagnostic.level {
			self.triggered.store(true, std::sync::atomic::Ordering::Relaxed);
		}

		Ok(())
	}

	fn write_annotations(&self, stream: &mut StandardStreamLock, annotations: &[Annotation], level: Level) -> std::io::Result<()> {
		// Sort the annotations by what file they are contained in
		let Some(source_map) = &self.source_map else {
			return Ok(());
		};

		let mut annotations_by_file: HashMap<firefly_span::BytePos, Vec<_>> = HashMap::new();

		for annotation in annotations {
			let Some(line_info) = source_map.line_info(annotation.loc) else {
				continue;
			};

			let annotations_for_file = annotations_by_file.entry(line_info.file.start_pos).or_default();

			annotations_for_file.push((
				annotation,
				line_info
			));
		}

		for (_, file) in annotations_by_file {
			self.write_file_annotations(stream, file, level)?;
		}

		Ok(())
	}

	fn write_file_annotations(&self, stream: &mut StandardStreamLock, annotations: Vec<(&Annotation, FileAndLineInfos)>, level: Level) -> std::io::Result<()> {
		// Get a list of lines to print, in order
		let Some(source_map) = &self.source_map else {
			return Ok(());
		};

		let mut all_lines_to_print = Vec::new();
		let file = annotations[0].1.file.clone();

		for (_, lines) in &annotations {
			for line in &lines.lines {
				all_lines_to_print.push(line);
			}
		}

		all_lines_to_print.sort_by_key(|line_info| line_info.line);
		all_lines_to_print.dedup_by_key(|line_info| line_info.line);

		let max_line_num_width = all_lines_to_print.iter().map(|line| line.line.to_string().len()).max().unwrap_or(0);

		// Write out each line
		// If lines are consecutive, write them back to back
		// If lines are less than 3 away, write the intermediate lines
		// If lines are less than 10, put an ellipsis
		// Otherwise, start a new section
		const FILL_IN_GAP: usize = 3;
		const ELLIPSIS_GAP: usize = 10;

		let mut last_line_num = 0;
		let mut is_first = true;

		for line in all_lines_to_print {
			let line_num = line.line;

			let is_new_section = is_first || (line_num - last_line_num > ELLIPSIS_GAP);
			let fill_in_lines = line_num - last_line_num < FILL_IN_GAP;

			let annotations_on_line = annotations.iter().filter(|anno| {
				let Some(anno_line) = source_map.line_info(anno.0.loc) else {
					return false;
				};

				anno_line.lines.iter().any(|anno_line| anno_line.line == line.line)
			}).map(|anno| anno.0).collect_vec();

			if is_new_section {
				let span = annotations_on_line.first()
											  .map(|anno| anno.loc)
											  .unwrap_or_default();

				self.write_span_header(stream, span)?;
				self.write_line_padding(stream, max_line_num_width)?;
			}

			if fill_in_lines {
				for i in (last_line_num + 1)..line_num {
					self.write_line(stream, &file, i, max_line_num_width)?;
				}
			}
			else {
				self.write_line_ellipsis(stream)?;
			}

			// write the line and annotations
			self.write_line(stream, &file, line.line, max_line_num_width)?;
			for anno in annotations_on_line {
				self.write_annotation(stream, anno, line_num, max_line_num_width, level)?;
			}

			is_first = false;
			last_line_num = line_num;
		}

		self.write_line_padding(stream, max_line_num_width)?;

		Ok(())
	}

	fn write_span_header(&self, stream: &mut StandardStreamLock, span: Span) -> std::io::Result<()> {
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
		)
	}

	fn write_line_ellipsis(&self, stream: &mut StandardStreamLock) -> std::io::Result<()> {
		self.set_color(stream, Some(Color::Blue), true);
		writeln!(stream, "...")?;
		self.set_color(stream, None, false);

		Ok(())
	}

	fn write_line_padding(&self, stream: &mut StandardStreamLock, max_line_num_width: usize) -> std::io::Result<()> {
		self.set_color(stream, Some(Color::Blue), true);
		writeln!(stream, "{} |", " ".repeat(max_line_num_width))?;
		self.set_color(stream, None, false);

		Ok(())
	}

	fn write_line(&self, stream: &mut StandardStreamLock, file: &Arc<SourceFile>, line: usize, max_line_num_width: usize) -> std::io::Result<()> {
		self.set_color(stream, Some(Color::Blue), true);
		write!(stream, "{:>max_line_num_width$} |", line)?;
		self.set_color(stream, None, false);

		let code_line = file
			.get_line_text(line)
			.unwrap_or("")
			.trim_end()
			.replace("\t", &" ".repeat(TAB_WIDTH));

		writeln!(stream, " {code_line}")
	}

	fn write_annotation(
		&self,
		stream: &mut StandardStreamLock,
		annotation: &Annotation,
		line: usize,
		max_line_num_width: usize,
		level: Level,
	) -> std::io::Result<()>
	{
		let Some(source_map) = &self.source_map else {
			return Ok(());
		};

		let Some(file_and_line_infos) = source_map.line_info(annotation.loc) else {
			return Ok(());
		};

		let line_text = file_and_line_infos.file.get_line_text(line).unwrap_or("");

		let is_last_line = file_and_line_infos.lines.last().map(|last_line| last_line.line == line).unwrap_or(false);
		let Some(line) = file_and_line_infos.lines.iter().find(|line_info| line_info.line == line) else {
			return Ok(())
		};

		let num_of_tabs_before_start = line_text
			.chars()
			.take(line.start.n())
			.filter(|c| *c == '\t')
			.count();
		let num_of_tabs_before_end = line_text
			.chars()
			.take(line.end.n())
			.filter(|c| *c == '\t')
			.count() - num_of_tabs_before_start;
		let num_of_tabs_in_selection = num_of_tabs_before_end - num_of_tabs_before_start;

		let extra_padding_from_tabs = num_of_tabs_before_start * (TAB_WIDTH - 1);
		let extra_length_from_tabs = num_of_tabs_in_selection * (TAB_WIDTH - 1);

		let padding = line.start.n() + extra_padding_from_tabs;
		let length = (line.end.n() - line.start.n()) + extra_length_from_tabs;

		let default_color = level.color();
		let (color, c) = match &annotation.kind {
			AnnotationKind::Suggestion => (Color::Blue, ' '),
			AnnotationKind::Message => (default_color, '^'),
			AnnotationKind::None => (Color::White, ' ')
		};

		let message =
			if is_last_line { self.stringify_message(&annotation.message) }
			else { " ".to_string() };

		self.write_annotation_inner(
			stream,
			padding,
			length,
			color,
			c,
			&message,
			max_line_num_width
		)?;

		Ok(())
	}

	fn write_annotation_inner(
		&self,
		stream: &mut StandardStreamLock,
		padding: usize,
		length: usize,
		color: Color,
		c: char,
		message: &str,
		max_line_num_width: usize) -> std::io::Result<()>
	{
		self.set_color(stream, Some(Color::Blue), true);
		write!(stream, "{} | ", " ".repeat(max_line_num_width))?;

		self.set_color(stream, Some(color), true);
		writeln!(stream, "{}{} {message}", " ".repeat(padding), c.to_string().repeat(length))?;

		self.set_color(stream, None, false);
		Ok(())
	}

	fn stringify_message(&self, message: &DiagnosticMessage) -> String {
		match message {
			DiagnosticMessage::Str(s) => s.clone(),
			DiagnosticMessage::Translated(s) => s.clone(),
			DiagnosticMessage::Fluent(id) => id.clone().into_owned(),
		}
	}
}

impl Destination {
	pub fn stderr() -> Self {
		Destination::Terminal(StandardStream::stderr(
			ColorChoice::Auto,
		))
	}
}

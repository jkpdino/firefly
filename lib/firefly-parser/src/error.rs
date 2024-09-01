use std::fmt::Display;

use firefly_error_messages::DiagnosticMessage;
use firefly_errors::{diagnostic::{Diagnostic, Level}, emitter::Emitter};
use firefly_span::{BytePos, Span};
use itertools::Itertools;
use lalrpop_util::ParseError;

use crate::lexer::Token;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    NewlineInString,
    UnclosedString,
    UnclosedEscape,

    #[default]
    Other,
}

pub enum Expecting {
    Item,
	Value,
	Type,
	Path,
	Stmt
}

pub struct ParserErrorEnv<'a>(pub(crate) &'a Emitter);


impl ParserErrorEnv<'_> {
	pub fn emit(
		&self,
		error: ParseError<BytePos, Token, (LexerError, Span)>,
        expecting: Option<Expecting>
	) {
		let expected_message = |expected_tokens: &[String]| match expecting {
			Some(expecting) => format!("{expecting}"),
			None => {
				let mut trimmed_expected = expected_tokens.iter().map(|t| t.trim_matches('\"'));

				format!("{}", trimmed_expected.join(", "))
			}
		};

		let diagnostic = match error {
			ParseError::InvalidToken { location } => {
				let span = Span::new(location, location);

				let message = DiagnosticMessage::Str("invalid token".into());
				Diagnostic::new(Level::Error, message).with_highlight(span)
			}
			ParseError::UnrecognizedEof { location, expected } => {
				let span = Span::new(location, location);

				let message = DiagnosticMessage::Str(format!(
					"expected {}, found <EOF>",
					expected_message(&expected)
				));
				Diagnostic::new(Level::Error, message).with_highlight(span)
			}
			ParseError::UnrecognizedToken {
				token: (left, token, right),
				expected,
			} => {
				let span = Span::new(left, right);

				let message = DiagnosticMessage::Str(format!(
					"expected {}, found {}",
					expected_message(&expected),
					self.stringify_token(&token)
				));
				Diagnostic::new(Level::Error, message).with_highlight(span)
			}
			ParseError::ExtraToken { token } => {
				let span = Span::new(token.0, token.2);

				let message = DiagnosticMessage::Str(format!(
					"unexpected {:?}",
					self.stringify_token(&token.1)
				));
				Diagnostic::new(Level::Error, message).with_highlight(span)
			}
			ParseError::User { error: (LexerError::NewlineInString, span) } => {
				let message = DiagnosticMessage::Str(format!(
                    "found newline in string literal"
                ));
				Diagnostic::new(Level::Error, message)
					.with_highlight(span)
			}
            ParseError::User { error: (LexerError::UnclosedString, span) } => {
                let message = DiagnosticMessage::Str(format!(
                    "unclosed string literal"
                ));
                Diagnostic::new(Level::Error, message)
					.with_highlight(span)
            }
            ParseError::User { error: (LexerError::UnclosedEscape, span) } => {
                let message = DiagnosticMessage::Str(format!(
                    "unclosed escape sequence"
                ));
                Diagnostic::new(Level::Error, message)
					.with_highlight(span)
            }
            ParseError::User { error: (LexerError::Other, span) } => {
                let message = DiagnosticMessage::Str(format!(
                    "unexpected error"
                ));
                Diagnostic::new(Level::Error, message)
					.with_highlight(span)
            }
		};

		self.emitter().emit(diagnostic).unwrap();
	}

	fn emitter(&self) -> &Emitter {
		&self.0
	}

	fn stringify_token(&self, t: &Token<'_>) -> String {
		match t {
            Token::Ident(ident) => format!("identifier `{}`", ident),

            Token::IntegerLiteral(int) => format!("integer literal `{}`", int),
			Token::FloatLiteral(float) => format!("float literal `{}`", float),
            Token::StringLiteral(string) => format!("string literal `{}`", string),
            Token::LongStringLiteral(string) => format!("string literal `{}`", string),

            // Keywords
            Token::PublicKw => "keyword `public`".to_string(),
            Token::InternalKw => "keyword `internal`".to_string(),
            Token::FilePrivateKw => "keyword `fileprivate`".to_string(),
            Token::PrivateKw => "keyword `private`".to_string(),

			Token::StaticKw => "keyword `static`".to_string(),

            Token::ModuleKw => "keyword `module`".to_string(),
            Token::ImportKw => "keyword `import`".to_string(),

			Token::AsKw => "keyword `as`".to_string(),

			Token::VarKw => "keyword `var`".to_string(),
            Token::FuncKw => "keyword `func`".to_string(),
            Token::StructKw => "keyword `struct`".to_string(),

            Token::ReturnKw => "keyword `return`".to_string(),
			Token::BreakKw => "keyword `break`".to_string(),
			Token::ContinueKw => "keyword `continue`".to_string(),

			Token::IfKw => "keyword `if`".to_string(),
			Token::ElseKw => "keyword `else`".to_string(),
			Token::WhileKw => "keyword `while`".to_string(),

            // Symbols
            Token::OpenParen => "symbol `(`".to_string(),
            Token::CloseParen => "symbol `)`".to_string(),
            Token::OpenBrace => "symbol `{`".to_string(),
            Token::CloseBrace => "symbol `}`".to_string(),

            Token::Period => "symbol `.`".to_string(),
            Token::Comma => "symbol `,`".to_string(),
            Token::Semicolon => "symbol `;`".to_string(),
            Token::Colon => "symbol `:`".to_string(),
            Token::Arrow => "symbol `->`".to_string(),

            Token::Equals => "symbol `=`".to_string(),

            Token::Whitespace => "whitespace".to_string(),

            Token::Comment => "comment".to_string(),
		}
	}
}

impl Display for Expecting {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expecting::Item => write!(f, "item"),
			Expecting::Type => write!(f, "type"),
			Expecting::Value => write!(f, "value"),
			Expecting::Path => write!(f, "path"),
			Expecting::Stmt => write!(f, "statement"),
		}
	}
}

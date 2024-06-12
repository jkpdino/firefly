use std::ops::Range;

use firefly_span::BytePos;
use logos::Logos;

use crate::error::LexerError;

#[derive(Logos, Debug, Clone)]
pub enum Token<'a> {
	#[regex("[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
	Ident(&'a str),

	// Literals
	#[regex("([0-9][0-9_]*)|(0[xX][0-9a-fA-F_]+)|(0[oO][0-7_]+)|(0[bB][01_]+)")]
	IntegerLiteral(&'a str),

	// Keywords
	#[token("public")] PublicKw,
	#[token("internal")] InternalKw,
	#[token("fileprivate")] FilePrivateKw,
	#[token("private")] PrivateKw,

	#[token("module")] ModuleKw,
	#[token("import")] ImportKw,

	#[token("func")] FuncKw,
	#[token("struct")] StructKw,
	#[token("init")] InitKw,

	#[token("var")] VarKw,

	#[token("static")] StaticKw,
	#[token("mutating")] MutatingKw,
	#[token("owning")] Owning,

	#[token("return")] ReturnKw,

	// Symbols
	#[token("(")] OpenParen,
	#[token(")")] CloseParen,
	#[token("{")] OpenBrace,
	#[token("}")] CloseBrace,

	#[token(".")] Period,
	#[token(",")] Comma,
	#[token(";")] Semicolon,
	#[token(":")] Colon,
	#[token("->")] Arrow,

	#[token("=")]
	Equals,

	#[regex("[\n\r\t ]", logos::skip)]
	Whitespace,

	#[regex("//[^\n]*", logos::skip)]
	Comment,
}

impl<'a> Token<'a> {
	pub fn to_lalr_triple(
		(t, r): (Result<Token<'a>, ()>, Range<usize>),
		base: BytePos,
	) -> Result<(BytePos, Token, BytePos), LexerError> {
		let Ok(t) = t else {
			panic!()
			//return Err(LexerError::Err);
		};
		Ok((base + r.start, t, base + r.end))
	}
}

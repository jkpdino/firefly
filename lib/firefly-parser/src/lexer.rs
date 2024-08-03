use std::ops::Range;

use firefly_span::{BytePos, Span};
use logos::Logos;

use crate::error::LexerError;

#[derive(Logos, Debug, Clone)]
#[logos(error = LexerError)]
pub enum Token<'a> {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    Ident(&'a str),

    // Literals
    #[regex("([0-9][0-9_]*)")]
    #[regex("(0[xX][0-9a-fA-F_]+)")]
    #[regex("(0[oO][0-7_]+)")]
    #[regex("(0[bB][01_]+)")]
    IntegerLiteral(&'a str),

    #[regex(r#"raw""#, |lex| lex_string(lex, true))]
    #[regex(r#"""#, |lex| lex_string(lex, false))]
    StringLiteral(&'a str),

    #[regex(r#"raw""""#, |lex| lex_long_string(lex, true))]
    #[regex(r#"""""#, |lex| lex_long_string(lex, false))]
    LongStringLiteral(&'a str),

    // Keywords
    #[token("public")] PublicKw,
    #[token("internal")] InternalKw,
    #[token("fileprivate")] FilePrivateKw,
    #[token("private")] PrivateKw,

    #[token("module")] ModuleKw,
    #[token("import")] ImportKw,

    #[token("as")] AsKw,

    #[token("var")] VarKw,
    #[token("func")] FuncKw,
    #[token("struct")] StructKw,

    #[token("static")] StaticKw,

    #[token("return")] ReturnKw,
    #[token("if")] IfKw,
    #[token("else")] ElseKw,
    #[token("while")] WhileKw,

    // Symbols
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,

    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,

    #[token("=")]
    Equals,

    #[regex("[\n\r\t ]", logos::skip)]
    Whitespace,

    #[regex("//[^\n]*", logos::skip)]
    Comment,
}

fn lex_string<'a>(lexer: &mut logos::Lexer<'a, Token<'a>>, raw: bool) -> Result<&'a str, LexerError> {
    let mut remaining = lexer.remainder().chars();

    while let Some(next) = remaining.next() {
        lexer.bump(next.len_utf8());

        match next {
            '"' => {
                return Ok(lexer.slice())
            }
            '\n' => return Err(LexerError::NewlineInString),
            '\\' if !raw => match remaining.next() {
                Some('x') => {
                    lexer.bump('x'.len_utf8());

                    let num_digits = lexer.remainder()
                        .chars()
                        .take(4)
                        .take_while(char::is_ascii_hexdigit)
                        .count();

                    for _ in 0..num_digits {
                        remaining.next();
                    }

                    lexer.bump(num_digits);
                }
                Some(c) => {
                    lexer.bump(c.len_utf8())
                }
                None => return Err(LexerError::UnclosedEscape),
            },
            _ => {}
        }
    }

    return Err(LexerError::UnclosedString)
}

fn lex_long_string<'a>(lexer: &mut logos::Lexer<'a, Token<'a>>, raw: bool) -> Result<&'a str, LexerError> {
    let num_of_quotes = lexer.remainder().chars().take_while(|c| *c == '"').count() + 3;
    lexer.bump('"'.len_utf8() * num_of_quotes);

    let mut remaining = lexer.remainder().chars();
    let mut seen_quotes = 0;

    while let Some(next) = remaining.next() {
        lexer.bump(next.len_utf8());

        match next {
            '"' => {
                seen_quotes += 1;
                if num_of_quotes == seen_quotes {
                    return Ok(lexer.slice())
                }
                continue;
            }
            '\\' if !raw => match remaining.next() {
                Some('x') => {
                    lexer.bump('x'.len_utf8());

                    let num_digits = lexer.remainder()
                        .chars()
                        .take(4)
                        .take_while(char::is_ascii_hexdigit)
                        .count();

                        for _ in 0..num_digits {
                            remaining.next();
                        }

                    lexer.bump(num_digits);
                }
                Some(c) => {
                    lexer.bump(c.len_utf8())
                }
                None => return Err(LexerError::UnclosedEscape)
            },
            _ => {}
        }

        seen_quotes = 0;
    }

    return Err(LexerError::UnclosedString)
}


impl<'a> Token<'a> {
    pub fn to_lalr_triple(
        (t, r): (Result<Token<'a>, LexerError>, Range<usize>),
        base: BytePos,
    ) -> Result<(BytePos, Token, BytePos), (LexerError, Span)> {
        let t = t.map_err(|e| (e, Span::new(base + r.start, base + r.end)))?;
        Ok((base + r.start, t, base + r.end))
    }
}

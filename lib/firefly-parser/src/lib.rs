use firefly_ast::item::Item;
use firefly_span::BytePos;
use lalrpop_util::lalrpop_mod;
use logos::Logos;

use crate::lexer::Token;

mod lexer;
mod error;
lalrpop_mod!(parser);

pub fn parse(source: &str, base: BytePos) -> Result<Vec<Item>, ()> {
	let tokens = Token::lexer(source)
		.spanned()
		.map(|tok| Token::to_lalr_triple(tok, base));

	match parser::TopParser::new().parse(tokens) {
		Ok(items) => Ok(items),
		Err(e) => {
			println!("{e:?}");
			Err(())
		}
	}
}

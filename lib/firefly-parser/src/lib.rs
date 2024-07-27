use error::ParserErrorEnv;
use firefly_ast::item::Item;
use firefly_errors::emitter::Emitter;
use firefly_span::BytePos;
use lalrpop_util::lalrpop_mod;
use logos::Logos;

use crate::lexer::Token;

mod error;
mod lexer;
lalrpop_mod!(parser);

pub fn parse(source: &str, base: BytePos, emitter: &Emitter) -> Result<Vec<Item>, ()> {
    let tokens = Token::lexer(source)
        .spanned()
        .map(|tok| Token::to_lalr_triple(tok, base));

    let mut error_env = ParserErrorEnv(emitter);

    match parser::TopParser::new().parse(&mut error_env, tokens) {
        Ok(items) => Ok(items),
        Err(e) => {
            error_env.emit(e, None);
            Err(())
        }
    }
}

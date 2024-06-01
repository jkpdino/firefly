use blink_span::Spanned;

use crate::{Name, Path};

#[derive(Debug)]
pub enum Value {
    Tuple(Vec<Spanned<Value>>),
    IntegerLiteral(Name),
    Path(Path),
    Call(Box<Spanned<Value>>, Vec<Spanned<Value>>),
}
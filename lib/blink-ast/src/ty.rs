use blink_span::Spanned;

use crate::Path;

#[derive(Debug)]
pub enum Ty {
    Tuple(Vec<Spanned<Ty>>),
    Path(Path),
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    NewlineInString,
    UnclosedString,
    UnclosedEscape,

    #[default]
    Other,
}
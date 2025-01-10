use firefly_span::{Span, Spanned};

pub mod func;
pub mod import;
pub mod item;
pub mod module;
pub mod operator;
pub mod stmt;
pub mod struct_def;
pub mod ty;
pub mod value;

pub type Name = Spanned<String>;

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Internal,
    FilePrivate,
    Private,
}

#[derive(Debug, Clone)]
pub struct PathSegment {
    pub name: Name,
}

impl PathSegment {
    pub fn new(name: Name) -> Self {
        Self { name }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

impl Path {
    pub fn new(segments: Vec<PathSegment>, span: Span) -> Self {
        Self { segments, span }
    }
}

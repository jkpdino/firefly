use firefly_span::{Span, Spanned};

pub mod ty;
pub mod stmt;
pub mod func;
pub mod item;
pub mod value;
pub mod module;
pub mod import;
pub mod struct_def;

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
    pub name: Name
}

impl PathSegment {
    pub fn new(name: Name) -> Self {
        Self {
            name
        }
    }
}

#[derive(Debug, Default)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

impl Path {
    pub fn new(segments: Vec<PathSegment>, span: Span) -> Self {
        Self {
            segments,
            span,
        }
    }
}
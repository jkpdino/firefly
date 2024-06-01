use blink_span::{Span, Spanned};

pub mod ty;
pub mod stmt;
pub mod func;
pub mod value;

pub type Name = Spanned<String>;

#[derive(Debug)]
pub enum Visibility {
    Public,
    Internal,
    FilePrivate,
    Private,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
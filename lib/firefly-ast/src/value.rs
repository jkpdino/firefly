use firefly_span::Spanned;

use crate::{operator::{InfixOperator, PrefixOperator}, stmt::CodeBlock, Name, Path, PathSegment};

#[derive(Debug)]
pub enum Value {
    Tuple(Vec<Spanned<Value>>),
    IntegerLiteral(Name),
    FloatLiteral(Name),
    StringLiteral(Name),
    Path(Path),
    Call(Box<Spanned<Value>>, Vec<Spanned<Value>>),
    Return(Option<Box<Spanned<Value>>>),
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    Break(Option<Name>),
    Continue(Option<Name>),
    Assign(Box<Spanned<Value>>, Box<Spanned<Value>>),
    Member(Box<Spanned<Value>>, PathSegment),
    TupleMember(Box<Spanned<Value>>, Name),
    Prefix(PrefixOperator, Box<Spanned<Value>>),
    Infix(Box<Spanned<Value>>, InfixOperator, Box<Spanned<Value>>),
    Error,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Spanned<Value>,
    pub positive: CodeBlock,
    pub negative: Option<ElseStatement>
}

#[derive(Debug)]
pub enum ElseStatement {
    Else(CodeBlock),
    ElseIf(Box<IfStatement>)
}

#[derive(Debug)]
pub struct WhileStatement {
    pub label: Option<Name>,
    pub condition: Spanned<Value>,
    pub body: CodeBlock,
}

impl Value {
    pub fn member(parent: Box<Spanned<Value>>, member: PathSegment) -> Value {
        match &parent.item {
            Value::Path(base) => {
                let new_span = base.span.to(member.name.span);
                let mut new_segments = base.segments.clone();
                new_segments.push(member);

                let new_base = Path::new(new_segments, new_span);

                Value::Path(new_base)
            }
            _ => Value::Member(parent, member)
        }
    }
}
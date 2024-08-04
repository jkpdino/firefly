use firefly_hir::{stmt::CodeBlock, Id, Name};

pub struct LoopLabel {
    pub label: Option<Name>,
    pub code_block: Id<CodeBlock>,
}

pub struct LabelStack {

}

impl LabelStack {
    pub fn new() -> Self {
        Self {}
    }

    pub fn push(&mut self, label: Option<Name>, code_block: Id<CodeBlock>) {

    }

    pub fn pop(&mut self) {

    }

    pub fn last(&self) -> Option<&LoopLabel> {
        None
    }

    pub fn find(&self, name: &str) -> Option<&LoopLabel> {
        None
    }
}
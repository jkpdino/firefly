use firefly_hir::{stmt::CodeBlock, Id, Name};

pub struct LoopLabel {
    pub label: Option<Name>,
    pub code_block: Id<CodeBlock>,
}

pub struct LabelStack {
    stack: Vec<LoopLabel>
}

impl LabelStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, label: Option<Name>, code_block: Id<CodeBlock>) {
        self.stack.push(LoopLabel { label, code_block });
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn last(&self) -> Option<&LoopLabel> {
        self.stack.last()
    }

    pub fn find(&self, name: &str) -> Option<&LoopLabel> {
        for loop_label in self.stack.iter().rev() {
            let Some(label) = &loop_label.label else { continue };

            if label.name == name {
                return Some(loop_label);
            }
        }

        return None;
    }
}
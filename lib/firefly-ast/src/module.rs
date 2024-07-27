use crate::Path;

#[derive(Debug)]
pub struct Module {
    pub path: Path
}

impl Default for Module {
    fn default() -> Self {
        Self { path: Default::default() }
    }
}
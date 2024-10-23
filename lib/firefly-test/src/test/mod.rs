use std::path::PathBuf;

use directive::{parse_test_directives, TestDirectives};

pub mod source;
pub mod suite;
pub mod run;
pub mod directive;

#[derive(Debug, Clone)]
pub struct Test {
    pub index: usize,
    pub path: PathBuf,
    pub name: String,   
    pub directives: TestDirectives,
}

impl Test {
    pub fn new(index: usize, path: PathBuf, name: String) -> Self {
        let content = std::fs::read_to_string(&path).unwrap();

        let directives = parse_test_directives(&content);

        Self { index, path, name, directives }
    }
}
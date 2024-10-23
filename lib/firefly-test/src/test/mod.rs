use std::path::PathBuf;

pub mod source;
pub mod suite;
pub mod run;

#[derive(Debug, Clone)]
pub struct Test {
    pub index: usize,
    pub path: PathBuf,
    pub name: String,   
}
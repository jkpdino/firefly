use super::suite::TestSuite;

pub mod walker;

pub trait TestSource {
  fn gather(&self) -> TestSuite;
}
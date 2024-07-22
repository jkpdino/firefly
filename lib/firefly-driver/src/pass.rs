/// A pass takes an input and processes it.
///
/// Each layer of the pass takes an input and a context
///
/// The
pub trait Pass<Input, Output> {
    fn process(&self, input: Self::Input) -> Self::Output {}
}

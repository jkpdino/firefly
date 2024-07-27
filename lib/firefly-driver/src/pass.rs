pub mod parse;
pub mod lower;

use crate::context::Context;

/// A pass takes an input and processes it.
///
/// Each layer of the pass takes an input and a context
///
/// The
pub trait Pass {
    type Input;
    type Output;

    fn process(&self, input: Self::Input, emitter: &mut Context) -> Self::Output;
}

pub trait ParallelPass {
    type Input;
    type Output;

    fn process(&self, input: Self::Input, emitter: &mut Context) -> Self::Output;
}

impl<T: ParallelPass> Pass for T {
    type Input = Vec<<Self as ParallelPass>::Input>;
    type Output = Vec<<Self as ParallelPass>::Output>;

    fn process(&self, input: Self::Input, context: &mut Context) -> Self::Output {
        input.into_iter().map(|input| self.process(input, context)).collect()
    }
}
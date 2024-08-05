pub mod parse;
pub mod lower;
pub mod vir_lower;

use std::marker::PhantomData;

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

pub struct IgnorePass<In, Out>
{
    _phantom: PhantomData<(In, Out)>
}

impl<In, Out> Pass for IgnorePass<In, Out>
    where Out: Default
{
    type Input = In;

    type Output = Out;

    fn process(&self, _: Self::Input, _: &mut Context) -> Self::Output {
        Out::default()
    }
}

impl<In, Out> IgnorePass<In, Out> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
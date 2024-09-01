use crate::{context::Context, pass::Pass};

pub trait Pipeline {
    type Input;

    fn run(&self, input: Self::Input, context: &mut Context);
}

impl<T1> Pipeline for (T1,)
    where T1: Pass
{
    type Input = T1::Input;

    fn run(&self, input: Self::Input, context: &mut Context) {
        self.0.process(input, context);
    }
}

impl<T1, T2> Pipeline for (T1, T2)
    where T1: Pass,
          T2: Pass<Input = T1::Output>
{
    type Input = T1::Input;

    fn run(&self, input: Self::Input, context: &mut Context) {
        let output1 = self.0.process(input, context);
        if context.emitter.has_triggered() { return; }
        self.1.process(output1, context);
    }
}

impl<T1, T2, T3> Pipeline for (T1, T2, T3)
    where T1: Pass,
          T2: Pass<Input = T1::Output>,
          T3: Pass<Input = T2::Output>
{
    type Input = T1::Input;

    fn run(&self, input: Self::Input, context: &mut Context) {
        let output1 = self.0.process(input, context);
        if context.emitter.has_triggered() { return; }
        let output2 = self.1.process(output1, context);
        if context.emitter.has_triggered() { return; }
        self.2.process(output2, context);
    }
}

impl<T1, T2, T3, T4> Pipeline for (T1, T2, T3, T4)
    where T1: Pass,
          T2: Pass<Input = T1::Output>,
          T3: Pass<Input = T2::Output>,
          T4: Pass<Input = T3::Output>
{
    type Input = T1::Input;

    fn run(&self, input: Self::Input, context: &mut Context) {
        let output1 = self.0.process(input, context);
        if context.emitter.has_triggered() { return; }
        let output2 = self.1.process(output1, context);
        if context.emitter.has_triggered() { return; }
        let output3 = self.2.process(output2, context);
        if context.emitter.has_triggered() { return; }
        self.3.process(output3, context);
    }
}

impl<T1, T2, T3, T4, T5> Pipeline for (T1, T2, T3, T4, T5)
    where T1: Pass,
          T2: Pass<Input = T1::Output>,
          T3: Pass<Input = T2::Output>,
          T4: Pass<Input = T3::Output>,
          T5: Pass<Input = T4::Output>
{
    type Input = T1::Input;

    fn run(&self, input: Self::Input, context: &mut Context) {
        let output1 = self.0.process(input, context);
        if context.emitter.has_triggered() { return; }
        let output2 = self.1.process(output1, context);
        if context.emitter.has_triggered() { return; }
        let output3 = self.2.process(output2, context);
        if context.emitter.has_triggered() { return; }
        let output4 = self.3.process(output3, context);
        if context.emitter.has_triggered() { return; }
        self.4.process(output4, context);
    }
}

impl<T1, T2, T3, T4, T5, T6> Pipeline for (T1, T2, T3, T4, T5, T6)
    where T1: Pass,
          T2: Pass<Input = T1::Output>,
          T3: Pass<Input = T2::Output>,
          T4: Pass<Input = T3::Output>,
          T5: Pass<Input = T4::Output>,
          T6: Pass<Input = T5::Output>
{
    type Input = T1::Input;

    fn run(&self, input: Self::Input, context: &mut Context) {
        let output1 = self.0.process(input, context);
        if context.emitter.has_triggered() { return; }
        let output2 = self.1.process(output1, context);
        if context.emitter.has_triggered() { return; }
        let output3 = self.2.process(output2, context);
        if context.emitter.has_triggered() { return; }
        let output4 = self.3.process(output3, context);
        if context.emitter.has_triggered() { return; }
        let output5 = self.4.process(output4, context);
        if context.emitter.has_triggered() { return; }
        self.5.process(output5, context);
    }
}
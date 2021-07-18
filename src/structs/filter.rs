use crate::{Generator, GeneratorResult, ValueResult};

/// Implements a filtered generator. See [`.filter()`](crate::GeneratorExt::filter) for more details.
#[derive(Clone)]
pub struct Filter<Gen, Pred> {
    generator: Gen,
    predicate: Pred,
}

impl<Gen, Pred> Filter<Gen, Pred>
where
    Gen: Generator,
    Pred: FnMut(&Gen::Output) -> bool,
{
    #[inline]
    pub(crate) fn new(generator: Gen, predicate: Pred) -> Self {
        Self {
            generator,
            predicate,
        }
    }
}

impl<Gen, Pred> Generator for Filter<Gen, Pred>
where
    Gen: Generator,
    Pred: FnMut(&Gen::Output) -> bool,
{
    type Output = Gen::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let (generator, predicate) = (&mut self.generator, &mut self.predicate);
        generator.run(move |x| {
            if predicate(&x) {
                output(x)
            } else {
                ValueResult::MoreValues
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test::StoppingGen;
    use crate::{GeneratorExt, GeneratorResult};

    #[test]
    fn spuriously_stopping() {
        let data = [1, 2, 3];
        fn is_odd(v: &&i32) -> bool {
            **v % 2 != 0
        }

        for x in 0..data.len() {
            let mut gen = StoppingGen::new(x as i32, &data).filter(is_odd);
            let mut output = Vec::new();
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [&1, &3]);
        }
    }
}

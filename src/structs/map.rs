use crate::{Generator, GeneratorResult, ReverseGenerator, ValueResult};
use core::num::NonZeroUsize;

/// Implements a mapped generator. See [`.map()`](crate::GeneratorExt::map) for details.
#[derive(Clone)]
pub struct Map<Gen, Func> {
    source: Gen,
    transform: Func,
}

impl<Gen, Func, Out> Map<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Out,
{
    #[inline]
    pub(crate) fn new(source: Gen, transform: Func) -> Self {
        Self { source, transform }
    }
}

impl<Gen, Func, Out> Generator for Map<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Out,
{
    type Output = Out;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let transform = &mut self.transform;
        self.source.run(move |value| output(transform(value)))
    }

    #[inline]
    fn try_advance(&mut self, n: core::num::NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance(n)
    }
}

impl<Gen, Func, Out> ReverseGenerator for Map<Gen, Func>
where
    Gen: ReverseGenerator,
    Func: FnMut(Gen::Output) -> Out,
{
    #[inline]
    fn run_back(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let transform = &mut self.transform;
        self.source.run_back(move |v| output(transform(v)))
    }

    #[inline]
    fn try_advance_back(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        self.source.try_advance_back(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::StoppingGen;
    use crate::{GeneratorExt, GeneratorResult, ReverseGenerator, SliceGenerator};
    use std::num::NonZeroUsize;

    #[test]
    fn spuriously_stopping() {
        let data = [1, 2, 3];

        for x in 0..data.len() {
            let mut gen = StoppingGen::new(x as i32, &data).map(|x| x * 2);
            let mut output = Vec::new();
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [2, 2 * 2, 2 * 3]);
        }
    }

    #[test]
    fn reverse() {
        let data = [1, 2, 3];
        fn x2(v: &i32) -> i32 {
            v * 2
        }

        let mut gen = SliceGenerator::new(&data).map(x2);
        assert_eq!(gen.next_back(), Ok(6));
        assert_eq!(gen.next_back(), Ok(4));
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));

        let mut gen = SliceGenerator::new(&data).map(x2);
        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        assert_eq!(gen.next_back(), Ok(4));
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
    }
}

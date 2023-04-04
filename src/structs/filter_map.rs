use crate::{Generator, GeneratorResult, ReverseGenerator, ValueResult};

/// Implements a mapped generator. See [`.map()`](crate::GeneratorExt::map) for details.
#[derive(Clone)]
pub struct FilterMap<Gen, Func> {
    source: Gen,
    transform: Func,
}

impl<Gen, Func, Out> FilterMap<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Option<Out>,
{
    #[inline]
    pub(crate) fn new(source: Gen, transform: Func) -> Self {
        Self { source, transform }
    }
}

impl<Gen, Func, Out> Generator for FilterMap<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Option<Out>,
{
    type Output = Out;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let (source, transform) = (&mut self.source, &mut self.transform);
        source.run(move |x| {
            if let Some(x) = transform(x) {
                output(x)
            } else {
                ValueResult::MoreValues
            }
        })
    }
}

impl<Gen, Func, Out> ReverseGenerator for FilterMap<Gen, Func>
where
    Gen: ReverseGenerator,
    Func: FnMut(Gen::Output) -> Option<Out>,
{
    #[inline]
    fn run_back(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let (source, transform) = (&mut self.source, &mut self.transform);
        source.run_back(move |x| {
            if let Some(x) = transform(x) {
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
    use crate::{GeneratorExt, GeneratorResult, ReverseGenerator, SliceGenerator};
    use std::num::NonZeroUsize;

    #[test]
    fn spuriously_stopping() {
        let data = [1, 2, 3];
        fn filter_map_odd(v: &i32) -> Option<i32> {
            if v % 2 != 0 {
                Some(v * 2)
            } else {
                None
            }
        }

        for x in 0..data.len() {
            let mut gen = StoppingGen::new(x as i32, &data).filter_map(filter_map_odd);
            let mut output = Vec::new();
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [2, 2 * 3]);
        }
    }

    #[test]
    fn reverse() {
        let data = [1, 2, 3];
        fn filter_map_odd(v: &i32) -> Option<i32> {
            if v % 2 != 0 {
                Some(v * 2)
            } else {
                None
            }
        }

        let mut gen = SliceGenerator::new(&data).filter_map(filter_map_odd);
        assert_eq!(gen.next_back(), Ok(6));
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));

        let mut gen = SliceGenerator::new(&data).filter_map(filter_map_odd);
        gen.try_advance_back(NonZeroUsize::new(1).unwrap());
        assert_eq!(gen.next_back(), Ok(2));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
    }
}

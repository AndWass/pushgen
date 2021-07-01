use crate::{Generator, GeneratorResult, ValueResult};

/// Flatten generator implementation. See [`.flatten()`](crate::GeneratorExt::flatten) for details.
pub struct Flatten<Src, AdaptorFn, Adaptor>
where
    Src: Generator,
    Adaptor: Generator,
    AdaptorFn: FnMut(Src::Output) -> Adaptor,
{
    source: Src,
    adaptor_gen: AdaptorFn,
    current_adaptor: Option<Adaptor>,
}

impl<Src, AdaptorFn, Adaptor> Flatten<Src, AdaptorFn, Adaptor>
where
    Src: Generator,
    Adaptor: Generator,
    AdaptorFn: FnMut(Src::Output) -> Adaptor,
{
    pub(crate) fn new(source: Src, adaptor_gen: AdaptorFn) -> Self {
        Self {
            source,
            adaptor_gen,
            current_adaptor: None,
        }
    }
}

impl<Src, AdaptorFn, Adaptor> Generator for Flatten<Src, AdaptorFn, Adaptor>
where
    Src: Generator,
    Adaptor: Generator,
    AdaptorFn: FnMut(Src::Output) -> Adaptor,
{
    type Output = Adaptor::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if let Some(current) = self.current_adaptor.as_mut() {
            if current.run(|x| output(x)) == GeneratorResult::Stopped {
                return GeneratorResult::Stopped;
            }
        }

        let current_adaptor = &mut self.current_adaptor;
        let adaptor_gen = &mut self.adaptor_gen;
        self.source.run(|x| {
            *current_adaptor = Some(adaptor_gen(x));
            match current_adaptor.as_mut().unwrap().run(|value| output(value)) {
                GeneratorResult::Stopped => ValueResult::Stop,
                GeneratorResult::Complete => ValueResult::MoreValues,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GeneratorExt, SliceGenerator};

    #[test]
    fn vector_flatten() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10]];
        let mut output: Vec<i32> = Vec::new();
        let result = Flatten::new(SliceGenerator::new(data.as_slice()), |x| {
            SliceGenerator::new(x.as_slice())
        })
        .for_each(|x| output.push(*x));

        assert_eq!(output, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(result, GeneratorResult::Complete);
    }

    #[test]
    fn slice_flatten() {
        let data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let mut output = Vec::new();
        let result = SliceGenerator::new(&data)
            .flatten(|x| SliceGenerator::new(x))
            .for_each(|x| output.push(*x));
        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn stopping_generator() {
        let data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        for x in 0..10 {
            let mut gen =
                crate::test::StoppingGen::new(x, &data).flatten(|v| SliceGenerator::new(v));

            let mut output = Vec::new();

            while gen.for_each(|x| output.push(*x)) == GeneratorResult::Stopped {}

            assert_eq!(output, expected);
        }
    }
}

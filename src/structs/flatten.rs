use crate::{
    structs::utility::set_some, Generator, GeneratorResult, IntoGenerator, ReverseGenerator,
    ValueResult,
};

/// Flatten generator implementation. See [`.flatten()`](crate::GeneratorExt::flatten) for details.
pub struct Flatten<Src>
where
    Src: Generator,
    Src::Output: IntoGenerator,
{
    source: Src,
    current_generator: Option<<Src::Output as IntoGenerator>::IntoGen>,
    current_back_generator: Option<<Src::Output as IntoGenerator>::IntoGen>,
}

impl<Src> Flatten<Src>
where
    Src: Generator,
    Src::Output: IntoGenerator,
{
    #[inline]
    pub(crate) fn new(source: Src) -> Self {
        Self {
            source,
            current_generator: None,
            current_back_generator: None,
        }
    }
}

// #[derive(Clone)] caused compilation error, probably due to current_generator not being
// one of the generic arguments. So we do it by hand instead.
impl<Src> Clone for Flatten<Src>
where
    Src: Generator + Clone,
    Src::Output: IntoGenerator,
    <Src::Output as IntoGenerator>::IntoGen: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            current_generator: self.current_generator.clone(),
            current_back_generator: self.current_back_generator.clone(),
        }
    }
}

impl<Src> Generator for Flatten<Src>
where
    Src: Generator,
    Src::Output: IntoGenerator,
{
    type Output = <<Src as Generator>::Output as IntoGenerator>::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if let Some(current) = self.current_generator.as_mut() {
            if current.run(&mut output) == GeneratorResult::Stopped {
                return GeneratorResult::Stopped;
            }
        }

        let current_generator = &mut self.current_generator;
        let result = self.source.run(|x| {
            match set_some(current_generator, x.into_gen()).run(&mut output) {
                GeneratorResult::Stopped => ValueResult::Stop,
                GeneratorResult::Complete => ValueResult::MoreValues,
            }
        });

        if result == GeneratorResult::Complete {
            if let Some(mut last) = self.current_back_generator.take() {
                return if last.run(output) == GeneratorResult::Stopped {
                    self.current_back_generator = Some(last);
                    GeneratorResult::Stopped
                } else {
                    GeneratorResult::Complete
                };
            }
        }

        result
    }
}

impl<Src> ReverseGenerator for Flatten<Src>
where
    Src: ReverseGenerator,
    Src::Output: IntoGenerator,
    <Src::Output as IntoGenerator>::IntoGen: ReverseGenerator,
{
    #[inline]
    fn run_back(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if let Some(mut current) = self.current_back_generator.take() {
            if current.run_back(&mut output) == GeneratorResult::Stopped {
                self.current_back_generator = Some(current);
                return GeneratorResult::Stopped;
            }
        }

        let current = &mut self.current_back_generator;
        let result = self.source.run_back(|x| {
            match set_some(current, x.into_gen()).run_back(&mut output) {
                GeneratorResult::Stopped => ValueResult::Stop,
                GeneratorResult::Complete => ValueResult::MoreValues,
            }
        });

        if result == GeneratorResult::Complete {
            if let Some(mut last) = self.current_generator.take() {
                return if last.run_back(output) == GeneratorResult::Stopped {
                    self.current_generator = Some(last);
                    GeneratorResult::Stopped
                } else {
                    GeneratorResult::Complete
                };
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::StoppingGen;
    use crate::{GeneratorExt, SliceGenerator};

    #[test]
    fn vector_flatten() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10]];
        let mut output: Vec<i32> = Vec::new();
        let result = SliceGenerator::new(data.as_slice())
            .map(|x| SliceGenerator::new(x.as_slice()))
            .flatten()
            .for_each(|x| output.push(*x));

        assert_eq!(output, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(result, GeneratorResult::Complete);
    }

    #[test]
    fn slice_flatten() {
        let data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let mut output = Vec::new();
        let result = SliceGenerator::new(&data)
            .map(|x| SliceGenerator::new(x))
            .flatten()
            .for_each(|x| output.push(*x));
        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn stopping_generator() {
        let data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        for x in 0..3 {
            let mut gen = crate::test::StoppingGen::new(x, &data)
                .map(|x| SliceGenerator::new(x))
                .flatten();

            let mut output = Vec::new();
            let mut num_stops = 0;
            while gen.for_each(|x| output.push(*x)) == GeneratorResult::Stopped {
                num_stops += 1;
            }
            assert_eq!(num_stops, 1);
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn stopping_nested_generator() {
        let data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        for i in 0..4 {
            let mut gen = SliceGenerator::new(&data)
                .map(|x| StoppingGen::new(i, x))
                .flatten();

            let mut output = Vec::new();
            let mut num_stops = 0;
            while gen.for_each(|x| output.push(*x)) == GeneratorResult::Stopped {
                num_stops += 1;
            }
            assert_eq!(num_stops, 3);
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn only_stopping_generators() {
        let data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        for outer in 0..3 {
            for i in 0..4 {
                let mut gen = StoppingGen::new(outer, &data)
                    .map(|x| StoppingGen::new(i, x))
                    .flatten();

                let mut output = Vec::new();

                let mut num_stops = 0;
                while gen.for_each(|x| output.push(*x)) == GeneratorResult::Stopped {
                    num_stops += 1;
                }
                assert_eq!(num_stops, 4);
                assert_eq!(output, expected);
            }
        }
    }

    #[test]
    fn reverse() {
        let data = [[1, 2], [3, 4], [5, 6]];
        let mut gen = SliceGenerator::new(&data).flatten();
        assert_eq!(gen.next_back(), Ok(&6));
        assert_eq!(gen.next_back(), Ok(&5));
        assert_eq!(gen.next_back(), Ok(&4));
        assert_eq!(gen.next_back(), Ok(&3));
        assert_eq!(gen.next_back(), Ok(&2));
        assert_eq!(gen.next_back(), Ok(&1));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn forward_then_reverse() {
        let data = [[1, 2], [3, 4], [5, 6]];
        let mut gen = SliceGenerator::new(&data).flatten();
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next_back(), Ok(&6));
        assert_eq!(gen.next_back(), Ok(&5));
        assert_eq!(gen.next_back(), Ok(&4));
        assert_eq!(gen.next_back(), Ok(&3));
        assert_eq!(gen.next_back(), Ok(&2));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }
    #[test]
    fn forward_then_reverse2() {
        let data = [[1, 2], [3, 4], [5, 6]];
        let mut gen = SliceGenerator::new(&data).flatten();
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next_back(), Ok(&6));
        assert_eq!(gen.next_back(), Ok(&5));
        assert_eq!(gen.next_back(), Ok(&4));
        assert_eq!(gen.next_back(), Ok(&3));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn forward_then_reverse3() {
        let data = [[1, 2], [3, 4], [5, 6]];
        let mut gen = SliceGenerator::new(&data).flatten();
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Ok(&3));
        assert_eq!(gen.next_back(), Ok(&6));
        assert_eq!(gen.next_back(), Ok(&5));
        assert_eq!(gen.next_back(), Ok(&4));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn reverse_then_foward3() {
        let data = [[1, 2], [3, 4], [5, 6]];
        let mut gen = SliceGenerator::new(&data).flatten();
        assert_eq!(gen.next_back(), Ok(&6));
        assert_eq!(gen.next_back(), Ok(&5));
        assert_eq!(gen.next_back(), Ok(&4));
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next(), Ok(&3));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn reverse_then_foward_alt() {
        let data = [[1, 2], [3, 4], [5, 6]];
        let mut gen = SliceGenerator::new(&data).flatten();
        assert_eq!(gen.next_back(), Ok(&6));
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next_back(), Ok(&5));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next_back(), Ok(&4));
        assert_eq!(gen.next(), Ok(&3));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
    }

    #[test]
    fn forward_then_reverse_alt() {
        let data = [[1, 2], [3, 4], [5, 6]];
        let mut gen = SliceGenerator::new(&data).flatten();
        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next_back(), Ok(&6));
        assert_eq!(gen.next(), Ok(&2));
        assert_eq!(gen.next_back(), Ok(&5));
        assert_eq!(gen.next(), Ok(&3));
        assert_eq!(gen.next_back(), Ok(&4));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete));
        assert_eq!(gen.next_back(), Err(GeneratorResult::Complete));
    }
}

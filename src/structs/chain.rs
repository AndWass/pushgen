use crate::{Generator, GeneratorResult, ValueResult};

/// Implements a chained generator. See [`.chain()`](crate::GeneratorExt::chain) for details.
#[derive(Clone)]
pub struct Chain<First, Second> {
    first: First,
    second: Second,
    first_active: bool,
}

impl<First, Second> Chain<First, Second> {
    #[inline]
    pub(crate) fn new(first: First, second: Second) -> Self {
        Self {
            first,
            second,
            first_active: true,
        }
    }
}

impl<First, Second> Generator for Chain<First, Second>
where
    First: Generator,
    Second: Generator<Output = First::Output>,
{
    type Output = First::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if self.first_active {
            let result = self.first.run(|x| output(x));
            if result == GeneratorResult::Stopped {
                return GeneratorResult::Stopped;
            }
            self.first_active = false;
        }
        self.second.run(|x| output(x))
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::chain::Chain;
    use crate::test::StoppingGen;
    use crate::{Generator, GeneratorResult, ValueResult};
    use crate::{GeneratorExt, SliceGenerator};

    #[test]
    fn basic_chain() {
        let data = [1, 2, 3];
        let mut output: Vec<i32> = Vec::new();
        let result = Chain::new(SliceGenerator::new(&data), SliceGenerator::new(&data)).run(|x| {
            output.push(*x);
            ValueResult::MoreValues
        });

        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn spuriously_stopping_first() {
        let data = [1, 2, 3];
        for x in 0..3 {
            let first = StoppingGen::new(x, &data);
            let second = SliceGenerator::new(&data);
            let mut output: Vec<i32> = Vec::new();
            let mut gen = first.chain(second);
            let result = gen.for_each(|x| output.push(*x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(*x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [1, 2, 3, 1, 2, 3]);
        }
    }

    #[test]
    fn spuriously_stopping_second() {
        let data = [1, 2, 3];
        for x in 0..3 {
            let second = StoppingGen::new(x, &data);
            let first = SliceGenerator::new(&data);
            let mut output: Vec<i32> = Vec::new();
            let mut gen = first.chain(second);
            let result = gen.for_each(|x| output.push(*x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(*x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [1, 2, 3, 1, 2, 3]);
        }
    }
}

use crate::{Generator, GeneratorResult, ValueResult};

/// A generator that copies the elements of an underlying generator. See [`.copied()`](crate::GeneratorExt::copied) for details.
#[derive(Clone)]
pub struct Copied<Src> {
    source: Src,
}

impl<Src> Copied<Src> {
    pub(crate) fn new(source: Src) -> Self {
        Self { source }
    }
}

impl<'a, Src, T> Generator for Copied<Src>
where
    T: 'a + Copy,
    Src: Generator<Output = &'a T>,
{
    type Output = T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        self.source.run(|&x| output(x))
    }
}

mod tests
{
    use crate::test::StoppingGen;
    use crate::{GeneratorExt, GeneratorResult};

    #[test]
    fn spuriously_stopping() {
        let data = [1, 2, 3];
        for x in 0..data.len() {
            let mut gen = StoppingGen::new(x as i32, &data).copied();
            let mut output: Vec<i32> = Vec::new();
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [1, 2, 3]);
        }
    }
}

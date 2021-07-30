use crate::{Generator, GeneratorResult, ValueResult};

/// Skip over a set amount of values. See [`.skip()`](crate::GeneratorExt::skip) for more details.
#[derive(Clone)]
pub struct Skip<Gen> {
    generator: Gen,
    amount: usize,
}

impl<Gen> Skip<Gen> {
    #[inline]
    pub(crate) fn new(generator: Gen, amount: usize) -> Self {
        Self { generator, amount }
    }
}

impl<Gen> Generator for Skip<Gen>
where
    Gen: Generator,
{
    type Output = Gen::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if self.amount > 0 {
            // Safety: checked by if clause
            match self
                .generator
                .try_advance(unsafe { core::num::NonZeroUsize::new_unchecked(self.amount) })
            {
                (_, GeneratorResult::Complete) => {
                    self.amount = 0;
                    return GeneratorResult::Complete;
                }
                (x, _) => {
                    self.amount -= x;
                    if self.amount != 0 {
                        return GeneratorResult::Stopped;
                    }
                }
            }
        }

        self.generator.run(|value| output(value))
    }
}

#[derive(Clone)]
pub struct SkipWhile<Src, P> {
    source: Src,
    predicate: P,
    need_skip_run: bool,
}

impl<Src, P> SkipWhile<Src, P>
where
    Src: Generator,
    P: FnMut(&Src::Output) -> bool,
{
    #[inline]
    pub(crate) fn new(source: Src, predicate: P) -> Self {
        Self {
            source,
            predicate,
            need_skip_run: true,
        }
    }
}

impl<Src, P> Generator for SkipWhile<Src, P>
where
    Src: Generator,
    P: FnMut(&Src::Output) -> bool,
{
    type Output = Src::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if self.need_skip_run {
            let mut first_to_push = None;
            let predicate = &mut self.predicate;

            let skip_run_result = self.source.run(|x| {
                if predicate(&x) {
                    ValueResult::MoreValues
                } else {
                    first_to_push = Some(x);
                    ValueResult::Stop
                }
            });

            if skip_run_result == GeneratorResult::Complete {
                return GeneratorResult::Complete;
            } else if let Some(x) = first_to_push {
                self.need_skip_run = false;
                if output(x) == ValueResult::Stop {
                    return GeneratorResult::Stopped;
                }
            } else {
                return GeneratorResult::Stopped;
            }
        }
        self.source.run(|x| output(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::StoppingGen;
    use crate::{GeneratorExt, IntoGenerator};

    #[test]
    fn skip() {
        let a = [1, 2, 3];

        let mut gen = Skip::new(a.into_gen(), 2);
        let mut output = Vec::new();
        let result = gen.for_each(|x| output.push(x));
        assert_eq!(output, [&3]);
        assert_eq!(result, GeneratorResult::Complete);
    }

    #[test]
    fn spuriously_stopping_skip() {
        let data = [1, 2, 3, 4, 5];
        for x in 0..5 {
            let mut gen = StoppingGen::new(x, &data).skip(3);
            let mut output = Vec::new();
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [&4, &5]);
        }
    }

    #[test]
    fn skip_while() {
        let a = [-1i32, 0, 1];

        let mut gen = SkipWhile::new(a.into_gen(), |x| x.is_negative());
        let mut output = Vec::new();
        let result = gen.for_each(|x| output.push(x));
        assert_eq!(output, [&0, &1]);
        assert_eq!(result, GeneratorResult::Complete);
    }

    #[test]
    fn spuriously_stopping_skip_while() {
        let data = [-1i32, -2, 0, -1, 2];
        for x in 0..5 {
            let mut gen = StoppingGen::new(x, &data).skip_while(|x| x.is_negative());
            let mut output = Vec::new();
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|x| output.push(x));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [&0, &-1, &2]);
        }
    }
}

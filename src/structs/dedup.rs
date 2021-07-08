use crate::{Generator, GeneratorResult, ValueResult};
use core::mem;

/// Deduplication of duplicate consecutive values. See [`.dedup()`](crate::GeneratorExt::dedup) for details.
pub struct Dedup<Src>
where
    Src: Generator,
    Src::Output: PartialEq,
{
    source: Src,
    next: Option<Src::Output>,
}

impl<Src> Dedup<Src>
where
    Src: Generator,
    Src::Output: PartialEq,
{
    #[inline]
    pub(crate) fn new(source: Src) -> Self {
        Self { source, next: None }
    }
}

impl<Src> Generator for Dedup<Src>
where
    Src: Generator,
    Src::Output: PartialEq,
{
    type Output = Src::Output;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let mut prev = match self.next.take() {
            Some(value) => value,
            None => {
                let next = &mut self.next;
                // Try to get the initial value
                let take_one_res = self.source.run(|x| {
                    *next = Some(x);
                    ValueResult::Stop
                });

                match self.next.take() {
                    Some(value) => value,
                    None => return take_one_res,
                }
            }
        };

        let mut result = self.source.run(|x| {
            if x == prev {
                // Removing this line causes the regression of the performance of
                // bench pushgen_dedup_flatten_filter_map
                prev = x;
                ValueResult::MoreValues
            } else {
                output(mem::replace(&mut prev, x))
            }
        });

        // if it was complete we assume no more values will be generated and
        // we need to output the last held value.
        if result == GeneratorResult::Complete {
            if output(prev) == ValueResult::Stop {
                result = GeneratorResult::Stopped;
            }
        } else {
            // If the source generator was stopped we might have more values
            // coming later runs,
            self.next = Some(prev);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GeneratorExt, SliceGenerator};

    fn run<Gen: Generator>(mut gen: Gen) -> Vec<Gen::Output> {
        let mut output: Vec<Gen::Output> = Vec::new();
        while gen.for_each(|x| output.push(x)) == GeneratorResult::Stopped {}
        output
    }

    #[test]
    fn dedup_nonduplicate() {
        let data = [1, 2, 3, 4];
        let out = run(Dedup::new(SliceGenerator::new(&data).map(|x| *x)));
        assert_eq!(out, [1, 2, 3, 4]);
    }

    #[test]
    fn dedup_all_duplicate() {
        let data = [1, 1, 2, 2, 3, 3, 4, 4];
        let out = run(Dedup::new(SliceGenerator::new(&data).map(|x| *x)));
        assert_eq!(out, [1, 2, 3, 4]);
    }

    #[test]
    fn dedup_some_duplicate() {
        let data = [1, 2, 2, 3, 3, 4];
        let out = run(Dedup::new(SliceGenerator::new(&data).map(|x| *x)));
        assert_eq!(out, [1, 2, 3, 4]);
    }

    #[test]
    fn dedup_stopping_source() {
        let data = [1, 2, 2, 3, 3, 4];

        for x in 0..10 {
            let gen = crate::test::StoppingGen::new(x, &data);

            let out = run(Dedup::new(gen).map(|x| *x));
            if out != [1, 2, 3, 4] {
                println!("Failed x = {}", x);
            }
            assert_eq!(out, [1, 2, 3, 4]);
        }
    }
}

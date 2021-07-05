use crate::{Generator, GeneratorResult, ValueResult};

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
        if self.next.is_none() {
            let next = &mut self.next;
            // Try to get the initial value
            let take_one_res = self.source.run(|x| {
                *next = Some(x);
                ValueResult::Stop
            });

            if !next.is_some() {
                return take_one_res;
            }
        }

        // self.next.is_some() == true always.
        let next = &mut self.next;

        let mut result = self.source.run(|x| {
            let next_value = next.take().unwrap();
            let is_equal = x == next_value;
            *next = Some(x);
            if is_equal {
                ValueResult::MoreValues
            } else {
                output(next_value)
            }
        });

        // If the source generator was stopped we might have more values coming later runs,
        // but if it was complete we assume no more values will be generated and
        // we need to output the last held value.
        if result == GeneratorResult::Complete
            && output(self.next.take().unwrap()) == ValueResult::Stop
        {
            result = GeneratorResult::Stopped;
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

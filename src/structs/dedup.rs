use crate::{Generator, GeneratorResult, ValueResult, structs::utility::unwrap_unchecked};

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
        let prev = match &mut self.next {
            Some(value) => value,
            None => {
                let next = &mut self.next;
                // Try to get the initial value
                let take_one_res = self.source.run(|x| {
                    *next = Some(x);
                    ValueResult::Stop
                });

                match &mut self.next {
                    Some(value) => value,
                    None => return take_one_res
                }
            }
        };

        let mut result = self.source.run(|x| {
            if x == *prev {
                *prev = x;
                ValueResult::MoreValues
            } else {
                output(std::mem::replace(prev, x))
            }
        });

        // If the source generator was stopped we might have more values coming later runs,
        // but if it was complete we assume no more values will be generated and
        // we need to output the last held value.
        if result == GeneratorResult::Complete
            && output(unsafe { unwrap_unchecked(self.next.take()) }) == ValueResult::Stop
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

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
    pub(crate) fn new(source: Src) -> Self {
        Self {
            source,
            next: None,
        }
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
        if !self.next.is_some() {
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
            if x == next_value {
                *next = Some(x);
                ValueResult::MoreValues
            }
            else {
                *next = Some(x);
                output(next_value)
            }
        });

        // If the source generator was stopped we might have more values coming later runs,
        // but if it was complete we assume no more values will be generated and
        // we need to output the last held value.
        if result == GeneratorResult::Complete {
            if output(self.next.take().unwrap()) == ValueResult::Stop {
                result = GeneratorResult::Stopped;
            }
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
        struct StoppingGen<'a> {
            stop_at: i32,
            stopped_data: Option<&'a i32>,
            data: SliceGenerator<'a, i32>,
        }

        impl<'a> Generator for StoppingGen<'a> {
            type Output = i32;

            fn run(
                &mut self,
                mut output: impl FnMut(Self::Output) -> ValueResult,
            ) -> GeneratorResult {
                if self.stop_at == 0 {
                    self.stop_at -= 1;
                    return GeneratorResult::Stopped;
                }

                if let Some(x) = self.stopped_data.take() {
                    if output(*x) == ValueResult::Stop {
                        return GeneratorResult::Stopped;
                    }
                }

                let stored_stop = &mut self.stopped_data;
                let stop_at = &mut self.stop_at;
                let result =
                self.data.run(|x| {
                    let old_stop_at = *stop_at;
                    *stop_at -= 1;
                    if old_stop_at == 0 {
                        *stored_stop = Some(x);
                        ValueResult::Stop
                    }
                    else {
                        output(*x)
                    }
                });
                if result == GeneratorResult::Complete {
                    *stop_at = -1;
                }
                result
            }
        }

        let data = [1, 2, 2, 3, 3, 4];

        //for x in 0..10 {
            let gen = StoppingGen {
                stop_at: 5,
                data: SliceGenerator::new(&data),
                stopped_data: None,
            };

            let out = run(Dedup::new(gen));
            if out != [1, 2, 3, 4] {
                //println!("Failed x = {}", x);
            }
            assert_eq!(out, [1, 2, 3, 4]);
        //}
    }
}

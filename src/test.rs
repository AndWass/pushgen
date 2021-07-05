use crate::{Generator, GeneratorResult, SliceGenerator, ValueResult};

pub struct StoppingGen<'a, T> {
    stop_at: i32,
    stopped_data: Option<&'a T>,
    data: SliceGenerator<'a, T>,
}

impl<'a, T> StoppingGen<'a, T> {
    pub fn new(stop_at: i32, data: &'a [T]) -> Self {
        Self {
            stop_at,
            stopped_data: None,
            data: SliceGenerator::new(data),
        }
    }
}

impl<'a, T> Generator for StoppingGen<'a, T> {
    type Output = &'a T;

    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        if self.stop_at == 0 {
            self.stop_at -= 1;
            return GeneratorResult::Stopped;
        }

        if let Some(x) = self.stopped_data.take() {
            if output(x) == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }

        let stored_stop = &mut self.stopped_data;
        let stop_at = &mut self.stop_at;
        let result = self.data.run(|x| {
            let old_stop_at = *stop_at;
            *stop_at -= 1;
            if old_stop_at == 0 {
                *stored_stop = Some(x);
                ValueResult::Stop
            } else {
                output(x)
            }
        });
        if result == GeneratorResult::Complete {
            *stop_at = -1;
        }
        result
    }
}

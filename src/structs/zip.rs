use crate::{Generator, GeneratorExt, GeneratorResult, ValueResult};

/// Zip two generators. See [`.zip()`](crate::GeneratorExt::zip) for details.
#[derive(Clone)]
pub struct Zip<Left, Right>
where
    Left: Generator,
{
    left: Left,
    right: Right,
    last_left: Option<Left::Output>,
}

impl<Left, Right> Zip<Left, Right>
where
    Left: Generator,
{
    #[inline]
    pub(crate) fn new(left: Left, right: Right) -> Self {
        Self {
            left,
            right,
            last_left: None,
        }
    }
}

impl<Left, Right> Generator for Zip<Left, Right>
where
    Left: Generator,
    Right: Generator,
{
    type Output = (Left::Output, Right::Output);

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let right = &mut self.right;
        let left = &mut self.left;
        let last_left = &mut self.last_left;

        if last_left.is_some() {
            let mut output_result = ValueResult::Stop;
            match right.run(|rv| {
                if let Some(lv) = last_left.take() {
                    output_result = output((lv, rv));
                }
                ValueResult::Stop
            }) {
                GeneratorResult::Stopped => {
                    if last_left.is_some() || output_result == ValueResult::Stop {
                        return GeneratorResult::Stopped;
                    }
                }
                GeneratorResult::Complete => {
                    return GeneratorResult::Complete;
                }
            }
        }

        let mut right_result = GeneratorResult::Stopped;

        let left_result = left.run(|left_value| match right.next() {
            Ok(right_value) => output((left_value, right_value)),
            Err(x) => {
                *last_left = Some(left_value);
                right_result = x;
                ValueResult::Stop
            }
        });
        if left_result == GeneratorResult::Complete || right_result == GeneratorResult::Complete {
            GeneratorResult::Complete
        } else {
            GeneratorResult::Stopped
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::StoppingGen;
    use crate::{GeneratorExt, GeneratorResult, SliceGenerator};

    fn do_zip(left: &[i32], right: &[i32]) -> (Vec<(i32, i32)>, GeneratorResult) {
        let mut output: Vec<(i32, i32)> = Vec::new();
        let result = Zip::new(SliceGenerator::new(left), SliceGenerator::new(right))
            .for_each(|(a, b)| output.push((*a, *b)));
        (output, result)
    }

    fn do_iter_zip(left: &[i32], right: &[i32]) -> Vec<(i32, i32)> {
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<(i32, i32)>>()
    }

    #[test]
    fn same_length() {
        let data = [1, 2, 3, 4];
        let (output, result) = do_zip(&data, &data);
        let expected = do_iter_zip(&data, &data);

        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, expected);
    }

    #[test]
    fn shorter_left_side() {
        let left = [1, 2, 3];
        let right = [1, 2, 3, 4];
        let (output, result) = do_zip(&left, &right);
        let expected = do_iter_zip(&left, &right);

        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, expected);
    }

    #[test]
    fn shorter_right_side() {
        let right = [1, 2, 3];
        let left = [1, 2, 3, 4];
        let (output, result) = do_zip(&left, &right);
        let expected = do_iter_zip(&left, &right);

        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, expected);
    }

    #[test]
    fn spuriously_stopped_left() {
        let data = [1, 2, 3];
        for x in 0..3 {
            let left = StoppingGen::new(x, &data);
            let mut gen = left.zip(SliceGenerator::new(&data));
            let mut output: Vec<(i32, i32)> = Vec::new();
            let result = gen.for_each(|(&a, &b)| output.push((a, b)));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|(&a, &b)| output.push((a, b)));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [(1, 1), (2, 2), (3, 3)]);
        }
    }

    #[test]
    fn spuriously_stopped_right() {
        let data = [1, 2, 3];
        for x in 0..3 {
            let right = StoppingGen::new(x, &data);
            let mut gen = SliceGenerator::new(&data).zip(right);
            let mut output: Vec<(i32, i32)> = Vec::new();
            let result = gen.for_each(|(&a, &b)| output.push((a, b)));
            assert_eq!(result, GeneratorResult::Stopped);
            let result = gen.for_each(|(&a, &b)| output.push((a, b)));
            assert_eq!(result, GeneratorResult::Complete);
            assert_eq!(output, [(1, 1), (2, 2), (3, 3)]);
        }
    }
}

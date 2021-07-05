use crate::{Generator, GeneratorResult, ValueResult};

/// Zip two generators. See [`.zip()`](crate::GeneratorExt::zip) for details.
pub struct Zip<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left, Right> Zip<Left, Right> {
    #[inline]
    pub(crate) fn new(left: Left, right: Right) -> Self {
        Self { left, right }
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
        let mut right_result = GeneratorResult::Stopped;

        let left_result = left.run(|left_value| {
            let mut right_value = None;
            right_result = right.run(|rv| {
                right_value = Some(rv);
                ValueResult::Stop
            });

            if let Some(right_value) = right_value {
                output((left_value, right_value))
            } else {
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
    use crate::{GeneratorExt, GeneratorResult, SliceGenerator};

    fn do_zip(left: &[i32], right: &[i32]) -> (Vec<(i32, i32)>, GeneratorResult) {
        let mut output: Vec<(i32, i32)> = Vec::new();
        let result = Zip::new(SliceGenerator::new(&left), SliceGenerator::new(&right))
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
}

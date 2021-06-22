
use crate::{InputStage, InputOutputStage};

/// A stage that removes repeated duplicates
/// See [`.dedup()`](crate::StageExt::dedup) for more information.
pub struct Dedup<T> {
    last_forwarded: Option<T>,
}

impl<T> Dedup<T>
where
    T: PartialEq + Clone {
    pub fn new() -> Self {
        Self {
            last_forwarded: None
        }
    }

    fn should_forward(&mut self, value: &T) -> bool {
        if self.last_forwarded.as_ref() != Some(value) {
            self.last_forwarded = Some(value.clone());
            true
        }
        else {
            false
        }
    }
}

impl<T> InputStage for Dedup<T>
where
    T: PartialEq + Clone
{
    type Input = T;

    fn process(&mut self, value: Self::Input) -> bool {
        self.should_forward(&value);
        true
    }
}

impl<T> InputOutputStage for Dedup<T>
where
    T: PartialEq + Clone
{
    type Output = T;

    fn process_and_then(&mut self, value: Self::Input, next: &mut dyn InputStage<Input=Self::Output>) -> bool {
        if self.should_forward(&value) {
            next.process(value)
        }
        else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StageExt;
    use super::*;
    #[test]
    fn dedup_integers() {
        let mut values = Vec::<i32>::new();
        let mut dedup = Dedup::<i32>::new().end(|x| {
            values.push(x);
            true
        });

        assert!(dedup.process(1));
        assert!(dedup.process(1));
        assert!(dedup.process(2));
        assert!(dedup.process(2));
        assert!(dedup.process(3));
        assert!(dedup.process(1));

        assert_eq!(values, [1,2,3,1]);

    }
}
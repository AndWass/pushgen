pub mod structs;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum ValueResult {
    Stop,
    MoreValues,
}

impl From<bool> for ValueResult {
    fn from(value: bool) -> Self {
        if !value {
            Self::Stop
        }
        else {
            Self::MoreValues
        }
    }
}

/// A value consumer-emitter provides per-value handling, taking values of one type and emitting
/// values of possibly another type.
///
/// ## Example
///
/// This implements a very basic numeric filter.
/// ```
/// use pipe_chan::value::ConsumerEmitter;
/// use pipe_chan::ValueResult;
/// struct LessThan(i32);
/// impl ConsumerEmitter for LessThan {
///     type Input = i32;
///     type Output = i32;
///     fn consume_and_emit(&mut self,value: Self::Input,mut output: impl FnMut(Self::Output) -> ValueResult) -> ValueResult {
///         if value < self.0 {
///             output(value)
///         }
///         else {
///             ValueResult::MoreValues
///         }
///     }
/// }
/// ```
pub trait ConsumerEmitter {
    /// The consumed type
    type Input;
    /// The emitted type
    type Output;
    /// Receive a value, potentially do
    fn consume_and_emit(&mut self, value: Self::Input, output: impl FnMut(Self::Output) -> ValueResult) -> ValueResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_result_from_bool() {
        let x: ValueResult = false.into();
        assert_eq!(x, ValueResult::Stop);

        let x: ValueResult = true.into();
        assert_eq!(x, ValueResult::MoreValues);
    }
}
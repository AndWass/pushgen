/// Value-consumption result.
///
/// Value-consumers can either request more values from a generator, or for a generator to stop
/// generating values.
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum ValueResult {
    /// Request that a generator stop generating values.
    Stop,
    /// Request more values from a generator.
    MoreValues,
}

impl From<bool> for ValueResult {
    #[inline]
    fn from(value: bool) -> Self {
        if !value {
            Self::Stop
        } else {
            Self::MoreValues
        }
    }
}

/// The result of generator runs.
///
/// A run can either run to completion, and no new values will
/// be produced, or it can be stopped. In case it is stopped there might be more values available
/// that can be obtained by calling [`Generator::run`](crate::Generator::run) again.
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum GeneratorResult {
    /// Returned from `Generator::run` when the generator was stopped because the `output` function
    /// returned `ValueResult::Stop`
    Stopped,
    /// Returned from `Generator::run` when the generator has sent all values to the `output` function.
    /// When this has been returned the generator will never generate more values again.
    Complete,
}

impl From<bool> for GeneratorResult {
    #[inline]
    fn from(b: bool) -> Self {
        if !b {
            Self::Stopped
        } else {
            Self::Complete
        }
    }
}
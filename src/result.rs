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

/// The result of a reduction.
///
/// A reduction can either be partial, producing an intermediate value, or complete. Partial
/// reductions can for instance be created when trying to reduce a spuriously stopping generator.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Reduction<T>
{
    Complete(T),
    Partial(T),
}

impl<T> Reduction<T>
{
    /// Check if the reduction is complete.
    #[inline]
    pub fn is_complete(&self) -> bool {
        match self {
            Reduction::Complete(_) => true,
            _ => false,
        }
    }

    /// Check if the reduction is partial.
    #[inline]
    pub fn is_partial(&self) -> bool {
        match self {
            Reduction::Partial(_) => true,
            _ => false,
        }
    }

    /// Get the underlying value, no matter if it's complete or partial.
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Reduction::Complete(x) => x,
            Reduction::Partial(x) => x
        }
    }
}

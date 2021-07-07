use crate::{Generator, GeneratorResult, ValueResult};

/// Implements a mapped generator. See [`.map()`](crate::GeneratorExt::map) for details.
pub struct FilterMap<Gen, Func> {
    source: Gen,
    transform: Func,
}

impl<Gen, Func, Out> FilterMap<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Option<Out>,
{
    #[inline]
    pub fn new(source: Gen, transform: Func) -> Self {
        Self { source, transform }
    }
}

impl<Gen, Func, Out> Generator for FilterMap<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Option<Out>,
{
    type Output = Out;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let (source, transform) = (&mut self.source, &mut self.transform);
        source.run(move |x| {
            if let Some(x) = transform(x) {
                output(x)
            } else {
                ValueResult::MoreValues
            }
        })
    }
}

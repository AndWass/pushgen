use crate::{Generator, GeneratorResult, ValueResult};

/// A generator adaptor that holds internal state and creates a new generator. See [`scan()`] for details.
///
/// [`scan()`]: crate::GeneratorExt::scan
#[derive(Clone)]
pub struct Scan<Src, State, F> {
    source: Src,
    state: State,
    func: F,
}

impl<Src, State, F> Scan<Src, State, F> {
    pub(crate) fn new(source: Src, state: State, func: F) -> Self {
        Self {
            source,
            state,
            func,
        }
    }
}

impl<Src, State, F, B> Generator for Scan<Src, State, F>
where
    Src: Generator,
    F: FnMut(&mut State, Src::Output) -> Option<B>,
{
    type Output = B;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        let (state, func) = (&mut self.state, &mut self.func);
        self.source.run(|x| match func(state, x) {
            Some(value) => output(value),
            None => ValueResult::Stop,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{GeneratorExt, GeneratorResult, SliceGenerator};

    #[test]
    fn iter_scan() {
        let a = [1, 2, 3, 4];
        let mut iter = a.iter().scan(2, |st, value| {
            *st -= 1;
            if *st == 0 {
                None
            } else {
                Some(value)
            }
        });
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));

        let mut gen = SliceGenerator::new(&a).scan(2, |st, value| {
            *st -= 1;
            if *st == 0 {
                None
            } else {
                Some(value)
            }
        });

        assert_eq!(gen.next(), Ok(&1));
        assert_eq!(gen.next(), Err(GeneratorResult::Stopped));
        assert_eq!(gen.next(), Ok(&3));
        assert_eq!(gen.next(), Ok(&4));
        assert_eq!(gen.next(), Err(GeneratorResult::Complete))
    }
}

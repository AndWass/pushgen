use core::hint;

#[inline(always)]
pub unsafe fn unwrap_unchecked<T>(option: Option<T>) -> T {
    match option {
        Some(val) => val,
        // unlike unreachable!() which will panic on reaching, unreachable_unchecked
        // has no effect on the generated code and it will be UB if it is actually reached.
        None => hint::unreachable_unchecked(),
    }
}

/// Implements this functionality as a struct instead of a function taking Option<T>
/// to make sure that we won't accidentally treats a None as a Some and avoids the
/// unsafe fn altogether.
#[derive(Clone)]
pub struct InplaceUpdatable<T> {
    // Using Option here is necessary for integrity during unwind,
    // which is the only moment where inner is set to None.
    //
    // In every other cases, it is set to `Some(value)`.
    inner: Option<T>,
}
impl<T> InplaceUpdatable<T> {
    #[inline(always)]
    pub const fn new(value: T) -> InplaceUpdatable<T> {
        InplaceUpdatable { inner: Some(value) }
    }

    #[inline(always)]
    pub fn update(&mut self, updater: impl FnOnce(T) -> T) {
        // take self.inner to ensure nothing will be dropped here during unwind,
        // if updater ever panic
        let new_val = updater(unsafe { unwrap_unchecked(self.inner.take()) });
        self.inner = Some(new_val);
    }

    #[inline(always)]
    pub fn get_inner(self) -> T {
        unsafe { unwrap_unchecked(self.inner) }
    }
}

#[inline(always)]
pub fn set_some<T>(option: &mut Option<T>, value: T) -> &mut T {
    *option = Some(value);
    unsafe { unwrap_unchecked(option.as_mut()) }
}

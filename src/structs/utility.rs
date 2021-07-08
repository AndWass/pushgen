use core::hint;

#[allow(dead_code)]
pub unsafe fn unwrap_unchecked<T>(option: Option<T>) -> T {
    match option {
        Some(val) => val,
        // unlike unreachable!() which will panic on reaching, unreachable_unchecked
        // has no effect on the generated code and it will be UB if it is actually reached.
        None => hint::unreachable_unchecked(),
    }
}

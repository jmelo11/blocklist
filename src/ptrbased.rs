use std::ptr::NonNull;

/// # PtrBased
/// A trait for types that can be used with pointers.
pub trait PtrBased {
    type Item;

    fn begin(&self) -> Option<NonNull<Self::Item>>;

    fn end(&self) -> Option<NonNull<Self::Item>>;

    fn next(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>>;

    fn prev(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>>;
}

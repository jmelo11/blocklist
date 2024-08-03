use std::ptr::NonNull;

/// # PtrBased
/// A trait for types that can be used with pointers.
pub trait PtrBased {
    type Item;

    fn begin(&self) -> Option<NonNull<Self::Item>>;

    fn end(&self) -> Option<NonNull<Self::Item>>;

    fn next(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>>;

    fn prev(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>>;

    fn distance(&self, first: NonNull<Self::Item>, last: NonNull<Self::Item>) -> usize {
        let mut count = 0;
        let mut ptr = first;
        while ptr != last {
            count += 1;
            ptr = self.next(ptr).unwrap();
        }
        count
    }
}

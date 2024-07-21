use std::{mem::MaybeUninit, ptr::NonNull};

use crate::ptrbased::PtrBased;

pub struct ArrayLike<T, const CAP: usize> {
    data: [MaybeUninit<T>; CAP],
    current_ptr: Option<NonNull<T>>,
}

impl<T: Clone + Copy, const CAP: usize> ArrayLike<T, CAP> {
    pub fn new() -> Self {
        ArrayLike {
            data: [const { MaybeUninit::uninit() }; CAP],
            current_ptr: None,
        }
    }

    /// Helper function to initialize the current pointer.
    fn init(&mut self) {
        self.current_ptr = self.begin();
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.data.iter().map(|x| unsafe { x.assume_init() })
    }

    /// Insert a value at the given index.
    pub fn insert(&mut self, index: usize, value: T) -> Option<()> {
        if index < CAP {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(index) as *mut T;
                ptr.write(value);
            }
            Some(())
        } else {
            None
        }
    }

    /// Insert a value at the given index without bounds checking.
    pub unsafe fn insert_unchecked(&mut self, index: usize, value: T) {
        let ptr = self.data.as_mut_ptr().add(index) as *mut T;
        ptr.write(value);
    }

    /// Try to push a value into the block.
    pub fn try_push(&mut self, value: T) -> Option<()> {
        if self.current_ptr.is_none() {
            self.init();
        }
        if self.current_ptr >= self.end() {
            None
        } else {
            unsafe {
                self.current_ptr.unwrap().as_ptr().write(value);
                self.current_ptr = self.next(self.current_ptr.unwrap());
            }
            Some(())
        }
    }

    /// Push a value into the block and return a pointer to the pushed value.
    pub unsafe fn try_push_and_get_ptr(&mut self, value: T) -> Option<NonNull<T>> {
        if self.current_ptr.is_none() {
            self.init();
        }
        if self.current_ptr >= self.end() {
            None
        } else {
            self.current_ptr.unwrap().as_ptr().write(value);
            let ptr = self.current_ptr.unwrap();
            self.current_ptr = self.next(self.current_ptr.unwrap());
            Some(ptr)
        }
    }
}

impl<T, const CAP: usize> Drop for ArrayLike<T, CAP> {
    fn drop(&mut self) {
        for i in 0..CAP {
            unsafe {
                self.data.as_mut_ptr().add(i).drop_in_place();
            }
        }
    }
}

impl<T, const CAP: usize> PtrBased for ArrayLike<T, CAP> {
    type Item = T;

    fn begin(&self) -> Option<NonNull<Self::Item>> {
        let ptr = self.data.as_ptr() as *const T;
        NonNull::new(ptr as *mut T)
    }

    fn end(&self) -> Option<NonNull<Self::Item>> {
        let ptr = unsafe { self.data.as_ptr().add(CAP) } as *const T;
        NonNull::new(ptr as *mut T)
    }

    fn next(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>> {
        if ptr > self.end().unwrap() {
            None
        } else {
            NonNull::new(unsafe { ptr.as_ptr().add(1) })
        }
    }

    fn prev(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>> {
        if ptr < self.begin().unwrap() {
            None
        } else {
            let ptr = ptr.as_ptr() as *const T;
            NonNull::new(unsafe { ptr.sub(1) as *mut T })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_array_like_try_push_panic() {
        let mut list: ArrayLike<i32, 1> = ArrayLike::new();
        for i in 0..2 {
            list.try_push(i).unwrap();
        }
    }

    #[test]
    fn test_array_like_new() {
        let list: ArrayLike<i32, 10000> = ArrayLike::new();
        assert_eq!(list.iter().count(), 10000);
    }

    #[test]
    fn test_array_like_begin() {
        let list: ArrayLike<i32, 1> = ArrayLike::new();
        let begin = list.begin().unwrap();
        let next = list.next(begin).unwrap();
        assert_eq!(list.end().unwrap(), next);

        let list: ArrayLike<i32, 2> = ArrayLike::new();
        let begin = list.begin().unwrap();
        let next = list.next(begin).unwrap();
        let next = list.next(next).unwrap();
        assert_eq!(list.end().unwrap(), next);

        let list: ArrayLike<i32, 3> = ArrayLike::new();
        let begin = list.begin().unwrap();
        let next = list.next(begin).unwrap();
        let next = list.next(next).unwrap();
        let next = list.next(next).unwrap();
        assert_eq!(list.end().unwrap(), next);
    }

    #[test]
    fn test_array_like_next() {
        let list: ArrayLike<i32, 10000> = ArrayLike::new();
        let mut begin = list.begin().unwrap();
        for i in 0..10000 {
            unsafe {
                begin.write(i);
                begin = list.next(begin).unwrap();
            }
        }
    }

    #[test]
    fn test_array_like_prev() {
        let list: ArrayLike<i32, 10000> = ArrayLike::new();
        let mut end = list.end().unwrap();
        for i in 0..10000 {
            unsafe {
                end.write(i);
                end = list.prev(end).unwrap();
            }
        }
    }

    #[test]
    fn test_array_like_insert() {
        let mut list: ArrayLike<i32, 10000> = ArrayLike::new();
        for i in 0..10000 {
            list.insert(i, i as i32);
        }
    }

    #[test]
    fn test_array_like_try_push() {
        let mut list: ArrayLike<i32, 100> = ArrayLike::new();
        for i in 0..100 {
            list.try_push(i).unwrap();
        }
    }
}

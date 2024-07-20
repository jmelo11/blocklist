use std::{mem::MaybeUninit, ptr::NonNull};

use crate::ptrbased::PtrBased;

pub struct DataBlock2<T, const CAP: usize> {
    data: [MaybeUninit<T>; CAP],
    current_ptr: NonNull<T>,
}

impl<T: Clone + Copy, const CAP: usize> DataBlock2<T, CAP> {
    pub fn new() -> Self {
        let mut data = DataBlock2 {
            data: [const { MaybeUninit::uninit() }; CAP],
            current_ptr: NonNull::dangling(),
        };
        data.init();
        data
    }

    /// Helper function to initialize the current pointer.
    fn init(&mut self) {
        self.current_ptr = self.begin().unwrap();
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
        if self.current_ptr == self.end().unwrap() {
            None
        } else {
            unsafe {
                self.current_ptr.as_ptr().write(value);
                self.current_ptr = self.next(self.current_ptr).unwrap();
            }
            Some(())
        }
    }

    /// Push a value into the block and return a pointer to the pushed value.
    pub unsafe fn try_push_and_get_ptr(&mut self, value: T) -> Option<NonNull<T>> {
        if self.current_ptr == self.end().unwrap() {
            None
        } else {
            self.current_ptr.as_ptr().write(value);
            let ptr = self.current_ptr;
            self.current_ptr = self.next(self.current_ptr).unwrap();
            Some(ptr)
        }
    }
}

impl<T, const CAP: usize> Drop for DataBlock2<T, CAP> {
    fn drop(&mut self) {
        for i in 0..CAP {
            unsafe {
                self.data.as_mut_ptr().add(i).drop_in_place();
            }
        }
    }
}

impl<T, const CAP: usize> PtrBased for DataBlock2<T, CAP> {
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
        if ptr >= self.end().unwrap() {
            None
        } else {
            let ptr = ptr.as_ptr() as *const T;
            NonNull::new(unsafe { ptr.add(1) as *mut T })
        }
    }

    fn prev(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>> {
        if ptr <= self.begin().unwrap() {
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
    fn test_data_block2() {
        let list: DataBlock2<i32, 10000> = DataBlock2::new();
        let mut begin = list.begin().unwrap();
        for i in 0..10000 {
            unsafe {
                begin.write(i);
                begin = list.next(begin).unwrap();
            }
        }
    }
}

use std::ptr::NonNull;

use crate::{
    arraylike::ArrayLike,
    linkedlist::{LinkedList, Node},
    ptrbased::PtrBased,
};

pub struct SmallObjectPool<T, const CAP: usize> {
    data: LinkedList<ArrayLike<T, CAP>>,
    current_block: NonNull<Node<ArrayLike<T, CAP>>>,
    last_block: NonNull<Node<ArrayLike<T, CAP>>>,
    next_space: NonNull<T>,
    last_space: NonNull<T>,
    marked_block: NonNull<Node<ArrayLike<T, CAP>>>,
    marked_space: NonNull<T>,
}

impl<T> PtrBased for Vec<T> {
    type Item = T;

    fn begin(&self) -> Option<NonNull<Self::Item>> {
        if self.is_empty() {
            None
        } else {
            let ptr = self.as_ptr();
            unsafe { Some(NonNull::new_unchecked(ptr as *mut T)) }
        }
    }

    fn end(&self) -> Option<NonNull<Self::Item>> {
        if self.is_empty() {
            None
        } else {
            let ptr = unsafe { self.as_ptr().add(self.len()) };
            unsafe { Some(NonNull::new_unchecked(ptr as *mut T)) }
        }
    }

    fn next(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>> {
        if ptr == self.end().unwrap() {
            None
        } else {
            let ptr = ptr.as_ptr() as *const T;
            unsafe { Some(NonNull::new_unchecked(ptr.add(1) as *mut T)) }
        }
    }

    fn prev(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>> {
        if ptr == self.begin().unwrap() {
            None
        } else {
            let ptr = ptr.as_ptr() as *const T;
            unsafe { Some(NonNull::new_unchecked(ptr.sub(1) as *mut T)) }
        }
    }
}

impl<T: Clone + Copy, const CAP: usize> SmallObjectPool<T, CAP> {
    pub fn new() -> Self {
        let mut data = LinkedList::new();
        data.push_back(ArrayLike::new());
        let mut sop = SmallObjectPool {
            data: data,
            current_block: NonNull::dangling(),
            last_block: NonNull::dangling(),
            next_space: NonNull::dangling(),
            last_space: NonNull::dangling(),
            marked_block: NonNull::dangling(),
            marked_space: NonNull::dangling(),
        };
        sop.init();
        sop
    }

    fn init(&mut self) {
        self.current_block = self.data.begin().unwrap();
        self.marked_block = self.current_block;
        self.last_block = self.data.end().unwrap();
        unsafe {
            self.next_space = self.current_block.as_ref().inner().begin().unwrap();
            self.last_space = self.current_block.as_ref().inner().end().unwrap();
            self.marked_space = self.next_space;
        }
    }

    fn new_block(&mut self) {
        self.data.push_back(ArrayLike::new());
        self.last_block = self.data.end().unwrap();
        unsafe {
            self.current_block = self.last_block;
            self.next_space = self.current_block.as_ref().inner().begin().unwrap();
            self.last_space = self.current_block.as_ref().inner().end().unwrap();
        }
    }

    fn next_block(&mut self) {
        if self.current_block == self.last_block {
            self.new_block();
        } else {
            unsafe {
                self.current_block = self.data.next(self.current_block).unwrap();
                self.next_space = self.current_block.as_ref().inner().begin().unwrap();
                self.last_space = self.current_block.as_ref().inner().end().unwrap();
            }
        }
    }

    pub fn rewind(&mut self) {
        self.current_block = self.data.begin().unwrap();
        unsafe {
            self.next_space = self.current_block.as_ref().inner().begin().unwrap();
            self.last_space = self.current_block.as_ref().inner().end().unwrap();
        }
    }

    pub fn mark(&mut self) {
        self.marked_block = self.current_block;
        self.marked_space = self.next_space;
    }

    pub fn push(&mut self, value: T) {
        unsafe {
            if self.next_space == self.last_space {
                self.next_block();
            }
            self.next_space.as_ptr().write(value);
            self.next_space = self
                .current_block
                .as_ref()
                .inner()
                .next(self.next_space)
                .unwrap();
        }
    }
}

impl<T, const CAP: usize> Drop for SmallObjectPool<T, CAP> {
    fn drop(&mut self) {
        let mut current = self.data.begin();
        while let Some(block) = current {
            unsafe {
                block.drop_in_place();
            }
            let next = self.data.next(block);
            match next {
                Some(next_block) => {
                    current = Some(next_block);
                }
                None => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_object_pool() {
        let mut sop = SmallObjectPool::<u32, 4>::new();
        for i in 0..8 {
            sop.push(i);
        }
    }
}

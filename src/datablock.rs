use std::{
    alloc::{self, Layout},
    mem::MaybeUninit,
    ptr::NonNull,
};

pub struct DataBlock<T, const CAP: usize> {
    data: [MaybeUninit<T>; CAP],
    next_slot: usize,
    marked_slot: Option<usize>,
}

impl<T: Clone + Copy, const CAP: usize> DataBlock<T, CAP> {
    pub fn new() -> Self {
        DataBlock {
            data: [MaybeUninit::uninit(); CAP],
            next_slot: 0,
            marked_slot: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.next_slot == 0
    }

    pub fn clear(&mut self) {
        self.next_slot = 0;
        self.marked_slot = None;
    }

    pub fn clear_after_mark(&mut self) {
        if let Some(slot) = self.marked_slot {
            self.next_slot = slot;
        }
    }

    pub fn mark_slot(&mut self) {
        self.marked_slot = Some(self.next_slot);
    }

    pub fn rewind_to_front(&mut self) {
        self.next_slot = 0;
        self.marked_slot = None;
    }

    pub fn rewind_to_mark(&mut self) {
        if let Some(slot) = self.marked_slot {
            self.next_slot = slot;
        } else {
            self.next_slot = 0;
        }
    }

    /// Try to push a value into the block in the next slot. Values might be overwritten if rewind is
    /// called.
    pub fn try_push(&mut self, value: T) -> Option<()> {
        if self.next_slot < CAP {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(self.next_slot) as *mut T;
                ptr.write(value);
            }
            self.next_slot += 1;
            Some(())
        } else {
            None
        }
    }

    pub fn push_to_ptr(&mut self, value: T) -> Option<*mut T> {
        if self.next_slot < CAP {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(self.next_slot) as *mut T;
                ptr.write(value);
                self.next_slot += 1;
                Some(ptr)
            }
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.data
            .iter()
            .take(self.next_slot)
            .map(|x| unsafe { x.assume_init() })
    }

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
}

impl<T, const CAP: usize> Drop for DataBlock<T, CAP> {
    fn drop(&mut self) {
        for i in 0..self.next_slot {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(i) as *mut T;
                ptr.drop_in_place();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_block() {
        let mut block = DataBlock::<i32, 4>::new();
        assert!(block.is_empty());
        block.try_push(1).unwrap();
        block.try_push(2).unwrap();
        block.try_push(3).unwrap();
        block.try_push(4).unwrap();
        assert_eq!(block.iter().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_data_block_mark() {
        let mut block = DataBlock::<i32, 4>::new();
        block.try_push(1).unwrap();
        block.try_push(2).unwrap();
        block.try_push(3).unwrap();
        block.try_push(4).unwrap();
        let r = block.try_push(5);
        assert!(r.is_none());
    }
}

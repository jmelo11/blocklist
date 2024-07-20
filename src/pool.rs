use std::ptr::NonNull;

use crate::{
    datablock::DataBlock,
    linkedlist::{LinkedList, Node},
};

/// # Pool
/// A pool of blocks that can hold up to `CAP` elements.
/// The pool is implemented as a linked list of blocks.
pub struct Pool<T: Clone + Copy, const CAP: usize> {
    data: LinkedList<DataBlock<T, CAP>>,
    marked_block: Option<NonNull<Node<DataBlock<T, CAP>>>>,
    current_block: Option<NonNull<Node<DataBlock<T, CAP>>>>,
}

impl<'a, T: Clone + Copy, const CAP: usize> Pool<T, CAP> {
    pub fn new() -> Self {
        let data = LinkedList::new();
        Pool {
            data: data,
            marked_block: None,
            current_block: None,
        }
    }

    /// Create a new block and set it as the current block.
    fn new_block(&'a mut self) {
        self.data.push_back(DataBlock::new());
        self.current_block = self.data.tail_ptr();
    }

    /// Mark the current position.
    pub fn mark(&mut self) {
        self.marked_block = self.current_block;
        match self.marked_block {
            Some(mut block) => unsafe {
                let marker = block.as_mut();
                marker.data().mark_slot();
            },
            None => {}
        }
    }

    /// Clear the blocks after the current position.
    pub fn rewind_to_front(&mut self) {
        self.data.iter_mut().for_each(|x| x.rewind_to_front());
    }

    /// Clear the blocks after the current position.
    pub fn rewind_to_mark(&mut self) {
        if let Some(mut block) = self.marked_block {
            unsafe {
                block.as_mut().data().rewind_to_mark();
                while let Some(next_block) = block.as_mut().next_ptr() {
                    block = next_block;
                    block.as_mut().data().rewind_to_front();
                }
            }
            self.current_block = self.marked_block;
        }
    }

    unsafe fn add_and_push(&mut self, value: T) {
        self.new_block();
        self.current_block
            .unwrap()
            .as_mut()
            .data()
            .try_push(value)
            .unwrap();
    }

    /// Push a value into the pool.
    pub fn push(&mut self, value: T) {
        match self.current_block {
            Some(mut block) => unsafe {
                let marker = block.as_mut();
                if let None = marker.data().try_push(value) {
                    match self.current_block.unwrap().as_mut().next() {
                        Some(next_block) => {
                            self.current_block = self.current_block.unwrap().as_mut().next_ptr();
                            next_block.data().try_push(value).unwrap();
                        }
                        None => {
                            self.add_and_push(value);
                        }
                    }
                }
            },
            None => unsafe {
                self.add_and_push(value);
            },
        }
    }

    pub fn iter(&self) -> crate::linkedlist::Iter<DataBlock<T, CAP>> {
        self.data.iter()
    }
}

pub struct Pool2<T: Clone + Copy, const CAP: usize> {
    data: LinkedList<DataBlock<T, CAP>>,
    marked_block: Option<NonNull<Node<DataBlock<T, CAP>>>>,
    current_block: Option<NonNull<Node<DataBlock<T, CAP>>>>,
    current_slot: usize,
    marked_slot: Option<usize>,
}

impl<'a, T: Clone + Copy, const CAP: usize> Pool2<T, CAP> {
    pub fn new() -> Self {
        let data = LinkedList::new();
        Pool2 {
            data: data,
            marked_block: None,
            current_block: None,
            current_slot: 0,
            marked_slot: None,
        }
    }

    /// Create a new block and set it as the current block.
    fn new_block(&'a mut self) {
        self.data.push_back(DataBlock::new());
        self.current_block = self.data.tail_ptr();
        self.current_slot = 0;
    }

    /// Mark the current position.
    pub fn mark(&mut self) {
        self.marked_block = self.current_block;
        self.marked_slot = Some(self.current_slot);
    }

    /// Clear the blocks after the current position.
    pub fn rewind_to_front(&mut self) {
        self.current_block = self.data.head_ptr();
        self.current_slot = 0;
    }

    /// Clear the blocks after the current position.
    pub fn rewind_to_mark(&mut self) {
        self.current_block = self.marked_block;
        self.current_slot = self.marked_slot.unwrap();
    }

    unsafe fn add_and_push(&mut self, value: T) {
        self.new_block();
        self.current_block
            .unwrap()
            .as_mut()
            .data()
            .insert(self.current_slot, value);
    }

    /// Push a value into the pool.
    pub fn push(&mut self, value: T) {
        match self.current_block {
            Some(mut block) => unsafe {
                let marker = block.as_mut();
                if self.current_slot < CAP {
                    marker.data().insert(self.current_slot, value);
                    self.current_slot += 1;
                } else {
                    match self.current_block.unwrap().as_mut().next() {
                        Some(next_block) => {
                            self.current_block = self.current_block.unwrap().as_mut().next_ptr();
                            self.current_slot = 0;
                            next_block.data().insert(self.current_slot, value);
                            self.current_slot += 1;
                        }
                        None => {
                            self.add_and_push(value);
                        }
                    }
                }
            },
            None => unsafe {
                self.add_and_push(value);
            },
        }
    }

    pub fn iter(&self) -> crate::linkedlist::Iter<DataBlock<T, CAP>> {
        self.data.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocklist_init() {
        let mut blocklist: Pool<f64, 1024> = Pool::new();
        for i in 0..100000 {
            blocklist.push(i as f64);
        }
    }

    #[test]
    fn test_push_and_print() {
        let mut blocklist: Pool2<i32, 4> = Pool2::new();
        blocklist.push(1);
        blocklist.push(2);
        blocklist.push(3);
        blocklist.push(4);
        blocklist.push(5);
        blocklist.push(6);
        blocklist.push(7);
        blocklist.push(8);
        blocklist.push(9);

        blocklist
            .iter()
            .for_each(|x| println!("{:?}", x.iter().collect::<Vec<_>>()));
    }

    #[test]
    fn test_push_and_print_struct() {
        #[derive(Debug, Copy, Clone)]
        #[allow(dead_code)]
        struct Test {
            a: i32,
            b: i32,
        }
        let mut blocklist: Pool<Test, 4> = Pool::new();
        blocklist.push(Test { a: 1, b: 2 });
        blocklist.push(Test { a: 3, b: 4 });

        // println!("{:?}", blocklist.iter().collect::<Vec<_>>());
        // blocklist
        //     .iter()
        //     .for_each(|x| println!("{:?}", x.iter().collect::<Vec<_>>()));
    }

    #[test]
    fn test_mark_and_rewind() {
        let mut blocklist: Pool<i32, 4> = Pool::new();
        blocklist.push(1); // [1]
        blocklist.push(2); // [1, 2]
        blocklist.push(3); // [1, 2, 3]
        blocklist.mark();
        blocklist.push(4); // [1, 2, 3, 4]
        blocklist.push(5); // [1, 2, 3, 4], [5]

        blocklist
            .iter()
            .for_each(|x| println!("{:?}", x.iter().collect::<Vec<_>>()));

        blocklist.rewind_to_mark();
        blocklist.push(6); // [1, 2, 3, 6], [5]
        blocklist.push(7); // [1, 2, 3, 6], [7]

        // blocklist.iter().for_each(|x| println!("{}", x));
        blocklist
            .iter()
            .for_each(|x| println!("{:?}", x.iter().collect::<Vec<_>>()));
    }

    // #[test]
    // fn test_rewind_to_front() {
    //     let mut blocklist: Pool<i32, 4> = Pool::new();
    //     blocklist.push(1);
    //     blocklist.push(2);
    //     blocklist.push(3);
    //     blocklist.mark();
    //     blocklist.push(4);
    //     blocklist.push(5);
    //     blocklist.rewind_to_front(true);
    //     blocklist.push(6);
    //     blocklist.push(7);
    //     blocklist.iter().for_each(|x| println!("{}", x));
    // }

    // #[test]
    // fn test_push_ptr() {
    //     let mut blocklist: Pool<i32, 4> = Pool::new();
    //     let ptr1 = blocklist.push_to_ptr(1);
    //     let ptr2 = blocklist.push_to_ptr(2);
    //     let ptr3 = blocklist.push_to_ptr(3);
    //     let ptr4 = blocklist.push_to_ptr(4);
    //     let ptr5 = blocklist.push_to_ptr(5);
    //     let ptr6 = blocklist.push_to_ptr(6);
    //     let ptr7 = blocklist.push_to_ptr(7);

    //     unsafe {
    //         println!("{}", *ptr1);
    //         println!("{}", *ptr2);
    //         println!("{}", *ptr3);
    //         println!("{}", *ptr4);
    //         println!("{}", *ptr5);
    //         println!("{}", *ptr6);
    //         println!("{}", *ptr7);
    //     }

    //     // rewind to front
    //     blocklist.rewind_to_front(false);

    //     blocklist.push(9);
    //     blocklist.push(9);
    //     blocklist.push(9);

    //     unsafe {
    //         println!("{}", *ptr1);
    //         println!("{}", *ptr2);
    //         println!("{}", *ptr3);
    //         println!("{}", *ptr7);
    //     }
    // }
}

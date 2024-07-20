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

use std::ptr::NonNull;

/// # Node
/// Basic node for a singly linked list.
pub struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Node {
            element,
            next: None,
            prev: None,
        }
    }

    pub fn data(&mut self) -> &mut T {
        &mut self.element
    }

    pub fn next(&self) -> Option<&mut Node<T>> {
        match self.next {
            Some(node) => Some(unsafe { &mut *node.as_ptr() }),
            None => None,
        }
    }

    pub fn next_ptr(&self) -> Option<NonNull<Node<T>>> {
        self.next
    }

    pub fn prev_ptr(&self) -> Option<NonNull<Node<T>>> {
        self.prev
    }

    pub fn prev(&self) -> Option<&mut Node<T>> {
        match self.prev {
            Some(node) => Some(unsafe { &mut *node.as_ptr() }),
            None => None,
        }
    }
}

/// # Linked List
/// Basic linked list.
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    unsafe fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            (*node.as_ptr()).next = self.head;
            (*node.as_ptr()).prev = None;
            let node = Some(node);

            match self.head {
                None => self.tail = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    pub fn push_front(&mut self, element: T) {
        let node = Box::new(Node::new(element));
        let node_ptr = NonNull::from(Box::leak(node));
        unsafe {
            self.push_front_node(node_ptr);
        }
    }

    unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`.
        unsafe {
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = self.tail;
            let node = Some(node);

            match self.tail {
                None => self.head = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(tail) => (*tail.as_ptr()).next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, element: T) {
        let node = Box::new(Node::new(element));
        let node_ptr = NonNull::from(Box::leak(node));
        unsafe {
            self.push_back_node(node_ptr);
        }
    }

    pub fn mut_head(&self) -> Option<&mut Node<T>> {
        unsafe { self.head.as_ref().map(|node| &mut *node.as_ptr()) }
    }

    pub fn mut_tail(&self) -> Option<&mut Node<T>> {
        unsafe { self.tail.as_ref().map(|node| &mut *node.as_ptr()) }
    }

    pub fn head_ptr(&self) -> Option<NonNull<Node<T>>> {
        self.head
    }

    pub fn tail_ptr(&self) -> Option<NonNull<Node<T>>> {
        self.tail
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head,
            len: self.len,
            marker: std::marker::PhantomData,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head,
            len: self.len,
            marker: std::marker::PhantomData,
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<NonNull<Node<T>>>,
    len: usize,
    marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let node = unsafe { self.next.unwrap().as_mut() };
        self.next = node.next;
        self.len -= 1;
        Some(&mut node.element)
    }
}

pub struct Iter<'a, T> {
    next: Option<NonNull<Node<T>>>,
    len: usize,
    marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let node = unsafe { self.next.unwrap().as_ref() };
        self.next = node.next;
        self.len -= 1;
        Some(&node.element)
    }
}

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

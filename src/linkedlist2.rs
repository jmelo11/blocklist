use std::ptr::NonNull;

use crate::ptrbased::PtrBased;

/// # Node
/// Basic node for a singly linked list.
pub struct Node2<T> {
    pub next: Option<NonNull<Node2<T>>>,
    pub prev: Option<NonNull<Node2<T>>>,
    pub data: T,
}

impl<T> Node2<T> {
    pub fn new(data: T) -> Self {
        Node2 {
            data,
            next: None,
            prev: None,
        }
    }

    pub fn inner(&self) -> &T {
        &self.data
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

/// # LinkedList
/// Basic linked list.
pub struct LinkedList2<T> {
    start: Option<NonNull<Node2<T>>>,
    end: Option<NonNull<Node2<T>>>,
}

impl<T> PtrBased for LinkedList2<T> {
    type Item = Node2<T>;

    fn begin(&self) -> Option<NonNull<Self::Item>> {
        self.start
    }

    fn end(&self) -> Option<NonNull<Self::Item>> {
        self.end
    }

    fn next(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>> {
        if ptr >= self.end.unwrap() {
            None
        } else {
            unsafe {
                match ptr.as_ref().next {
                    Some(next) => Some(next),
                    None => None,
                }
            }
        }
    }

    fn prev(&self, ptr: NonNull<Self::Item>) -> Option<NonNull<Self::Item>> {
        if ptr <= self.end.unwrap() {
            None
        } else {
            unsafe {
                match ptr.as_ref().prev {
                    Some(prev) => Some(prev),
                    None => None,
                }
            }
        }
    }
}

impl<T> LinkedList2<T> {
    pub fn new() -> Self {
        LinkedList2 {
            start: None,
            end: None,
        }
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node2::new(data));
        let mut new_node_ptr = NonNull::new(Box::into_raw(new_node)).unwrap();
        if let Some(mut end) = self.end {
            unsafe {
                new_node_ptr.as_mut().prev = Some(end);
                end.as_mut().next = Some(new_node_ptr);
            }
        } else {
            self.start = Some(new_node_ptr);
        }
        self.end = Some(new_node_ptr);
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node2::new(data));
        let mut new_node_ptr = NonNull::new(Box::into_raw(new_node)).unwrap();
        match self.start {
            Some(mut start) => unsafe {
                new_node_ptr.as_mut().next = Some(start);
                start.as_mut().prev = Some(new_node_ptr);
                self.start = Some(new_node_ptr);
            },
            None => {
                self.start = Some(new_node_ptr);
                self.end = Some(new_node_ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_linked_list2() {
        let mut list: LinkedList2<i32> = LinkedList2::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let begin = list.begin().unwrap();
        let end = list.end().unwrap();
        assert_eq!(unsafe { begin.as_ref().data }, 1);
        assert_eq!(unsafe { end.as_ref().data }, 3);
    }

    #[test]
    fn test_next() {
        let mut list: LinkedList2<i32> = LinkedList2::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let begin = list.begin().unwrap();
        let mut next = list.next(begin).unwrap();
        assert_eq!(unsafe { next.as_ref().data }, 2);
        next = list.next(next).unwrap();
        assert_eq!(unsafe { next.as_ref().data }, 3);
    }
}

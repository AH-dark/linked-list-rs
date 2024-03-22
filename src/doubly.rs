use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Node<T> {
    data: T,
    prev: RefCell<Option<Weak<Node<T>>>>,
    next: RefCell<Option<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<Self> {
        Rc::new(Node {
            data,
            prev: RefCell::new(None),
            next: RefCell::new(None),
        })
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

pub struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Weak<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, tail: None, length: 0 }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Node::new(elem);
        self.length += 1;
        match self.head.take() {
            Some(old_head) => {
                old_head.prev.borrow_mut().replace(Rc::downgrade(&new_node));
                new_node.next.borrow_mut().replace(old_head);
                self.head = Some(new_node);
            }
            None => {
                let weak_new_node = Rc::downgrade(&new_node);
                self.head = Some(new_node);
                self.tail = Some(weak_new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let res = self.head.take().and_then(|head_node| {
            match head_node.next.borrow_mut().take() {
                Some(next_node) => {
                    *next_node.prev.borrow_mut() = None;
                    self.head = Some(next_node);
                }
                None => {
                    self.tail = None;
                }
            }
            
            Some(Rc::try_unwrap(head_node).ok().unwrap().data)
        });
        
        if res.is_some() {
            self.length = self.length.saturating_sub(1); // Decrement length safely
        }
        
        res
    }

    pub fn push_back(&mut self, elem: T) {
        let new_node = Node::new(elem);
        self.length += 1;
        match self.tail.replace(Rc::downgrade(&new_node)) {
            Some(old_tail_weak) => {
                if let Some(old_tail) = old_tail_weak.upgrade() {
                    *old_tail.next.borrow_mut() = Some(new_node.clone());
                    *new_node.prev.borrow_mut() = Some(old_tail_weak);
                }
            }
            None => {
                self.head = Some(new_node.clone());
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_some() {
            self.length = self.length.saturating_sub(1); // Decrement length safely
        }
        
        let tail_weak = self.tail.take();
        let old_tail = match tail_weak.and_then(|weak| weak.upgrade()) {
            Some(node) => node,
            None => return None,
        };

        let prev_node = old_tail.prev.borrow_mut().take().and_then(|weak| weak.upgrade());
        match prev_node {
            Some(prev) => {
                *prev.next.borrow_mut() = None;
                self.tail = Some(Rc::downgrade(&prev));
            }
            None => {
                // If there's no previous node, it means the list had only one element.
                self.head = None;
            }
        }

        let res = Rc::try_unwrap(old_tail).ok().map(|node| node.data);
        if res.is_some() {
            self.length = self.length.saturating_sub(1); // Decrement length safely
        }
        
        res
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn push_and_pop_back() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn mixed_push_pop() {
        let mut list = LinkedList::new();

        list.push_front(1);
        list.push_back(2);
        list.push_front(3);
        list.push_back(4);

        // Expected list: 3 -> 1 -> 2 -> 4

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn length_after_operations() {
        let mut list = LinkedList::new();

        list.push_front(1);
        list.push_back(2);
        assert_eq!(list.len(), 2); // Ensure `len` method exists and works as expected

        list.pop_front();
        assert_eq!(list.len(), 1);

        list.pop_back();
        assert_eq!(list.len(), 0);
    }
}


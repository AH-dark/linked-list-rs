type OptionNode<T> = Option<Box<Node<T>>>;

/// The Node struct represents each element in the LinkedList.
struct Node<T> {
    data: T,
    next: OptionNode<T>,
}

/// LinkedList struct, which will use the Node struct for its elements.
pub struct LinkedList<T> {
    head: OptionNode<T>,
}

impl<T> LinkedList<T> {
    /// Constructs a new, empty LinkedList.
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    /// Pushes a new element onto the front of the list.
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            data: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /// Appends a new element onto the end of the list.
    pub fn append(&mut self, elem: T) {
        let new_node = Box::new(Node { data: elem, next: None });

        let mut cursor = &mut self.head;
        while let Some(ref mut next_node) = *cursor {
            cursor = &mut next_node.next;
        }

        *cursor = Some(new_node);
    }

    /// Removes and returns the element from the front of the list.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    /// Removes and returns the element from the end of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let mut cursor = &mut self.head;
        while cursor.as_ref()?.next.is_some() {
            cursor = &mut cursor.as_mut()?.next;
        }

        cursor.take().map(|node| node.data)
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(ref node) = *current {
            count += 1;
            current = &node.next;
        }
        count
    }

    /// Clears the list, removing all elements.
    pub fn clear(&mut self) {
        self.head = None;
    }

    /// Checks if the list is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self.head, None)
    }

    /// Returns an iterator over the list.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: match &self.head {
                None => None,
                Some(node) => Some(&node),
            },
        }
    }
}

/// Implementation of Debug trait to enable printing of the list for debugging purposes.
impl<T> std::fmt::Debug for LinkedList<T> where T: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = &self.head;
        while let Some(node) = current {
            write!(f, "{:?} -> ", node.data)?;
            current = &node.next;
        }
        write!(f, "End")
    }
}

/// An iterator over the elements of the linked list.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

/// Implementation of the Iterator trait for Iter.
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = match &node.next {
                None =>
                    None,
                Some(next_node) => Some(&next_node),
            };
            &node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the creation of a new LinkedList.
    #[test]
    fn test_new() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
    }

    // Test pushing elements onto the front of the list.
    #[test]
    fn test_push() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.len(), 3);
        assert_eq!(format!("{:?}", list), "3 -> 2 -> 1 -> End");
    }

    // Test popping elements from the front of the list.
    #[test]
    fn test_pop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    // Test appending elements to the end of the list.
    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.len(), 3);
        assert_eq!(format!("{:?}", list), "1 -> 2 -> 3 -> End");
    }

    // Test popping elements from the back of the list.
    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    // Test clearing the list.
    #[test]
    fn test_clear() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.clear();

        assert!(list.is_empty());
    }

    // Test iterating over the list.
    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}

/// Type alias for an optional boxed node, simplifying the type signature.
type OptionNode<T> = Option<Box<Node<T>>>;

/// A node in the singly linked list.
///
/// Each node holds its own data of generic type `T` and a pointer (optional) to the next node in the list.
#[derive(Clone)]
pub struct Node<T> {
    data: T,
    next: OptionNode<T>,
}

impl<T> Node<T> {
    /// Constructs a new `Node` instance encapsulating the given data, with no subsequent node.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be stored in the new node.
    fn new(data: T) -> Self {
        Node { data, next: None }
    }

    /// Provides a reference to the node's data.
    ///
    /// # Returns
    ///
    /// A reference to the data stored in the node.
    pub fn data(&self) -> &T {
        &self.data
    }
}

/// Represents a singly linked list with elements of type `T`.
///
/// The list maintains ownership of its nodes, and provides methods to manipulate
/// the list's structure, such as adding and removing elements.
pub struct LinkedList<T> {
    head: OptionNode<T>,
    length: usize,
}

impl<T> LinkedList<T> {
    /// Constructs a new, empty LinkedList.
    pub fn new() -> Self {
        LinkedList { head: None, length: 0 }
    }

    /// Inserts an element at the start of the list.
    ///
    /// # Arguments
    ///
    /// * `elem` - The element to be added to the list.
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            data: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.length += 1;
    }

    /// Appends an element to the end of the list.
    ///
    /// # Arguments
    ///
    /// * `elem` - The element to be appended to the list.
    pub fn append(&mut self, elem: T) {
        let new_node = Box::new(Node::new(elem));

        let mut cursor = &mut self.head;
        while let Some(ref mut next_node) = *cursor {
            cursor = &mut next_node.next;
        }

        *cursor = Some(new_node);
        self.length += 1;
    }

    /// Removes and returns the first element of the list, if it exists.
    ///
    /// # Returns
    ///
    /// The removed element, if the list was not empty.
    pub fn pop(&mut self) -> Option<T> {
        let res = self.head.take().map(|node| {
            self.head = node.next;
            node.data
        });

        if res.is_some() {
            self.length = self.length.saturating_sub(1);
        }

        res
    }

    /// Removes and returns the last element of the list, if it exists.
    ///
    /// # Returns
    ///
    /// The removed element, if the list was not empty.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let mut cursor = &mut self.head;
        while cursor.as_ref()?.next.is_some() {
            cursor = &mut cursor.as_mut()?.next;
        }

        let res = cursor.take().map(|node| node.data);
        if res.is_some() {
            let _ = self.length.saturating_sub(1);
        }

        res
    }

    /// Returns the current length of the list.
    ///
    /// # Returns
    ///
    /// The number of elements in the list.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Clears the list, removing all elements.
    pub fn clear(&mut self) {
        self.head = None;
        self.length = 0;
    }

    /// Checks if the list is empty.
    ///
    /// # Returns
    ///
    /// `true` if the list contains no elements, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Provides an iterator over the list's elements.
    ///
    /// # Returns
    ///
    /// An iterator that yields references to the elements in the list.
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

/// Iterator over the elements of a `LinkedList`.
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

    // Test the length of the list after operations.
    #[test]
    fn test_length_after_operations() {
        let mut list = LinkedList::new();

        list.push(1);
        list.append(2);
        assert_eq!(list.len(), 2);

        list.pop();
        assert_eq!(list.len(), 1);
    }
}

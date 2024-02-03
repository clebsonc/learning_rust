#![allow(dead_code)]
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type NodePointer = Option<Rc<RefCell<Node>>>;

pub struct Node {
    value: i32,
    next: NodePointer,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }
}

pub struct LinkedList {
    head: NodePointer,
    tail: NodePointer,
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}

impl LinkedList {
    /// Create a empty linked list with head and tail set as None.
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: i32) {
        let node_reference = Rc::new(RefCell::new(Node::new(value)));

        match self.tail.take() {
            Some(tail) => tail.borrow_mut().next = Some(node_reference.clone()),
            None => self.head = Some(node_reference.clone()),
        }
        self.tail = Some(node_reference);
    }

    pub fn push_front(&mut self, value: i32) {
        let node_reference = Rc::new(RefCell::new(Node::new(value)));

        // empty list, both head and tail point to first item.
        if self.head.is_none() {
            self.tail = Some(node_reference.clone());
        }
        // not empty, then make new node point to head.
        else {
            node_reference.borrow_mut().next = self.head.take();
        }
        self.head = Some(node_reference.clone());
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        if self.head.is_some() {
            let front = self.head.take().unwrap();
            self.head = front.borrow().next.clone();
            let val = front.borrow().value;
            if self.head.is_none() {
                self.tail = self.head.clone();
            }
            return Some(val);
        }
        None
    }
    
    pub fn print_list(&self) {
        let mut current = self.head.clone();
        print!("[");
        while let Some(node) = current {
            print!("{}", node.borrow().value);
            current = node.borrow().next.clone();
            if current.is_some() {
                print!(", ");
            }
        }
        println!("]");
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        while self.head.is_some() {
            let current = self.head.take().unwrap();
            self.head = current.borrow().next.clone();
        }
    }
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.clone();
        write!(f, "[")?;
        while let Some(node) = current {
            write!(f, "{} ", node.borrow().value)?;
            current = node.borrow().next.clone();
        }
        if current.is_some() {
            write!(f, ", ")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocating_single_node() {
        let n1 = Node {
            value: 10,
            next: None,
        };
        let n2 = Node::new(10);
        assert_eq!(n1.value, n2.value);
        assert_eq!(n1.next.is_none(), n2.next.is_none());
    }

    #[test]
    fn test_empty_linked_list() {
        let ll1 = LinkedList {
            head: None,
            tail: None,
        };
        let ll2 = LinkedList::new();
        assert_eq!(ll1.head.is_none(), ll1.tail.is_none());
        assert_eq!(ll2.head.is_none(), ll2.tail.is_none());
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        match &list.head {
            Some(node) => assert_eq!(node.borrow().value, 10),
            None => panic!("Head of list is None"),
        }

        let binding = list.head.as_ref().unwrap().borrow();
        let middle = binding.next.as_ref().unwrap().borrow();
        assert_eq!(middle.value, 20);

        let last = middle.next.as_ref().unwrap().borrow();
        assert_eq!(last.value, 30);

        match &list.tail {
            Some(node) => assert_eq!(node.borrow().value, 30),
            None => panic!("tail of list is None"),
        }
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::new();
        list.push_front(10);
        list.push_front(20);
        list.push_front(30);

        match &list.head {
            Some(node) => assert_eq!(node.borrow().value, 30),
            None => panic!("Head of list is None"),
        }

        let binding = list.head.as_ref().unwrap().borrow();
        let middle = binding.next.as_ref().unwrap().borrow();
        assert_eq!(middle.value, 20);

        let last = middle.next.as_ref().unwrap().borrow();
        assert_eq!(last.value, 10);

        match &list.tail {
            Some(node) => assert_eq!(node.borrow().value, 10),
            None => panic!("tail of list is None"),
        }
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::new();
        let values = [10, 20, 30];
        for i in values {
            list.push_back(i);
        }
        for i in values {
            assert_eq!(list.head.as_ref().unwrap().borrow().value, i);
            assert_eq!(list.tail.as_ref().unwrap().borrow().value, 30);

            let val = list.pop_front();
            assert_eq!(val, Some(i));
        }
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::default();
        let val = list.pop_back();
        assert!(val.is_none());

        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        let val = list.pop_back();
        assert_eq!(val, Some(30));
        // assert_eq!(val, Some(30));
    }
}

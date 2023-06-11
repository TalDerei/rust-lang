use crate::node::Node;
use std::rc::Rc;        // Allows multiple immutable "borrows" and enables multiple owners
use std::cell::RefCell; // Interior mutability

#[derive(Debug)]
pub struct LinkedList {
    pub head: Option<Rc<RefCell<Node>>>,
    pub tail: Option<Rc<RefCell<Node>>>,
    pub element: usize
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None, tail: None, element: 0
        }
    }

    pub fn insert_at_head(&mut self, item: u64) {
        let mut node = Node::new(item);
        let boxed_new = Rc::new(RefCell::new(node));
        self.head = Some(boxed_new.clone());
        self.tail = Some(boxed_new.clone());
    }

    pub fn len(&self) -> usize {
        self.element
    }
}

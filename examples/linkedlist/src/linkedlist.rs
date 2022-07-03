use crate::node::Node;

#[derive(Debug)]
pub struct LinkedList {
    pub head: Option<*mut Node>,
    pub tail: Option<*mut Node>,
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
        let n: *mut Node = &mut node;
        self.head = Some(n);
        self.tail = Some(n);
    }

    pub fn len(&self) -> usize {
        self.element
    }
}

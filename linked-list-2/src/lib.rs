// Wrapper struct for LinkedList that serves as public API!
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    // Constructor
    fn empty() -> LinkedList<T> {
        LinkedList { head: None }
    }

    fn push(&mut self, element: T) {
        // Temporarily replace self.head with None
        let old_head = std::mem::replace(&mut self.head, None);
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        let old_head = std::mem::replace(&mut self.head, None);
        match old_head {
            Some(n) => {
                self.head = n.next;
                Some(n.element)
            }
            None => None,
        }
    }

    fn peak(&self) -> Option<&T> {
        self.head.as_ref().map(|n | &n.element)
    }
}

struct Node<T> {
    element: T,
    next: Link<T>, // Box is an owned, heap allocated pointer
}

// type alias for 'Option'
type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::empty();
        list.push(1024);
        list.push(1025);
    }
}

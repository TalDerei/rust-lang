#[derive(Debug)]
pub struct Node {
    item: u64,
    next: u64
}

impl Node {
    pub fn new(item: u64) -> Node {
        Node {
            item,
            next: 0
        }
    }

    pub fn set_item(&mut self, item: u64) {
        self.item = item;
    }
}
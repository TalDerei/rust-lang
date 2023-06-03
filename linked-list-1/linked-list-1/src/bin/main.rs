// Import linkedlist namespace, use LinkedList struct inside the namespace
extern crate linkedlist;       
use linkedlist::LinkedList;     

fn main() {
    // Instantiate the linkedlist to create object
    // Node Type: Option<Rc<RefCell<Node>>>
    let mut my_list = LinkedList::new(); 
    my_list.insert_at_head(69);

    println!("The list: {:?}", &my_list);

    match my_list.head {
        Some(ref rc) => { 
            rc.borrow_mut().set_item(10);
        }
        None => (),
    }
    // `borrow_mut` represents the cell, and performs a borrow on the 
    // node at runtime and maintains a borrow counter. `RefCell` provides
    // interior mutability, `rc` enables multiple owners
    println!("The list: {:?}", &my_list);
}
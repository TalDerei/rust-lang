extern crate linkedlist;       // Importing linkedlist namespace
use linkedlist::LinkedList;     // Using LinkedList struct inside the namespace

fn main() {
    let mut my_list = LinkedList::new(); // Instantiate the linkedlist to create object
    my_list.insert_at_head(5);
    println!("The list: {:?}", unsafe{&*my_list.head.unwrap()});
}

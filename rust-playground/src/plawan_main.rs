// @plawanrath

// Rust can discover modules but Rust doesn't auto-discover subfolders in a flat layout. So you tell it where the files are
#[path = "plawan/starter.rs"]
mod starter;

#[path = "plawan/single_linked_list.rs"]
mod single_inked_list;

use single_inked_list::LinkedList;

#[path = "plawan/starter_test.rs"]
#[cfg(test)]
mod starter_test;

#[path = "plawan/single_linked_list_test.rs"]
#[cfg(test)]
mod single_inked_list_test;

fn main() {
    starter::run();
    // add your modules below
    //SingleLinkedList
    let mut list = LinkedList::new();
    list.insert_front(1);
    list.insert_front(2);
    println!("{:?}", list.to_vec());
    list.reverse();
    list.print();
    list.delete(1);
    println!("{:?}", list.to_vec());
}
#[derive(Debug)]
pub struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

impl<T: PartialEq + Copy> LinkedList<T> {
    /// create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    /// insert at the beginning of the list
    pub fn insert_front(&mut self, val: T) {
        let new_node = Box::new(
            ListNode {
                val,
                next: self.head.take(),
            }
        );
        self.head = Some(new_node);
    }

    ///Delete first occurance of a value
    pub fn delete(&mut self, val: T) -> bool {
        let mut current: *mut Option<Box<ListNode<T>>> = &mut self.head;

        // Where is node initialized?

        // It's not initialized beforehand — it's a binding created by pattern matching on the Option.

        // Step-by-step:
            // current is an &mut Option<Box<ListNode<T>>>.
            // while let Some(ref mut node) checks:
                // If current is Some(...)
                // If yes, it binds node as a mutable reference to the Box<ListNode<T>> inside.
            // Inside the loop:
                // You can now do node.val, node.next, etc.
                // You can also mutate the node because it’s a ref mut.
        unsafe {
            while let Some(node) = &mut *current {
                if node.val == val {
                    *current = node.next.take(); // remove node
                    return true;
                }
                current = &mut node.next as *mut _;
            }
        }
        false // this is same as doing return false;
    }

    /// reverse the list
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next_node = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next_node;
        }
        self.head = prev;
    }

    /// print the list contents
    pub fn print(&self)
    where 
        T: std::fmt::Display,
    {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = &node.next;
        }
        println!("None");
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push(node.val);
            current = &node.next;
        }
        result
    }
}
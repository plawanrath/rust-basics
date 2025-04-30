#[path = "single_linked_list.rs"]
mod single_linked_list;

#[cfg(test)]
mod tests {
    use single_linked_list::LinkedList;

    use super::single_linked_list; 

    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        list.insert_front(1);
        list.insert_front(2);
        list.insert_front(3);
        assert_eq!(list.to_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn test_delete() {
        let mut list = LinkedList::new();
        list.insert_front(1);
        list.insert_front(2);
        list.insert_front(3);

        assert!(list.delete(2));
        assert_eq!(list.to_vec(), vec![3, 1]);

        assert!(!list.delete(5));
    }

    #[test]
    fn test_reverse() {
        let mut list = LinkedList::new();
        list.insert_front(1);
        list.insert_front(2);
        list.insert_front(3); // 3-> 2 -> 1

        list.reverse(); // 1 -> 2 -> 3
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }
}
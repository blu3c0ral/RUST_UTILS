extern crate rust_utils;

#[cfg(test)]
mod tests {

    #[test]
    fn push_pop_size_test() {
        let mut l = rust_utils::data_structures::List::new();

        l.push(1);
        l.push(3);
        l.push(8);

        assert_eq!(l.len(), 3);

        assert_eq!(l.pop(), Some(8));
        assert_eq!(l.pop(), Some(3));

        assert_eq!(l.len(), 1);

        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);

        assert_eq!(l.is_empty(), true);

        l.push(3);
        l.push(8);

        assert_eq!(l.pop(), Some(8));

        l.push(4);
        l.push(2);

        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(4));
        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), None);
    }

    #[test]
    fn peek_test() {
        let mut l = rust_utils::data_structures::List::new();

        l.push(1);
        l.push(3);
        l.push(8);

        assert_eq!(l.peek(), Some(&8));
        l.pop();
        assert_eq!(l.peek(), Some(&3));
        l.pop();
        assert_eq!(l.peek(), Some(&1));
    }

    #[test]
    fn into_iter_test() {
        let mut l = rust_utils::data_structures::List::new();

        l.push(1);
        l.push(3);
        l.push(8);

        let mut itr = l.into_iter();

        assert_eq!(itr.next(), Some(8));
        assert_eq!(itr.next(), Some(3));
        assert_eq!(itr.next(), Some(1));
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn iter_test() {
        let mut l = rust_utils::data_structures::List::new();

        l.push(1);
        l.push(3);
        l.push(8);

        let mut itr = l.iter();

        assert_eq!(itr.next(), Some(&8));
        assert_eq!(itr.next(), Some(&3));
        assert_eq!(itr.next(), Some(&1));
        assert_eq!(itr.next(), None);
    }
}
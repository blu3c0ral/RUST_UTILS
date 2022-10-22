extern crate rust_utils;

#[cfg(test)]
mod tests {
    
    #[test]
    fn max_insert() {
        let mut h: rust_utils::data_structures::Heap<i32> = rust_utils::data_structures::Heap::max_heap();
        h.insert(1);
        h.insert(3);
        h.insert(6);
        h.insert(8);
        h.insert(2);
        h.insert(10);
        h.insert(4);
        h.insert(33);
        h.insert(9);
        h.insert(33);
        h.insert(6);

        assert_eq!(h.extract_first(), Some(33));
        assert_eq!(h.extract_first(), Some(33));
        assert_eq!(h.extract_first(), Some(10));
        assert_eq!(h.extract_first(), Some(9));
        assert_eq!(h.extract_first(), Some(8));
        assert_eq!(h.extract_first(), Some(6));
        assert_eq!(h.extract_first(), Some(6));
        assert_eq!(h.extract_first(), Some(4));
        assert_eq!(h.extract_first(), Some(3));
        assert_eq!(h.extract_first(), Some(2));
        assert_eq!(h.extract_first(), Some(1));
        
        assert_eq!(h.is_empty(), true);
    }

    #[test]
    fn max_heapify() {
        let v = vec![1, 3, 6, 8, 2, 10, 4, 33, 9, 33, 6];
        let mut h: rust_utils::data_structures::Heap<i32> = rust_utils::data_structures::Heap::max_heap();

        h.heapify(v);

        assert_eq!(h.extract_first(), Some(33));
        assert_eq!(h.extract_first(), Some(33));
        assert_eq!(h.extract_first(), Some(10));
        assert_eq!(h.extract_first(), Some(9));
        assert_eq!(h.extract_first(), Some(8));
        assert_eq!(h.extract_first(), Some(6));
        assert_eq!(h.extract_first(), Some(6));
        assert_eq!(h.extract_first(), Some(4));
        assert_eq!(h.extract_first(), Some(3));
        assert_eq!(h.extract_first(), Some(2));
        assert_eq!(h.extract_first(), Some(1));
        
        assert_eq!(h.is_empty(), true);

    }

    #[test]
    fn min_insert() {
        let mut h: rust_utils::data_structures::Heap<i32> = rust_utils::data_structures::Heap::min_heap();
        h.insert(1);
        h.insert(3);
        h.insert(6);
        h.insert(8);
        h.insert(2);
        h.insert(10);
        h.insert(4);
        h.insert(33);
        h.insert(9);
        h.insert(33);
        h.insert(6);

        assert_eq!(h.extract_first(), Some(1));
        assert_eq!(h.extract_first(), Some(2));
        assert_eq!(h.extract_first(), Some(3));
        assert_eq!(h.extract_first(), Some(4));
        assert_eq!(h.extract_first(), Some(6));
        assert_eq!(h.extract_first(), Some(6));
        assert_eq!(h.extract_first(), Some(8));
        assert_eq!(h.extract_first(), Some(9));
        assert_eq!(h.extract_first(), Some(10));
        assert_eq!(h.extract_first(), Some(33));
        assert_eq!(h.extract_first(), Some(33));
        
        assert_eq!(h.is_empty(), true);
    }

    #[test]
    fn min_heapify() {
        let v = vec![1, 3, 6, 8, 2, 10, 4, 33, 9, 33, 6];
        let mut h: rust_utils::data_structures::Heap<i32> = rust_utils::data_structures::Heap::min_heap();

        h.heapify(v);


        assert_eq!(h.extract_first(), Some(1));
        assert_eq!(h.extract_first(), Some(2));
        assert_eq!(h.extract_first(), Some(3));
        assert_eq!(h.extract_first(), Some(4));
        assert_eq!(h.extract_first(), Some(6));
        assert_eq!(h.extract_first(), Some(6));
        assert_eq!(h.extract_first(), Some(8));
        assert_eq!(h.extract_first(), Some(9));
        assert_eq!(h.extract_first(), Some(10));
        assert_eq!(h.extract_first(), Some(33));
        assert_eq!(h.extract_first(), Some(33));
        
        assert_eq!(h.is_empty(), true);

    }
}
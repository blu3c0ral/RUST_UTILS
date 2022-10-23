extern crate rust_utils;

#[cfg(test)]
mod tests {
    use rust_utils::sorting::Qsort;
    use rust_utils::sorting::PivotType;
    use rust_utils::sorting::Order;

    #[test]
    fn sort_test() {
        let pivot_type = PivotType::HIGHEST;
        let order = Order::ASC;
        let qsr: Qsort<i32> = Qsort::new(pivot_type, order);
        let mut v: Vec<i32>;

        v= qsr.sort(vec![3, 7, 1, 2, 9, 0, 5, 8]);
        assert_eq!(v, vec![0, 1, 2, 3, 5, 7, 8, 9]);

        v= qsr.sort(vec![3, 7, 1, 2, 9, 0, 5, 8, 7]);
        assert_eq!(v, vec![0, 1, 2, 3, 5, 7, 7, 8, 9]);

        v= qsr.sort(vec![3, 7, 1]);
        assert_eq!(v, vec![1, 3, 7]);

        v= qsr.sort(vec![3, 3, 1]);
        assert_eq!(v, vec![1, 3, 3]);

        v= qsr.sort(vec![3, 1]);
        assert_eq!(v, vec![1, 3]);

        v= qsr.sort(vec![3]);
        assert_eq!(v, vec![3]);

        v= qsr.sort(vec![]);
        assert_eq!(v, vec![]);

    }
}
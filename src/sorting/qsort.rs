use std::cmp::Ord;
use std::default::Default;

use rand::Rng;

use crate::data_structures::List;

pub enum PivotType {
    RAND,
    LOWEST,
    HIGHEST,
    MIDDLE,
}

pub enum Order {
    ASC,
    DES,
}

pub struct Qsort<T>
where
    T: Default + Ord,
{
    pivot_type: PivotType,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Qsort<T>
where   T: Default + Ord,
{
    pub fn new(pivot_type: PivotType, order: Order) -> Self {
        let comparator: fn(&T, &T) -> bool;
        match order {
            Order::ASC => comparator = |a, b| a < b,
            Order::DES => comparator = |a, b| a > b
        }
        
        Self {
            pivot_type,
            comparator,
        }
    }

    fn get_pivot_idx(&self, low: usize, high: usize) -> usize {
        match &self.pivot_type {
            PivotType::RAND => {
                rand::thread_rng().gen_range(low..high)
            },
            PivotType::LOWEST => low,
            PivotType::HIGHEST => high - 1,
            PivotType::MIDDLE => (low + high) / 2 + low,
        }
    }

    fn partition(&self, mut v: Vec<T>, mut low: usize, mut high: usize) -> (Vec<T>, usize) {
        let pivot = self.get_pivot_idx(low, high);

        v.swap(pivot, high - 1);

        for i in low..high - 1 {
            if (self.comparator)(&v[i], &v[high - 1]) {
                v.swap(low, i);
                low += 1;
            }
        }

        v.swap(low + 1, high - 1);

        (v, low + 1)
    }

    fn sort(&self, mut v: Vec<T>) -> Vec<T> {
        let mut low: usize = 0;
        let mut high: usize = v.len();

        let mut idx = 0;

        let mut stack: List<(usize, usize)> = List::new();

        stack.push((low, high));

        while !stack.is_empty() {
            stack.pop().map(|x| {
                (low, high) = x;
            });

            (v, idx) = self.partition(v, low, high);
            
            if idx - low > 1 {
                stack.push((low, idx));
            }

            if high - idx > 1 {
                stack.push((idx + 1, high));
            }
        }
        v
    }
}
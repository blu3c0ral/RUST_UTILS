// Heap Data Structure

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Ord,
{
    size: usize,
    elements: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where   T: Default + Ord,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            size: 0,
            elements: Vec::new(),
            comparator,
        }
    }

    fn sift_down(&mut self, mut idx: usize) -> usize {
        loop {
            let mut child_idx = self.child(idx, true);
            if child_idx.is_err() {
                return idx
            }
            let mut larget_idx = child_idx.unwrap();
            if (self.comparator)(&self.elements[idx], &self.elements[larget_idx]) {
                larget_idx = idx;
            }

            child_idx = self.child(idx, false);
            if child_idx.is_ok() {
                if (self.comparator)(&self.elements[child_idx.unwrap()], &self.elements[larget_idx]) {
                    larget_idx = child_idx.unwrap();
                }
            }

            if larget_idx == idx { return idx; }

            self.elements.swap(idx, larget_idx);
            idx = larget_idx;
        }
    }

    fn sift_up(&mut self, mut idx: usize) -> usize {
        if idx == 0 { return 0 }
        while idx > 0 {
            let parent = self.parent(idx);
            if (self.comparator)(&self.elements[idx], &self.elements[parent]) { 
                self.elements.swap(idx, parent);
                idx = parent;
            } else {
                return idx
            }
        }
        return idx
    }

    fn parent(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn child(&self, idx: usize, first: bool) -> Result<usize, u32> {
        let ret = idx * 2 + if first {1} else {2};
        if self.size > ret { Ok(ret) } else { Err(0) }
    }

    fn delete_idx(&mut self, idx: usize) -> Option<T> {
        if idx > self.size - 1 { return None }
        
        if self.size == 1 {
            self.size = 0;
        } else {
            self.elements.swap(idx, self.size - 1);
            self.size -= 1;
            self.sift_down(idx);
        }

        return self.elements.pop(); 
    }
    
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, el: T) {
        self.elements.push(el);
        self.sift_up(self.size);
        self.size += 1;
    }

    pub fn peek_first(&self) -> Option<&T> {
        if self.size == 0 { return None }
        return Some(&self.elements[0])
    }

    pub fn extract_first(&mut self) -> Option<T> {
        return self.delete_idx(0);
    }

    pub fn replace_first(&mut self, el: T) -> Option<T> {
        if self.size == 0 { return None; }

        self.elements.swap(0, self.size - 1);
        let ret_val = self.elements.pop();

        self.elements.push(el);
        self.elements.swap(0, self.size - 1);
        self.sift_down(0);

        return ret_val;
    }

    pub fn delete_element(&mut self, el: &T) -> Option<T> {
        let idx = self.elements.iter().position(|x| x == el);
        match idx {
            None => return None,
            Some(x) => return self.delete_idx(x),
        }
    }

    pub fn heapify(&mut self, v: Vec<T>) -> Option<&T> {
        self.elements = v;
        self.size = self.elements.len();

        for i in (0..=self.parent(self.size - 1)).rev() {
            self.sift_down(i);
        }

        Some(&self.elements[0])
    }

}


impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn min_heap() -> Heap<T> {
        Self::new(|a, b| a < b)
    }

    pub fn max_heap() -> Heap<T> {
        Self::new(|a, b| a > b)
    }
}

/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        self.items.push(value);
        let mut cur_idx = self.count;
        let mut par_idx = cur_idx / 2;
        while par_idx >= 1 {
            if (self.comparator)(&self.items[cur_idx], &self.items[par_idx]) {
                self.items.swap(cur_idx, par_idx)
            }
            cur_idx = par_idx;
            par_idx /= 2;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.is_empty() {
            return None;
        }
        else {
            let res = self.items.swap_remove(1);
            self.count -= 1;
            let mut cur_idx = 1;
            let mut cl_idx = cur_idx * 2;
            let mut cr_idx = cl_idx + 1;
            loop {
                if cl_idx <= self.len() && cr_idx <= self.len() {
                    match ((self.comparator)(&self.items[cur_idx], &self.items[cl_idx]),
                           (self.comparator)(&self.items[cur_idx], &self.items[cr_idx])) {
                        (true, true) => break,
                        (true, false) => {
                            self.items.swap(cur_idx, cr_idx);
                            cur_idx = cr_idx;
                            cl_idx = cur_idx * 2;
                            cr_idx = cl_idx + 1;
                        },
                        (false, true) => {
                            self.items.swap(cur_idx, cl_idx);
                            cur_idx = cl_idx;
                            cl_idx = cur_idx * 2;
                            cr_idx = cl_idx + 1;
                        },
                        (false, false) => {
                            if(self.comparator)(&self.items[cl_idx], &self.items[cr_idx]) {
                                self.items.swap(cur_idx, cl_idx);
                                cur_idx = cl_idx;
                                cl_idx = cur_idx * 2;
                                cr_idx = cl_idx + 1;
                            }
                            else {
                                self.items.swap(cur_idx, cr_idx);
                                cur_idx = cr_idx;
                                cl_idx = cur_idx * 2;
                                cr_idx = cl_idx + 1;
                            }
                        },
                    }                    
                }
                else if cl_idx > self.len() && cr_idx > self.len() {
                    break;
                }
                else {
                    if cl_idx <= self.len() {
                        match (self.comparator)(&self.items[cur_idx], &self.items[cl_idx]) {
                            true => {
                                cur_idx = cl_idx;
                                cl_idx = cur_idx * 2;
                                cr_idx = cl_idx + 1;
                            },
                            false => break,
                        }
                    }
                }

            }
            return Some(res);
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.items, vec![0, 2, 4, 9, 11]);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
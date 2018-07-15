use std::clone::Clone;
use std::cmp::Ord;
use std::vec::Vec;

pub struct ArrayHeap<T> {
    v: Vec<T>,
}

impl<T: Ord + Clone> ArrayHeap<T> {
    pub fn new() -> ArrayHeap<T> {
        ArrayHeap { v: Vec::new() }
    }

    pub fn with_elem(items: &[T]) -> ArrayHeap<T> {
        let mut h = ArrayHeap { v: Vec::new() };
        h.v.extend_from_slice(items);

        for i in (0..h.v.len() - 1).rev() {
            h.bubble_down(i)
        }
        h
    }

    pub fn insert(&mut self, value: T) {
        let n = self.v.len();
        self.v.insert(n, value);
        self.bubble_up(n);
    }

    fn bubble_up(&mut self, cur_index: usize) {
        if cur_index == 0 {
            return;
        }

        let parent_index = (cur_index - 1) / 2;
        if self.v[parent_index] > self.v[cur_index] {
            self.v.swap(cur_index, parent_index);
            self.bubble_up(parent_index)
        }
    }

    pub fn items(&self) -> Vec<T> {
        self.v.clone()
    }

    pub fn shift(&mut self) -> Option<T> {
        if self.v.is_empty() {
            return None;
        }

        let min = self.v.swap_remove(0);
        self.bubble_down(0);

        Some(min)
    }

    fn bubble_down(&mut self, cur_index: usize) {
        let mut min_index = cur_index;

        let child_index = 2 * cur_index + 1;
        for i in &[child_index, child_index + 1] {
            match self.v.get(*i) {
                None => {}
                Some(child) => {
                    if child < &self.v[min_index] {
                        min_index = *i
                    }
                }
            }
        }

        if min_index != cur_index {
            self.v.swap(cur_index, min_index);
            self.bubble_down(min_index);
        }
    }
}

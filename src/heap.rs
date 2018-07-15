use std::clone::Clone;
use std::cmp::Ord;
use std::vec::Vec;

pub struct Heap<T> {
    q: Vec<T>,
}

impl<T: Ord + Clone> Heap<T> {
    pub fn new() -> Heap<T> {
        Heap { q: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn with_elem(items: &[T]) -> Heap<T> {
        let mut q: Heap<T> = Heap::new();
        for i in items {
            q.insert(i.clone());
        }
        q
    }

    pub fn insert(&mut self, value: T) {
        let n = self.q.len();
        self.q.insert(n, value);
        self.bubble_up(n);
    }

    fn bubble_up(&mut self, cur_index: usize) {
        if cur_index == 0 {
            return;
        }

        let parent_index = (cur_index - 1) / 2;
        if self.q[parent_index] > self.q[cur_index] {
            self.q.swap(cur_index, parent_index);
            self.bubble_up(parent_index)
        }
    }

    pub fn items(&self) -> Vec<T> {
        self.q.clone()
    }

    pub fn shift(&mut self) -> Option<T> {
        if self.q.is_empty() {
            return None;
        }

        let min = self.q.swap_remove(0);
        self.bubble_down(0);

        Some(min)
    }

    fn bubble_down(&mut self, cur_index: usize) {
        let mut min_index = cur_index;

        let child_index = 2 * cur_index + 1;
        for i in &[child_index, child_index + 1] {
            match self.q.get(*i) {
                None => {}
                Some(child) => {
                    if child < &self.q[min_index] {
                        min_index = *i
                    }
                }
            }
        }

        if min_index != cur_index {
            self.q.swap(cur_index, min_index);
            self.bubble_down(min_index);
        }
    }
}

use std::clone::Clone;
use std::cmp::Ord;
use std::vec::Vec;

pub struct ArrayHeap<T> {
    v: Vec<T>,
}

fn get_first_child(n: usize) -> usize {
    2 * n + 1
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

    pub fn index(&self, i: usize) -> Option<&T> {
        if i >= self.v.len() {
            return None;
        }
        Some(&self.v[i])
    }

    pub fn items(&self) -> Vec<T> {
        self.v.clone()
    }

    pub fn len(&self) -> usize {
        self.v.len()
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

        let child_index = get_first_child(cur_index);
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

pub fn sort<T: Clone + Ord>(items: &mut [T]) {
    let mut q: ArrayHeap<T> = ArrayHeap::with_elem(items);

    for i in 0..items.len() {
        match q.shift() {
            None => {
                return;
            }
            Some(item) => {
                items[i] = item;
            }
        }
    }
}

pub fn is_n_smallest_gte<T: Clone + Ord>(items: &[T], k: usize, val: &T) -> bool {
    let h = ArrayHeap::with_elem(items);
    compare_level(&h, 0, k as i32, &val) >= 0
}

fn compare_level<T: Clone + Ord>(h: &ArrayHeap<T>, cur_index: usize, level: i32, val: &T) -> i32 {
    if level < 0 || cur_index >= h.len() {
        return level;
    }

    if h.index(cur_index).unwrap() >= val {
        return level;
    }

    let level1 = compare_level(h, get_first_child(cur_index), level - 1, val);
    compare_level(h, get_first_child(cur_index) + 1, level1, val)
}

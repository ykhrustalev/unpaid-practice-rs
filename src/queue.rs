use std;

pub struct Queue<T> {
    q: std::vec::Vec<T>,
}

#[allow(dead_code)]
pub fn with_elem<T: std::cmp::Ord + std::clone::Clone>(items: &[T]) -> Queue<T> {
    let mut q: Queue<T> = Queue::new();
    for i in items {
        q.insert(i.clone());
    }
    q
}

impl<T: std::cmp::Ord + std::clone::Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            q: std::vec::Vec::new(),
        }
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

    pub fn items(&self) -> std::vec::Vec<T> {
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

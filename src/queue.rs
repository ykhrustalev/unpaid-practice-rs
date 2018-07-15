use std;

pub struct Queue<T> {
    q: std::vec::Vec<T>,
    n: usize,
}

#[allow(dead_code)]
pub fn from_elem<T: std::cmp::Ord + std::clone::Clone>(items: &[T]) -> Queue<T> {
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
            n: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.q.insert(self.n, value);
        let n = self.n;
        //        let n = self.q.len()  - 1;
        self.bubble_up(n); // todo: use len -1
        self.n += 1;
    }

    fn bubble_up(&mut self, cur_index: usize) {
        if cur_index == 0 {
            return;
        }

        let parent_index = (cur_index - 1) / 2;

        match self.q[parent_index].cmp(&self.q[cur_index]) {
            std::cmp::Ordering::Greater => {
                self.q.swap(cur_index, parent_index);
                self.bubble_up(parent_index)
            }
            _ => {}
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
        self.n -= 1;
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
                    if child < self.q.get(min_index).unwrap() {
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

#[test]
fn test_queue() {
    let mut q: Queue<i32> = from_elem(&[12, 11, 13, 5, 6, 7]);

    assert_eq!(q.items(), [5, 6, 7, 12, 11, 13]);

    assert_eq!(q.shift(), Some(5));
    assert_eq!(q.shift(), Some(6));
    assert_eq!(q.shift(), Some(7));
    assert_eq!(q.shift(), Some(11));
    assert_eq!(q.shift(), Some(12));
    //    assert_eq!(q.shift(), Some(13));

    assert_eq!(q.items(), [5, 7, 8, 9]);

    q.insert(10);
    q.insert(3);
    q.insert(11);

    assert_eq!(q.items(), [3, 7, 5, 9, 10, 8, 11]);
    assert_eq!(q.shift(), Some(3));
    assert_eq!(q.shift(), Some(5));
    assert_eq!(q.shift(), Some(7));
    assert_eq!(q.shift(), Some(8));
    assert_eq!(q.shift(), Some(9));
    assert_eq!(q.shift(), Some(10));
    assert_eq!(q.shift(), Some(11));
}

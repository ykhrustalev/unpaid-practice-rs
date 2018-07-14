use std;
use self::Element::*;

// todo: use Option
enum Element {
    Root,
    Item(usize),
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Root => write!(f, "root"),
            &Item(n) => write!(f, "item({})", n)
        }
    }
}

impl std::cmp::PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        match other {
            &Root => match self {
                &Root => true,
                _ => false,
            },
            &Item(a) => match self {
                &Item(b) => a == b,
                _ => false,
            },
        }
    }
}


fn get_parent(n: usize) -> Element {
    match n {
        0 => Root,
        x @ _ => Item((x - 1) / 2),
    }
}

fn get_young_child(n: usize) -> usize {
    2 * n + 1

//        (n*2)-1
}


pub struct Queue<T> {
    q: std::vec::Vec<T>,
    n: usize,
}

pub fn from_elem<T: std::cmp::Ord + std::clone::Clone>(items: &[T]) -> Queue<T> {
    let mut q: Queue<T> = Queue::new();
    for i in items {
        q.insert(i.clone());
    }
    q
}

impl<T: std::cmp::Ord + std::clone::Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { q: std::vec::Vec::new(), n: 0 }
    }

    pub fn insert(&mut self, value: T) {
        self.q.insert(self.n, value);
        let n = self.n;
        self.bubble_up(n);
        self.n += 1;
    }

    pub fn items(&self) -> std::vec::Vec<T> {
        self.q.clone()
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.q.is_empty() {
            return None;
        }
        let min = self.q.remove(0);
        match self.q.pop() {
            None => {}
            Some(last) => {
                self.q.insert(0, last);
                self.n -= 1;
                self.bubble_down(0)
            }
        }
        Some(min)
    }

    fn bubble_up(&mut self, p: usize) {
        match get_parent(p) {
            Root => {}
            Item(parent) => {
                match self.q[parent].cmp(&self.q[p]) {
                    std::cmp::Ordering::Greater => {
                        self.q.swap(p, parent);
                        self.bubble_up(parent)
                    }
                    _ => {}
                }
            }
        }
    }

    fn bubble_down(&mut self, p: usize) {
        let c = get_young_child(p);
        let mut min_index = p;

        for i in 0..1 {
            let candidate = c + i;
            if candidate <= self.n && self.q[min_index] > self.q[candidate] {
                min_index = candidate
            }
        }

        if min_index != p {
            self.q.swap(p, min_index);
            self.bubble_down(min_index);
        }
    }
}

#[test]
fn test_parent() {
    assert_eq!(get_parent(0), Root);
    assert_eq!(get_parent(1), Item(0));
    assert_eq!(get_parent(2), Item(0));
    assert_eq!(get_parent(9), Item(4));
    assert_eq!(get_parent(10), Item(4));
}


#[test]
fn test_get_young_child() {
    assert_eq!(get_young_child(0), 1);
    assert_eq!(get_young_child(1), 3);
    assert_eq!(get_young_child(2), 5);
}

#[test]
fn test_queue() {
    let mut q: Queue<i32> = Queue::new();

    q.insert(5);
    q.insert(2);
    q.insert(8);
    q.insert(7);
    q.insert(9);

    assert_eq!(q.items(), [2, 5, 8, 7, 9]);

    assert_eq!(q.extract_min(), Some(2));
    assert_eq!(q.items(), [5, 7, 8, 9]);

    q.insert(10);
    q.insert(3);
    q.insert(11);

    assert_eq!(q.items(), [3, 7, 5, 9, 10, 8, 11]);
    assert_eq!(q.extract_min(), Some(3));
    assert_eq!(q.extract_min(), Some(5));
    assert_eq!(q.extract_min(), Some(7));
    assert_eq!(q.extract_min(), Some(8));
    assert_eq!(q.extract_min(), Some(9));
    assert_eq!(q.extract_min(), Some(10));
    assert_eq!(q.extract_min(), Some(11));

}

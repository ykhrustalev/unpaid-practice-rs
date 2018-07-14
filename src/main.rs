fn main() {
    let mut q: Queue<i32> = Queue::new();

    q.insert(5);
    q.insert(2);
    q.insert(8);
    q.insert(7);

    for i in &q.q {
        println!("{}", i)
    }
}


enum Element {
    Root,
    Item(usize),
}

use Element::*;

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


fn pg_parent(n: usize) -> Element {
    match n {
        0 => Root,
        x @ _ => Item((x - 1) / 2),
    }
}

#[test]
fn test_pg_parent() {
    assert_eq!(pg_parent(0), Root);
    assert_eq!(pg_parent(1), Item(0));
    assert_eq!(pg_parent(2), Item(0));
    assert_eq!(pg_parent(9), Item(4));
    assert_eq!(pg_parent(10), Item(4));
}

//#[derive(Copy, Clone)]
pub struct Queue<T> {
    q: std::vec::Vec<T>,
    n: usize,
}


impl<T: std::cmp::Ord + std::fmt::Display> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            q: std::vec::Vec::new(),
            n: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.q.insert(self.n, value);
        let n = self.n;
        self.bubble_up(n);
        self.n += 1;
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.q.swap(a, b)
    }

    fn bubble_up(&mut self, p: usize) {
        match pg_parent(p) {
            Root => {}
            Item(parent) => {
                match self.q[parent].cmp(&self.q[p]) {
                    std::cmp::Ordering::Greater => {
                        self.swap(p, parent);
                        self.bubble_up(parent)
                    }
                    _ => {}
                }
            }
        }
    }
}

#[test]
fn test_pg() {
    let mut q: Queue<i32> = Queue::new();

    q.insert(5);
    q.insert(2);
    q.insert(8);
    q.insert(7);

    assert_eq!(q.q[0], 2);
    assert_eq!(q.q[1], 5);
    assert_eq!(q.q[2], 8);
    assert_eq!(q.q[3], 7);
}

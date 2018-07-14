fn main() {
    println!("Hello, world!");
}

enum PQElement {
    Root,
    Item(usize),
}

use PQElement::*;

impl std::fmt::Debug for PQElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Root => write!(f, "root"),
            Item(n) => write!(f, "item({})", n)
        }
    }
}

impl std::cmp::PartialEq for PQElement {
    fn eq(&self, other: &PQElement) -> bool {
        match other {
            Root => match self {
                Root => true,
                _ => false,
            },
            Item(a) => match self {
                Item(b) => a == b,
                _ => false,
            },
        }
    }
}

const PQ_SIZE: usize = 10;

struct PQ {
    q: [i32; PQ_SIZE],
    n: usize,
}

fn pg_parent(n: usize) -> PQElement {
    match n {
        1 => Root,
        x @ _ => Item(x / 2),
    }
}

#[test]
fn test_pg_parent() {
    assert_eq!(pg_parent(1), Root);
    assert_eq!(pg_parent(2), Item(1));
    assert_eq!(pg_parent(3), Item(1));
    assert_eq!(pg_parent(10), Item(5));
    assert_eq!(pg_parent(11), Item(5));
}

fn pg_young_child(n: usize) -> usize {
    n * 2
}

fn pg_insert(q: &mut PQ, x: i32) {
    if q.n >= PQ_SIZE {
        println!("overflow");
        return;
    }

    q.n += 1;
    q.q[q.n] = x;
    let n = q.n;
    bubble_up(q, n)
}

#[test]
fn test_pg_insert() {
    let q = &mut PQ { q: [0; 10], n: 0 };

    pg_insert(q, 5);
    pg_insert(q, 2);
    pg_insert(q, 8);
    pg_insert(q, 7);

    assert_eq!(q.q[0], 2);
    assert_eq!(q.q[1], 5);
    assert_eq!(q.q[2], 7);
    assert_eq!(q.q[3], 8);
}

fn bubble_up(q: &mut PQ, p: usize) {
    match pg_parent(p) {
        Item(parent) => {
            if q.q[parent] > q.q[p] {
                pg_swap(q, p, parent);
                bubble_up(q, parent)
            }
        }
        Root => {}
    }
}

fn pg_swap(q: &mut PQ, a: usize, b: usize) {
    if a >= PQ_SIZE || b >= PQ_SIZE {
        return;
    }

    let (x, y) = (q.q[a], q.q[b]);
    q.q[b] = x;
    q.q[a] = y;
}


#[test]
fn test_pg_swap() {
    let q = &mut PQ { q: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], n: 0 };

    pg_swap(q, 0, 9);
    pg_swap(q, 0, 19);

    assert_eq!(q.q[0], 10);
    assert_eq!(q.q[9], 1);
}
use std::clone::Clone;
use std::cmp::Ord;

use super::heap::*;

pub fn sort<T: Clone + Ord>(items: &mut [T]) {
    let mut q: Heap<T> = Heap::with_elem(items);

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

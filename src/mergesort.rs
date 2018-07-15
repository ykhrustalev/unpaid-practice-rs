use std::clone::Clone;
use std::cmp::Ord;
use std::vec::*;

pub fn sort<T: Clone + Ord>(items: &mut Vec<T>) {
    let n = items.len();
    merge_sort(items, 0, n);
}

fn merge_sort<T: Clone + Ord>(items: &mut Vec<T>, low: usize, high: usize) {
    if high - low < 2 {
        return;
    }

    let middle = (low + high + 1) / 2;
    merge_sort(items, low, middle);
    merge_sort(items, middle, high);
    merge(items, low, middle, high)
}

fn merge<T: Clone + Ord>(items: &mut Vec<T>, low: usize, middle: usize, high: usize) {
    let mut left = match items.get(low..middle) {
        None => Vec::new(),
        Some(slice) => slice.to_vec()
    };
    let mut right = match items.get(middle..high) {
        None => Vec::new(),
        Some(slice) => slice.to_vec()
    };

    let mut i = low;
    while !(left.is_empty() || right.is_empty()) {
        if left.first().unwrap() <= right.first().unwrap() {
            items[i] = left.remove(0);
        } else {
            items[i] = right.remove(0);
        }
        i += 1;
    }

    for item in left {
        items[i] = item;
        i += 1;
    }

    for item in right {
        items[i] = item;
        i += 1;
    }
}

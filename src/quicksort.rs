use std::clone::Clone;
use std::cmp::Ord;

pub fn sort<T: Clone + Ord>(items: &mut [T]) {
    let n = items.len();
    qsort(items, 0, n);
}

fn qsort<T: Clone + Ord>(items: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    let p = partition(items, low, high);
    qsort(items, 0, p);
    qsort(items, p + 1, high);
}

fn partition<T: Clone + Ord>(items: &mut [T], low: usize, high: usize) -> usize {
    let mut firsthigh = low;

    for i in low..high {
        if items[i] < items[high - 1] {
            items.swap(firsthigh, i);
            firsthigh += 1;
        }
    }

    items.swap(firsthigh, high - 1);

    firsthigh
}

mod queue;

use std::iter::FromIterator;

fn main() {
    let mut q: queue::Queue<i32> = queue::from_elem(&[
        1492,
        1783,
        1776,
        1804,
        1865,
        1945,
        1963,
        1918,
        2001,
        1941,
    ]);

    println!("items: {}", std::vec::Vec::from_iter(q.items().iter().map(|x| x.to_string())).join(","));
    println!("min: {}", q.extract_min().unwrap());
    println!("items: {}", std::vec::Vec::from_iter(q.items().iter().map(|x| x.to_string())).join(","));

}

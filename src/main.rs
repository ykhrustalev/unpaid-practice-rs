pub mod heap;

use std::iter::FromIterator;

impl std::fmt::Display for heap::Heap<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let x = std::vec::Vec::from_iter(self.items().iter().map(|x| x.to_string())).join(",");
        std::fmt::Display::fmt(&x, f)
    }
}

fn main() {
    let mut h: heap::Heap<i32> = heap::Heap::new();
    let items = &[1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918, 2001, 1941];

    for item in items {
        h.insert(item.clone());
        println!("add {} [{}]", item, h);
    }

    for _ in 0..items.len() + 1 {
        match h.shift() {
            None => println!("none"),
            Some(item) => println!("shift {} [{}]", item, h),
        }
    }
}

mod queue;

use std::iter::FromIterator;

impl std::fmt::Display for queue::Queue<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let x = std::vec::Vec::from_iter(self.items().iter().map(|x| x.to_string())).join(",");
        std::fmt::Display::fmt(&x, f)
    }
}

fn main() {
    let mut q: queue::Queue<i32> = queue::Queue::new();
    let items = &[1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918, 2001, 1941];

    for item in items {
        q.insert(item.clone());
        println!("add {} [{}]", item, q);
    }

//    q.shift();
//    q.shift();

    for _ in 0..items.len() + 1 {
        match q.shift() {
            None => println!("none"),
            Some(item) => println!("shift {} [{}]", item, q),
        }
    }
}

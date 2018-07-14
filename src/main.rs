mod queue;

fn main() {
//    let mut q: Queue<i32> = Queue::new();

//    q.insert(5);
//    q.insert(2);
//    q.insert(8);
//    q.insert(7);

    let q: queue::Queue<i32> = queue::Queue::new_with_elements(&[5, 2, 8, 7]);
    for i in &q.items() {
        println!("{}", i)
    }
}

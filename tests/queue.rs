extern crate unpaidpractice;

use unpaidpractice::queue::*;


#[test]
fn test_1() {
    let mut q: Queue<i32> = Queue::new();

    q.insert(1492);
    assert_eq!(q.items(), [1492]);

    q.insert(1783);
    assert_eq!(q.items(), [1492, 1783]);

    q.insert(1776);
    assert_eq!(q.items(), [1492, 1783, 1776]);

    q.insert(1804);
    assert_eq!(q.items(), [1492, 1783, 1776, 1804]);

    q.insert(1865);
    assert_eq!(q.items(), [1492, 1783, 1776, 1804, 1865]);

    q.insert(1945);
    assert_eq!(q.items(), [1492, 1783, 1776, 1804, 1865, 1945]);

    q.insert(1963);
    assert_eq!(q.items(), [1492, 1783, 1776, 1804, 1865, 1945, 1963]);

    q.insert(1918);
    assert_eq!(q.items(), [1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918]);

    q.insert(2001);
    assert_eq!(q.items(), [1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918, 2001]);

    q.insert(1941);
    assert_eq!(q.items(), [1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918, 2001, 1941]);

    assert_eq!(q.shift(), Some(1492));
    assert_eq!(q.items(), [1776, 1783, 1941, 1804, 1865, 1945, 1963, 1918, 2001]);

    assert_eq!(q.shift(), Some(1776));
    assert_eq!(q.items(), [1783, 1804, 1941, 1918, 1865, 1945, 1963, 2001]);

    assert_eq!(q.shift(), Some(1783));
    assert_eq!(q.items(), [1804, 1865, 1941, 1918, 2001, 1945, 1963]);

    assert_eq!(q.shift(), Some(1804));
    assert_eq!(q.items(), [1865, 1918, 1941, 1963, 2001, 1945]);

    assert_eq!(q.shift(), Some(1865));
    assert_eq!(q.items(), [1918, 1945, 1941, 1963, 2001]);

    assert_eq!(q.shift(), Some(1918));
    assert_eq!(q.items(), [1941, 1945, 2001, 1963]);

    assert_eq!(q.shift(), Some(1941));
    assert_eq!(q.items(), [1945, 1963, 2001]);

    assert_eq!(q.shift(), Some(1945));
    assert_eq!(q.items(), [1963, 2001]);

    assert_eq!(q.shift(), Some(1963));
    assert_eq!(q.items(), [2001]);

    assert_eq!(q.shift(), Some(2001));
    assert_eq!(q.items(), []);

    assert_eq!(q.shift(), None);
    assert_eq!(q.items(), []);
}

#[test]
fn test_2() {
    let mut q: Queue<i32> = with_elem(&[12, 11, 13, 5, 6, 7]);

    assert_eq!(q.items(), [5, 6, 7, 12, 11, 13]);

    assert_eq!(q.shift(), Some(5));
    assert_eq!(q.shift(), Some(6));
    assert_eq!(q.shift(), Some(7));
    assert_eq!(q.shift(), Some(11));
    assert_eq!(q.shift(), Some(12));
    assert_eq!(q.shift(), Some(13));
    assert_eq!(q.shift(), None);
    assert_eq!(q.items(), []);

    q.insert(10);
    q.insert(3);
    q.insert(11);
    q.insert(2);
    q.insert(1);
    q.insert(9);
    q.insert(10);

    assert_eq!(q.items(), [1, 2, 9, 10, 3, 11, 10]);
    assert_eq!(q.shift(), Some(1));
    assert_eq!(q.shift(), Some(2));
    assert_eq!(q.shift(), Some(3));
    assert_eq!(q.shift(), Some(9));
    assert_eq!(q.shift(), Some(10));
    assert_eq!(q.shift(), Some(10));
    assert_eq!(q.shift(), Some(11));
    assert_eq!(q.shift(), None);
    assert_eq!(q.items(), []);
}
extern crate unpaidpractice;

use unpaidpractice::heap::*;

#[test]
fn test_1() {
    let mut h: ArrayHeap<i32> = ArrayHeap::new();

    h.insert(1492);
    assert_eq!(h.items(), [1492]);

    h.insert(1783);
    assert_eq!(h.items(), [1492, 1783]);

    h.insert(1776);
    assert_eq!(h.items(), [1492, 1783, 1776]);

    h.insert(1804);
    assert_eq!(h.items(), [1492, 1783, 1776, 1804]);

    h.insert(1865);
    assert_eq!(h.items(), [1492, 1783, 1776, 1804, 1865]);

    h.insert(1945);
    assert_eq!(h.items(), [1492, 1783, 1776, 1804, 1865, 1945]);

    h.insert(1963);
    assert_eq!(h.items(), [1492, 1783, 1776, 1804, 1865, 1945, 1963]);

    h.insert(1918);
    assert_eq!(h.items(), [1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918]);

    h.insert(2001);
    assert_eq!(
        h.items(),
        [1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918, 2001]
    );

    h.insert(1941);
    assert_eq!(
        h.items(),
        [1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918, 2001, 1941]
    );

    assert_eq!(h.shift(), Some(1492));
    assert_eq!(
        h.items(),
        [1776, 1783, 1941, 1804, 1865, 1945, 1963, 1918, 2001]
    );

    assert_eq!(h.shift(), Some(1776));
    assert_eq!(h.items(), [1783, 1804, 1941, 1918, 1865, 1945, 1963, 2001]);

    assert_eq!(h.shift(), Some(1783));
    assert_eq!(h.items(), [1804, 1865, 1941, 1918, 2001, 1945, 1963]);

    assert_eq!(h.shift(), Some(1804));
    assert_eq!(h.items(), [1865, 1918, 1941, 1963, 2001, 1945]);

    assert_eq!(h.shift(), Some(1865));
    assert_eq!(h.items(), [1918, 1945, 1941, 1963, 2001]);

    assert_eq!(h.shift(), Some(1918));
    assert_eq!(h.items(), [1941, 1945, 2001, 1963]);

    assert_eq!(h.shift(), Some(1941));
    assert_eq!(h.items(), [1945, 1963, 2001]);

    assert_eq!(h.shift(), Some(1945));
    assert_eq!(h.items(), [1963, 2001]);

    assert_eq!(h.shift(), Some(1963));
    assert_eq!(h.items(), [2001]);

    assert_eq!(h.shift(), Some(2001));
    assert_eq!(h.items(), []);

    assert_eq!(h.shift(), None);
    assert_eq!(h.items(), []);
}

#[test]
fn test_2() {
    let mut h: ArrayHeap<i32> = ArrayHeap::new();

    h.insert(10);
    h.insert(3);
    h.insert(11);
    h.insert(2);
    h.insert(1);
    h.insert(9);
    h.insert(10);

    assert_eq!(h.items(), [1, 2, 9, 10, 3, 11, 10]);
    assert_eq!(h.shift(), Some(1));
    assert_eq!(h.shift(), Some(2));
    assert_eq!(h.shift(), Some(3));
    assert_eq!(h.shift(), Some(9));
    assert_eq!(h.shift(), Some(10));
    assert_eq!(h.shift(), Some(10));
    assert_eq!(h.shift(), Some(11));
    assert_eq!(h.shift(), None);
    assert_eq!(h.items(), []);
}

#[test]
fn test_with_elem() {
    let h: ArrayHeap<i32> =
        ArrayHeap::with_elem(&[1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918, 2001, 1941]);

    assert_eq!(
        h.items(),
        [1492, 1783, 1776, 1804, 1865, 1945, 1963, 1918, 2001, 1941]
    );
}

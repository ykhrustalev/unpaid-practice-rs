extern crate unpaidpractice;

use unpaidpractice::heap::*;

#[test]
fn test_heap_1() {
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
fn test_heap_2() {
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

#[test]
fn test_heasort_1() {
    let mut items = [8, 5, 3, 1, 6, 8, 9, 9, 12];
    sort(&mut items);
    assert_eq!(items, [1, 3, 5, 6, 8, 8, 9, 9, 12]);
}

#[test]
fn test_heasort_2() {
    let mut items = ["b", "a", "d", "c"];
    sort(&mut items);
    assert_eq!(items, ["a", "b", "c", "d"]);
}


#[test]
fn test_is_n_smallest_gte() {
    let items = &[4, 1, 7, 3, 6, 2, 5];

    let x = 3;

    assert!(!is_n_smallest_gte(items, 0, &x));
    assert!(!is_n_smallest_gte(items, 1, &x));
    assert!(is_n_smallest_gte(items, 2, &x));
    assert!(is_n_smallest_gte(items, 3, &x));
}
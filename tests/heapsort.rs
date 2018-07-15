extern crate unpaidpractice;

use unpaidpractice::heapsort::*;

#[test]
fn test_1() {
    let mut items = [8, 5, 3, 1, 6, 8, 9, 9, 12];
    sort(&mut items);
    assert_eq!(items, [1, 3, 5, 6, 8, 8, 9, 9, 12]);
}

#[test]
fn test_2() {
    let mut items = ["b", "a", "d", "c"];
    sort(&mut items);
    assert_eq!(items, ["a", "b", "c", "d"]);
}

extern crate unpaidpractice;

use unpaidpractice::quicksort::*;

#[test]
fn test_sort0() {
    let mut items = vec![8, 3, 5];
    sort(&mut items);
    assert_eq!(items, vec![3, 5, 8]);
}

#[test]
fn test_sort1() {
    let mut items = vec![8, 5, 3, 1, 6, 8, 9, 9, 12];
    sort(&mut items);
    assert_eq!(items, vec![1, 3, 5, 6, 8, 8, 9, 9, 12]);
}

#[test]
fn test_sort2() {
    let mut items = vec!["b", "a", "d", "c"];
    sort(&mut items);
    assert_eq!(items, vec!["a", "b", "c", "d"]);
}

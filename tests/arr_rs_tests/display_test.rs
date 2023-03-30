use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(Array::new(vec![], vec![0]), "[]"),
case(Array::new(vec![1, 2, 3, 4], vec![4]), "[1, 2, 3, 4]"),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), "[[1, 2], [3, 4]]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![8]), "[1, 2, 3, 4, 5, 6, 7, 8]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]), "[[1, 2], [3, 4], [5, 6], [7, 8]]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), "[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]"),
)] fn test_display(array: Array<i32>, expected: &str) {
    assert_eq!(expected, format!("{array}"))
}

#[rstest(
array, expected,
case(Array::new(vec![], vec![0]), "[]"),
case(Array::new(vec![1, 2, 3, 4], vec![4]), "[1, 2, 3, 4]"),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), "[[1, 2], \n [3, 4]]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![8]), "[1, 2, 3, 4, 5, 6, 7, 8]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]), "[[1, 2], \n [3, 4], \n [5, 6], \n [7, 8]]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), "[[[1, 2], \n  [3, 4]], \n [[5, 6], \n  [7, 8]]]"),
)] fn test_display_pretty(array: Array<i32>, expected: &str) {
    assert_eq!(expected, format!("{array:#}"))
}

use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(Array::empty(), "[]"),
case(array!(i32, [1, 2, 3, 4]), "[1, 2, 3, 4]"),
case(array!(i32, [[1, 2], [3, 4]]), "[[1, 2], [3, 4]]"),
case(array!(i32, [1, 2, 3, 4, 5, 6, 7, 8]), "[1, 2, 3, 4, 5, 6, 7, 8]"),
case(array!(i32, [[1, 2, 3, 4], [5, 6, 7, 8]]), "[[1, 2, 3, 4], [5, 6, 7, 8]]"),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), "[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]"),
)] fn test_display(array: Result<Array<i32>, ArrayError>, expected: &str) {
    assert_eq!(expected, format!("{}", array.unwrap()))
}

#[rstest(
array, expected,
case(Array::empty(), "[]"),
case(array!(i32, [1, 2, 3, 4]), "[1, 2, 3, 4]"),
case(array!(i32, [[1, 2], [3, 4]]), "[[1, 2],\n [3, 4]]"),
case(array!(i32, [1, 2, 3, 4, 5, 6, 7, 8]), "[1, 2, 3, 4, 5, 6, 7, 8]"),
case(array!(i32, [[1, 2, 3, 4], [5, 6, 7, 8]]), "[[1, 2, 3, 4],\n [5, 6, 7, 8]]"),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), "[[[1, 2],\n  [3, 4]],\n [[5, 6],\n  [7, 8]]]"),
)] fn test_display_pretty(array: Result<Array<i32>, ArrayError>, expected: &str) {
    assert_eq!(expected, format!("{:#}", array.unwrap()))
}

#[rstest(
array, expected,
case(Array::empty(), "[]"),
case(array!(f64, [1., 2., 3., 4.]), "[1.00, 2.00, 3.00, 4.00]"),
case(array!(f64, [[1., 2.], [3., 4.]]), "[[1.00, 2.00], [3.00, 4.00]]"),
case(array!(f64, [1., 2., 3., 4., 5., 6., 7., 8.]), "[1.00, 2.00, 3.00, 4.00, 5.00, 6.00, 7.00, 8.00]"),
case(array!(f64, [[1., 2., 3., 4.], [5., 6., 7., 8.]]), "[[1.00, 2.00, 3.00, 4.00], [5.00, 6.00, 7.00, 8.00]]"),
case(array!(f64, [[[1., 2.], [3., 4.]], [[5., 6.], [7., 8.]]]), "[[[1.00, 2.00], [3.00, 4.00]], [[5.00, 6.00], [7.00, 8.00]]]"),
)] fn test_display_precision(array: Result<Array<f64>, ArrayError>, expected: &str) {
    assert_eq!(expected, format!("{:.2}", array.unwrap()))
}

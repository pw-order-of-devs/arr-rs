use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_flat!(1, 2, 3, 4), vec![4]),
case(array_flat!(1, 2, 3, 4, 5, 6, 7, 8), vec![8]),
)] fn test_new_array_macro(arr: Result<Array<i32>, ArrayError>, expected: Vec<usize>) {
    assert_eq!(expected, arr.get_shape().unwrap());
}

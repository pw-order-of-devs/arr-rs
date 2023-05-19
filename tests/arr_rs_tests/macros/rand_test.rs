use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_rand!(8), vec![8]),
case(array_rand!(3, 3), vec![3, 3]),
case(array_rand!(2, 2, 2), vec![2, 2, 2]),
)] fn test_rand_array_macro(arr: Result<Array<i32>, ArrayError>, expected: Vec<usize>) {
    assert_eq!(expected, arr.get_shape().unwrap());
}

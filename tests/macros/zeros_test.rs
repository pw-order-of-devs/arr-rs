use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_zeros!(i32, 8), array![i32, 0, 0, 0, 0, 0, 0, 0, 0]),
case(array_zeros!(i32, 3, 3), array!(i32, [[0, 0, 0], [0, 0, 0], [0, 0, 0]])),
case(array_zeros!(i32, 2, 2, 2), array!(i32, [[[0, 0], [0, 0]], [[0, 0], [0, 0]]])),
)] fn test_zeros_array_macro(arr: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, arr);
}

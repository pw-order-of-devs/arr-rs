use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_ones!(i32, 8), array![i32, 1, 1, 1, 1, 1, 1, 1, 1]),
case(array_ones!(i32, 3, 3), array!(i32, [[1, 1, 1], [1, 1, 1], [1, 1, 1]])),
case(array_ones!(i32, 2, 2, 2), array!(i32, [[[1, 1], [1, 1]], [[1, 1], [1, 1]]])),
)] fn test_ones_array_macro(arr: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, arr);
}

use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_ones!(8), array![1, 1, 1, 1, 1, 1, 1, 1]),
case(array_ones!(3, 3), array!([[1, 1, 1], [1, 1, 1], [1, 1, 1]])),
case(array_ones!(2, 2, 2), array!([[[1, 1], [1, 1]], [[1, 1], [1, 1]]])),
)] fn test_ones_array_macro(arr: Array<i32>, expected: Array<i32>) {
    assert_eq!(expected, arr);
}

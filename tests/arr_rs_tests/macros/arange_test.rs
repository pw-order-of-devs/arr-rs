use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_arange!(0, 5), array!([0, 1, 2, 3, 4])),
case(array_arange!(0, 5, 1), array!([0, 1, 2, 3, 4])),
case(array_arange!(0, 8, 2), array!([0, 2, 4, 6])),
)] fn test_arange_array_macro(arr: Array<i32>, expected: Array<i32>) {
    assert_eq!(expected, arr);
}
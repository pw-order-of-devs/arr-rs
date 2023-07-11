use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_arange!(0, 4), array!([0, 1, 2, 3, 4])),
case(array_arange!(0, 4, 1), array!([0, 1, 2, 3, 4])),
case(array_arange!(0, 7, 2), array!([0, 2, 4, 6])),
)] fn test_arange_array_macro(arr: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, arr);
}
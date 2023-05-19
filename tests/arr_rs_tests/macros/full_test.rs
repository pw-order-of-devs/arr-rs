use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_full!(vec![8], 2), array![2, 2, 2, 2, 2, 2, 2, 2]),
case(array_full!(vec![3, 3], 2), array!([[2, 2, 2], [2, 2, 2], [2, 2, 2]])),
case(array_full!(vec![2, 2, 2], 2), array!([[[2, 2], [2, 2]], [[2, 2], [2, 2]]])),
)] fn test_full_array_macro(arr: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, arr);
}

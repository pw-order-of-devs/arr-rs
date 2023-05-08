use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_zeros!(8), array![0, 0, 0, 0, 0, 0, 0, 0]),
case(array_zeros!(3, 3), array!([[0, 0, 0], [0, 0, 0], [0, 0, 0]])),
case(array_zeros!(2, 2, 2), array!([[[0, 0], [0, 0]], [[0, 0], [0, 0]]])),
)] fn test_zeros_array_macro(arr: Array<i32>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr);
}

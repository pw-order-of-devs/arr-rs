use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array![1, 4, 9, 16], array![1, 2, 3, 4]),
case(array!([[1, 4], [9, 16]]), array!([[1, 2], [3, 4]])),
)] fn test_sqrt(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.sqrt())
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], array![1, 2, 3, 4]),
case(array![1, -2, 3, -4], array![1, 2, 3, 4]),
case(array!([[1, -2], [3, -4]]), array!([[1, 2], [3, 4]])),
)] fn test_absolute(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.absolute())
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], array![1, 2, 3, 4]),
case(array![1, -2, 3, -4], array![1, 2, 3, 4]),
case(array!([[1, -2], [3, -4]]), array!([[1, 2], [3, 4]])),
)] fn test_abs(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.abs())
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], array![1, 2, 3, 4]),
case(array![1, -2, 3, -4], array![1, 2, 3, 4]),
case(array!([[1, -2], [3, -4]]), array!([[1, 2], [3, 4]])),
)] fn test_fabs(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.fabs())
}

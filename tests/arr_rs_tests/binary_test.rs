use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, expected,
case(array_flat![13], array_flat![17], array_flat![1]),
case(array_flat![11, 7], array_flat![4, 25], array_flat![0, 1]),
case(array_flat![2, 5, 255], array_flat![3, 14, 16], array_flat![2, 4, 16]),
case(array_flat![1, 2, 3], array_flat![2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_bitwise_and(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.bitwise_and(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat![13], array_flat![16], array_flat![29]),
case(array_flat![33, 4], array_flat![1, 2], array_flat![33, 6]),
case(array_flat![2, 5, 255], array_flat![4, 4, 4], array_flat![6, 5, 255]),
case(array_flat![1, 2, 3], array_flat![2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_bitwise_or(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.bitwise_or(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat![13], array_flat![17], array_flat![28]),
case(array_flat![31], array_flat![5], array_flat![26]),
case(array_flat![31, 3], array_flat![5, 6], array_flat![26, 5]),
case(array_flat![1, 2, 3], array_flat![2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_bitwise_xor(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.bitwise_xor(&other.unwrap()))
}

#[rstest(
array, expected,
case(array!([13]), Array::<u8>::flat(vec![242])),
case(array!([13]), Array::<u16>::flat(vec![65522])),
case(array!([13]), Array::<i32>::flat(vec![-14])),
)] fn test_bitwise_not<N: Numeric>(array: Result<Array<N>, ArrayError>, expected: Result<Array<N>, ArrayError>) {
    assert_eq!(expected, array.bitwise_not())
}

#[rstest(
array, expected,
case(array!([13]), Array::<u8>::flat(vec![242])),
case(array!([13]), Array::<u16>::flat(vec![65522])),
case(array!([13]), Array::<i32>::flat(vec![-14])),
)] fn test_invert<N: Numeric>(array: Result<Array<N>, ArrayError>, expected: Result<Array<N>, ArrayError>) {
    assert_eq!(expected, array.invert())
}

#[rstest(
array, other, expected,
case(array_flat![5], array_flat![2], array_flat![20]),
case(array_flat![5], array_flat![1, 2, 3], array_flat![10, 20, 40]),
case(array_flat![1, 2, 3], array_flat![2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_left_shift(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.left_shift(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat![10], array_flat![1], array_flat![5]),
case(array_flat![10], array_flat![1, 2, 3], array_flat![5, 2, 1]),
case(array_flat![1, 2, 3], array_flat![2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_right_shift(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.right_shift(&other.unwrap()))
}

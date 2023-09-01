use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, expected,
case(array_flat![i32, 13], array_flat![i32, 17], array_flat![i32, 1]),
case(array_flat![i32, 11, 7], array_flat![i32, 4, 25], array_flat![i32, 0, 1]),
case(array_flat![i32, 2, 5, 255], array_flat![i32, 3, 14, 16], array_flat![i32, 2, 4, 16]),
case(array_flat![i32, 1, 2, 3], array_flat![i32, 2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_bitwise_and(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.bitwise_and(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat![i32, 13], array_flat![i32, 16], array_flat![i32, 29]),
case(array_flat![i32, 33, 4], array_flat![i32, 1, 2], array_flat![i32, 33, 6]),
case(array_flat![i32, 2, 5, 255], array_flat![i32, 4, 4, 4], array_flat![i32, 6, 5, 255]),
case(array_flat![i32, 1, 2, 3], array_flat![i32, 2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_bitwise_or(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.bitwise_or(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat![i32, 13], array_flat![i32, 17], array_flat![i32, 28]),
case(array_flat![i32, 31], array_flat![i32, 5], array_flat![i32, 26]),
case(array_flat![i32, 31, 3], array_flat![i32, 5, 6], array_flat![i32, 26, 5]),
case(array_flat![i32, 1, 2, 3], array_flat![i32, 2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_bitwise_xor(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.bitwise_xor(&other.unwrap()))
}

#[rstest(
array, expected,
case(array!(u8, [13]), Array::<u8>::flat(vec![242])),
case(array!(u16, [13]), Array::<u16>::flat(vec![65522])),
case(array!(i32, [13]), Array::<i32>::flat(vec![-14])),
)] fn test_bitwise_not<N: Numeric>(array: Result<Array<N>, ArrayError>, expected: Result<Array<N>, ArrayError>) {
    assert_eq!(expected, array.bitwise_not())
}

#[rstest(
array, expected,
case(array!(u8, [13]), Array::<u8>::flat(vec![242])),
case(array!(u16, [13]), Array::<u16>::flat(vec![65522])),
case(array!(i32, [13]), Array::<i32>::flat(vec![-14])),
)] fn test_invert<N: Numeric>(array: Result<Array<N>, ArrayError>, expected: Result<Array<N>, ArrayError>) {
    assert_eq!(expected, array.invert())
}

#[rstest(
array, other, expected,
case(array_flat![i32, 5], array_flat![i32, 2], array_flat![i32, 20]),
case(array_flat![i32, 5], array_flat![i32, 1, 2, 3], array_flat![i32, 10, 20, 40]),
case(array_flat![i32, 1, 2, 3], array_flat![i32, 2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_left_shift(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.left_shift(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat![i32, 10], array_flat![i32, 1], array_flat![i32, 5]),
case(array_flat![i32, 10], array_flat![i32, 1, 2, 3], array_flat![i32, 5, 2, 1]),
case(array_flat![i32, 1, 2, 3], array_flat![i32, 2, 2, 2, 2], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_right_shift(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.right_shift(&other.unwrap()))
}

#[rstest(
num, expected,
case(2_u8, "10"),
case(3_u8, "11"),
case(-3_i8, "11111101"),
case(255_i32, "11111111"),
)] fn test_binary_repr<N: Numeric>(num: N, expected: &str) {
    assert_eq!(expected, Array::binary_repr(num))
}

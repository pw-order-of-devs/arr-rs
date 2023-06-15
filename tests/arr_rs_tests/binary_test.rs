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

#[rstest(
array, axis, count, bit_order, expected,
case(array_flat!(2, 3, 5), None, None, None, array!([0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1])),
case(array_flat!(2, 3, 5), None, None, Some(BitOrder::Little), array!([0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0])),
case(array_flat!(2, 3, 5), None, Some(1), None, array_flat!(0)),
case(array!([[2], [3], [5]]), Some(0), None, None, array!([[0], [0], [0], [0], [0], [0], [1], [0], [0], [0], [0], [0], [0], [0], [1], [1], [0], [0], [0], [0], [0], [1], [0], [1]])),
case(array!([[2], [3], [5]]), Some(1), None, None, array!([[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 1, 0, 1]])),
case(array!([[[2]], [[3]], [[5]]]), Some(0), None, None, array!([[[0]], [[0]], [[0]], [[0]], [[0]], [[0]], [[1]], [[0]], [[0]], [[0]], [[0]], [[0]], [[0]], [[0]], [[1]], [[1]], [[0]], [[0]], [[0]], [[0]], [[0]], [[1]], [[0]], [[1]]])),
case(array!([[[2]], [[3]], [[5]]]), Some(1), None, None, array!([[[0], [0], [0], [0], [0], [0], [1], [0]], [[0], [0], [0], [0], [0], [0], [1], [1]], [[0], [0], [0], [0], [0], [1], [0], [1]]])),
case(array!([[[2]], [[3]], [[5]]]), Some(2), None, None, array!([[[0, 0, 0, 0, 0, 0, 1, 0]], [[0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1]]])),
case(array!([[[2, 2]], [[3, 3]], [[5, 5]]]), Some(0), None, None, array!([[[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[1, 1]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[1, 1]], [[1, 1]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[1, 1]], [[0, 0]], [[1, 1]]])),
case(array!([[[2, 2]], [[3, 3]], [[5, 5]]]), Some(1), None, None, array!([[[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [0, 0]], [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [1, 1]], [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [0, 0], [1, 1]]])),
case(array!([[[2, 2]], [[3, 3]], [[5, 5]]]), Some(2), None, None, array!([[[0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0]], [[0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1]]])),
)] fn test_unpack_bits(array: Result<Array<u8>, ArrayError>, axis: Option<isize>, count: Option<isize>, bit_order: Option<BitOrder>, expected: Result<Array<u8>, ArrayError>) {
    assert_eq!(expected, array.unpack_bits(axis, count, bit_order))
}

#[rstest(
array, axis, bit_order, expected,
case(array!([0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1]), None, None, array_flat!(2, 3, 5)),
case(array!([0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0]), None, Some(BitOrder::Little), array_flat!(2, 3, 5)),
case(array!([[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 1, 0, 1]]), None, None, array_flat!(2, 3, 5)),
case(array!([[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 1, 0, 1]]), Some(0), None, array!([[0, 0, 0, 0, 0, 32, 192, 96]])),
case(array!([[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 1, 0, 1]]), Some(1), None, array!([[2], [3], [5]])),
case(array!([[[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1], [0, 0, 0, 0,  0, 0, 1, 1]]]), None, None, array_flat!(2, 3, 5, 3)),
case(array!([[[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1], [0, 0, 0, 0,  0, 0, 1, 1]]]), Some(0), None, array!([[[0, 0, 0, 0, 0, 64, 128, 64], [0, 0, 0, 0, 0, 0, 192, 192]]])),
case(array!([[[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1], [0, 0, 0, 0,  0, 0, 1, 1]]]), Some(1), None, array!([[[0, 0, 0, 0, 0, 0, 192, 64]], [[0, 0, 0, 0, 0, 128, 64, 192]]])),
case(array!([[[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1], [0, 0, 0, 0,  0, 0, 1, 1]]]), Some(2), None, array!([[[2], [3]], [[5], [3]]])),
)] fn test_pack_bits(array: Result<Array<u8>, ArrayError>, axis: Option<isize>, bit_order: Option<BitOrder>, expected: Result<Array<u8>, ArrayError>) {
    assert_eq!(expected, array.pack_bits(axis, bit_order))
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

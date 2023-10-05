use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, axis, count, bit_order, expected,
case(array_flat!(u8, 2, 3, 5), None, None, None, array!(u8, [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1])),
case(array_flat!(u8, 2, 3, 5), None, None, Some(BitOrder::Little), array!(u8, [0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0])),
case(array_flat!(u8, 2, 3, 5), None, Some(1), None, array_flat!(u8, 0)),
case(array!(u8, [[2], [3], [5]]), Some(0), None, None, array!(u8, [[0], [0], [0], [0], [0], [0], [1], [0], [0], [0], [0], [0], [0], [0], [1], [1], [0], [0], [0], [0], [0], [1], [0], [1]])),
case(array!(u8, [[2], [3], [5]]), Some(1), None, None, array!(u8, [[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 1, 0, 1]])),
case(array!(u8, [[[2]], [[3]], [[5]]]), Some(0), None, None, array!(u8, [[[0]], [[0]], [[0]], [[0]], [[0]], [[0]], [[1]], [[0]], [[0]], [[0]], [[0]], [[0]], [[0]], [[0]], [[1]], [[1]], [[0]], [[0]], [[0]], [[0]], [[0]], [[1]], [[0]], [[1]]])),
case(array!(u8, [[[2]], [[3]], [[5]]]), Some(1), None, None, array!(u8, [[[0], [0], [0], [0], [0], [0], [1], [0]], [[0], [0], [0], [0], [0], [0], [1], [1]], [[0], [0], [0], [0], [0], [1], [0], [1]]])),
case(array!(u8, [[[2]], [[3]], [[5]]]), Some(2), None, None, array!(u8, [[[0, 0, 0, 0, 0, 0, 1, 0]], [[0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1]]])),
case(array!(u8, [[[2, 2]], [[3, 3]], [[5, 5]]]), Some(0), None, None, array!(u8, [[[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[1, 1]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[1, 1]], [[1, 1]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[0, 0]], [[1, 1]], [[0, 0]], [[1, 1]]])),
case(array!(u8, [[[2, 2]], [[3, 3]], [[5, 5]]]), Some(1), None, None, array!(u8, [[[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [0, 0]], [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [1, 1]], [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [0, 0], [1, 1]]])),
case(array!(u8, [[[2, 2]], [[3, 3]], [[5, 5]]]), Some(2), None, None, array!(u8, [[[0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0]], [[0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1]]])),
)] fn test_unpack_bits(array: Result<Array<u8>, ArrayError>, axis: Option<isize>, count: Option<isize>, bit_order: Option<BitOrder>, expected: Result<Array<u8>, ArrayError>) {
    assert_eq!(expected, array.unpack_bits(axis, count, bit_order))
}

#[rstest(
array, bit_order, expected,
case(array_flat!(u8, 2, 3, 5), Some("big"), array!(u8, [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1])),
case(array_flat!(u8, 2, 3, 5), Some("little"), array!(u8, [0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0])),
case(array_flat!(u8, 2, 3, 5), Some("err"), Err(ArrayError::ParameterError { param: "`bit_order`", message: "must be one of {`big`, `little`}", })),
)] fn test_unpack_bits_str(array: Result<Array<u8>, ArrayError>, bit_order: Option<&str>, expected: Result<Array<u8>, ArrayError>) {
    assert_eq!(expected, array.unpack_bits(None, None, bit_order))
}

#[rstest(
array, axis, bit_order, expected,
case(array!(u8, [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1]), None, None, array_flat!(u8, 2, 3, 5)),
case(array!(u8, [0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0]), None, Some(BitOrder::Little), array_flat!(u8, 2, 3, 5)),
case(array!(u8, [[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 1, 0, 1]]), None, None, array_flat!(u8, 2, 3, 5)),
case(array!(u8, [[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 1, 0, 1]]), Some(0), None, array!(u8, [[0, 0, 0, 0, 0, 32, 192, 96]])),
case(array!(u8, [[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1], [0, 0, 0, 0, 0, 1, 0, 1]]), Some(1), None, array!(u8, [[2], [3], [5]])),
case(array!(u8, [[[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 0, 0, 1, 1]]]), None, None, array_flat!(u8, 2, 3, 5, 3)),
case(array!(u8, [[[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 0, 0, 1, 1]]]), Some(0), None, array!(u8, [[[0, 0, 0, 0, 0, 64, 128, 64], [0, 0, 0, 0, 0, 0, 192, 192]]])),
case(array!(u8, [[[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 0, 0, 1, 1]]]), Some(1), None, array!(u8, [[[0, 0, 0, 0, 0, 0, 192, 64]], [[0, 0, 0, 0, 0, 128, 64, 192]]])),
case(array!(u8, [[[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 1]], [[0, 0, 0, 0, 0, 1, 0, 1], [0, 0, 0, 0, 0, 0, 1, 1]]]), Some(2), None, array!(u8, [[[2], [3]], [[5], [3]]])),
)] fn test_pack_bits(array: Result<Array<u8>, ArrayError>, axis: Option<isize>, bit_order: Option<BitOrder>, expected: Result<Array<u8>, ArrayError>) {
    assert_eq!(expected, array.pack_bits(axis, bit_order))
}

#[rstest(
array, bit_order, expected,
case(array!(u8, [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1]), Some("big"), array_flat!(u8, 2, 3, 5)),
case(array!(u8, [0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0]), Some("little"), array_flat!(u8, 2, 3, 5)),
case(array!(u8, [0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0]), Some("err"), Err(ArrayError::ParameterError { param: "`bit_order`", message: "must be one of {`big`, `little`}", })),
)] fn test_pack_bits_str(array: Result<Array<u8>, ArrayError>, bit_order: Option<&str>, expected: Result<Array<u8>, ArrayError>) {
    assert_eq!(expected, array.pack_bits(None, bit_order))
}

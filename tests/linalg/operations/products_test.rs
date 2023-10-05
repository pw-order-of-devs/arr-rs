use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, expected,
case(array_single!(i32, 2), array_single!(i32, 3), array_single!(i32, 6)),
case(array_single!(i32, 2), array_flat!(i32, 1, 2, 3), array_flat!(i32, 2, 4, 6)),
case(array_flat!(i32, 1, 2, 3), array_single!(i32, 2), array_flat!(i32, 2, 4, 6)),
case(array_single!(i32, 2), array!(i32, [[1, 2], [3, 4]]), array!(i32, [[2, 4], [6, 8]])),
case(array!(i32, [[1, 2], [3, 4]]), array_single!(i32, 2), array!(i32, [[2, 4], [6, 8]])),
case(array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 1, 2), array_flat!(i32, 7, 10)),
case(array_flat!(i32, 1, 2), array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 5, 11)),
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [[5, 6], [7, 8]]), array!(i32, [[19, 22], [43, 50]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, [[1, 2], [3, 4]]), array!(i32, [[[7, 10], [15, 22]], [[23, 34], [31, 46]]])),
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, [[[7, 10], [19, 22]], [[15, 22], [43, 50]]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, [[[[7, 10], [19, 22]], [[15, 22], [43, 50]]], [[[23, 34], [67, 78]], [[31, 46], [91, 106]]]])),
case(array_flat!(i32, 1, 2, 3), array!(i32, [[1, 2, 3], [4, 5, 6]]), Err(ArrayError::MustBeEqual { value1: "3".to_string(), value2: "2".to_string() })),
case(array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 1, 2, 3), Err(ArrayError::MustBeEqual { value1: "2".to_string(), value2: "3".to_string() })),
case(array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 1, 2, 3, 4), Err(ArrayError::MustBeEqual { value1: "2".to_string(), value2: "4".to_string() })),
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [[5, 6, 3], [7, 8, 3]]), Err(ArrayError::ParameterError { param: "`shapes`", message: "are not aligned" })),
case(array_arange!(i32, 1, 24).reshape(&[2, 3, 4]), array_arange!(i32, 1, 24).reshape(&[2, 3, 4]), Err(ArrayError::ParameterError { param: "`shapes`", message: "are not aligned" })),
)] fn test_linalg_dot(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.dot(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_single!(i32, 2), array_single!(i32, 3), array_single!(i32, 6)),
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [[5, 6], [7, 8]]), array!(i32, 70)),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, 204)),
case(array_flat!(i32, 1, 2, 3), array!(i32, [[1, 2, 3], [4, 5, 6]]), Err(ArrayError::MustBeEqual { value1: "3".to_string(), value2: "6".to_string() })),
case(array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 1, 2, 3), Err(ArrayError::MustBeEqual { value1: "4".to_string(), value2: "3".to_string() })),
)] fn test_linalg_vdot(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.vdot(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_single!(i32, 2), array_single!(i32, 3), array_single!(i32, 6)),
case(array_flat!(i32, 1, 2), array_flat!(i32, 1, 2), array_single!(i32, 5)),
case(array_flat!(i32, 1, 2), array!(i32, [[1, 2], [1, 2]]), array_flat!(i32, 3, 6)),
case(array!(i32, [[1, 2], [1, 2]]), array_flat!(i32, 1, 2), array_flat!(i32, 5, 5)),
case(array_flat!(i32, 1, 2), array!(i32, [[[1, 2], [1, 2]], [[1, 2], [1, 2]]]), array!(i32, [[3, 6], [3, 6]])),
case(array!(i32, [[[1, 2], [1, 2]], [[1, 2], [1, 2]]]), array_flat!(i32, 1, 2), array!(i32, [[5, 5], [5, 5]])),
case(array!(i32, [[1, 2], [1, 2]]), array!(i32, [[4, 1], [2, 2]]), array!(i32, [[8, 5], [8, 5]])),
case(array!(i32, [[1, 2, 3], [1, 2, 3]]), array!(i32, [[1, 2], [1, 2], [1, 2]]), array!(i32, [[6, 12], [6, 12]])),
case(array!(i32, [[1, 2], [1, 2], [1, 2]]), array!(i32, [[1, 2, 3], [1, 2, 3]]), array!(i32, [[3, 6, 9], [3, 6, 9], [3, 6, 9]])),
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), array_arange!(i32, 0, 3).reshape(&[2, 2]), array!(i32, [[[2, 3], [6, 11]], [[10, 19], [14, 27]]])),
case(array_arange!(i32, 0, 3).reshape(&[2, 2]), array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), array!(i32, [[[2, 3], [6, 11]], [[6, 7], [26, 31]]])),
)] fn test_linalg_matmul(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.matmul(&other.unwrap()))
}

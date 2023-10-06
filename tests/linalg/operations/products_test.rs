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
case(array_flat!(i32, 1, 2), array_flat!(i32, 3, 4), array_single!(i32, 11)),
case(array_flat!(i32, 1, 2, 3, 4), array_flat!(i32, 4, 3, 2, 1), array_single!(i32, 20)),
case(array_flat!(i32, 1, 2), array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 5, 11)),
case(array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 1, 2), array_flat!(i32, 5, 11)),
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [[4, 3], [2, 1]]), array!(i32, [[10, 4], [24, 10]])),
case(array_flat!(i32, 1, 2, 3), array!(i32, [[6, 5, 4], [3, 2, 1]]), array_flat!(i32, 28, 10)),
case(array!(i32, [[6, 5, 4], [3, 2, 1]]), array_flat!(i32, 1, 2, 3), array_flat!(i32, 28, 10)),
)] fn test_linalg_inner(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.inner(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_single!(i32, 2), array_single!(i32, 3), array!(i32, [[6]])),
case(array_flat!(i32, 1, 2), array_flat!(i32, 3, 4), array!(i32, [[3, 4], [6, 8]])),
case(array_flat!(i32, 1, 2, 3), array_flat!(i32, 4, 3), array!(i32, [[4, 3], [8, 6], [12, 9]])),
case(array_flat!(i32, 4, 3), array_flat!(i32, 1, 2, 3), array!(i32, [[4, 8, 12], [3, 6, 9]])),
case(array_flat!(i32, 1, 2), array!(i32, [[1, 2], [3, 4]]), array!(i32, [[1, 2, 3, 4], [2, 4, 6, 8]])),
case(array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 1, 2), array!(i32, [[1, 2], [2, 4], [3, 6], [4, 8]])),
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [[4, 3], [2, 1]]), array!(i32, [[4, 3, 2, 1], [8, 6, 4, 2], [12, 9, 6, 3], [16, 12, 8, 4]])),
)] fn test_linalg_outer(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.outer(&other.unwrap()))
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

#[rstest(
array, expected,
case(array!(i32, [[1, 2], [3, 4]]), array_single!(i32, -2)),
case(array!(i32, [[3, 8], [4, 6]]), array_single!(i32, -14)),
case(array!(i32, [[6, 2], [1, 4]]), array_single!(i32, 22)),
case(array!(i32, [[1, 2, 3], [3, 2, 1], [2, 1, 3]]), array_single!(i32, -12)),
case(array!(i32, [[5, 6, 4], [6, 3, 5], [8, 2, 8]]), array_single!(i32, -26)),
case(array!(i32, [[1, 2, 6, 6], [4, 7, 3, 2], [0, 0, 0, 0], [1, 2, 2, 9]]), array_single!(i32, 0)),
case(array!(i32, [[4, 3, 2, 2], [0, 1, -3, 3], [0, -1, 3, 3], [0, 3, 1, 1]]), array_single!(i32, -240)),
case(array!(i32, [[[1, 2], [3, 4]], [[4, 3], [2, 1]]]), array_flat!(i32, -2, -2)),
case(array!(i32, [[[1, 2, 3], [3, 2, 1], [2, 1, 3]], [[5, 6, 4], [6, 3, 5], [8, 2, 8]]]), array_flat!(i32, -12, -26)),
)] fn test_linalg_det(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.det())
}

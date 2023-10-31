use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, idx, expected,
case(array!(i32, [1, 2, 3, 4]), 0, Ok(vec![0])),
case(array!(i32, [1, 2, 3, 4]), 3, Ok(vec![3])),
case(array!(i32, [[1, 2], [3, 4]]), 0, Ok(vec![0, 0])),
case(array!(i32, [[1, 2], [3, 4]]), 3, Ok(vec![1, 1])),
case(array!(i32, [[1, 2, 3], [4, 5, 6]]), 4, Ok(vec![1, 1])),
case(array!(i32, [[1, 2], [3, 4], [5, 6]]), 4, Ok(vec![2, 0])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), 1, Ok(vec![0, 0, 1])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), 7, Ok(vec![1, 1, 1])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), 8, Err(ArrayError::ParameterError { param: "idx", message: "index must be smaller than array length", })),
)] fn test_index_to_coord(array: Result<Array<i32>, ArrayError>, idx: usize, expected: Result<Vec<usize>, ArrayError>) {
    assert_eq!(expected, array.index_to_coord(idx))
}

#[rstest(
array, coords, expected,
case(array!(i32, [1, 2, 3, 4]), &[0], Ok(0)),
case(array!(i32, [1, 2, 3, 4]), &[3], Ok(3)),
case(array!(i32, [[1, 2], [3, 4]]), &[0, 0], Ok(0)),
case(array!(i32, [[1, 2], [3, 4]]), &[1, 1], Ok(3)),
case(array!(i32, [[1, 2, 3], [4, 5, 6]]), &[1, 1], Ok(4)),
case(array!(i32, [[1, 2], [3, 4], [5, 6]]), &[2, 0], Ok(4)),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), &[0, 0, 1], Ok(1)),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), &[1, 1, 1], Ok(7)),
case(array!(i32, [[1, 2], [3, 4]]), &[2, 3, 4], Err(ArrayError::ParameterError { param: "coords", message: "length must match array dimension" })),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), &[2, 3, 4], Err(ArrayError::ParameterError { param: "coords", message: "value must match array shape" })),
)] fn test_index_at(array: Result<Array<i32>, ArrayError>, coords: &[usize], expected: Result<usize, ArrayError>) {
    assert_eq!(expected, array.index_at(coords))
}

#[rstest(
array, coords, expected,
case(array!(i32, [1, 2, 3, 4]), &[0], Ok(1)),
case(array!(i32, [1, 2, 3, 4]), &[3], Ok(4)),
case(array!(i32, [[1, 2], [3, 4]]), &[0, 0], Ok(1)),
case(array!(i32, [[1, 2], [3, 4]]), &[1, 1], Ok(4)),
case(array!(i32, [[1, 2, 3], [4, 5, 6]]), &[1, 1], Ok(5)),
case(array!(i32, [[1, 2], [3, 4], [5, 6]]), &[2, 0], Ok(5)),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), &[0, 0, 1], Ok(2)),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), &[1, 1, 1], Ok(8)),
case(array!(i32, [[1, 2], [3, 4]]), &[2, 3, 4], Err(ArrayError::ParameterError { param: "coords", message: "length must match array dimension" })),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), &[2, 3, 4], Err(ArrayError::ParameterError { param: "coords", message: "value must match array shape" })),
)] fn test_at(array: Result<Array<i32>, ArrayError>, coords: &[usize], expected: Result<i32, ArrayError>) {
    assert_eq!(expected, array.at(coords))
}

#[rstest(
array, range, expected,
case(array!(i32, [1, 2, 3, 4, 5, 6, 7, 8]), 0..4, array!(i32, [1, 2, 3, 4])),
case(array!(i32, [1, 2, 3, 4, 5, 6, 7, 8]), 0..6, array!(i32, [1, 2, 3, 4, 5, 6])),
case(array!(i32, [[1, 2], [3, 4], [5, 6], [7, 8]]), 0..1, array!(i32, [1, 2])),
case(array!(i32, [[1, 2], [3, 4], [5, 6], [7, 8]]), 0..2, array!(i32, [[1, 2], [3, 4]])),
case(array!(i32, [[1, 2], [3, 4], [5, 6], [7, 8]]), 1..2, array!(i32, [3, 4])),
case(array!(i32, [[1, 2], [3, 4], [5, 6], [7, 8]]), 2..4, array!(i32, [[5, 6], [7, 8]])),
case(array!(i32, [[1, 2, 3, 4], [5, 6, 7, 8]]), 0..1, array!(i32, [1, 2, 3, 4])),
case(array!(i32, [[1, 2, 3, 4], [5, 6, 7, 8]]), 0..2, array!(i32, [[1, 2, 3, 4], [5, 6, 7, 8]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), 0..1, array!(i32, [[1, 2], [3, 4]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), 2..3, array!(i32, [[5, 6], [7, 8]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), 3..10, Err(ArrayError::OutOfBounds { value: "slice range" })),
)] fn test_slice(array: Result<Array<i32>, ArrayError>, range: std::ops::Range<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.slice(range))
}

#[rstest(
array, indices, expected,
case(array!(i32, [1, 2, 3, 4, 5, 6, 7, 8]), &[4], array!(i32, [5])),
case(array!(i32, [1, 2, 3, 4, 5, 6, 7, 8]), &[0, 1, 2, 3], array!(i32, [1, 2, 3, 4])),
case(array!(i32, [1, 2, 3, 4, 5, 6, 7, 8]), &[3, 3, 3, 3], array!(i32, [4, 4, 4, 4])),
case(array!(i32, [1, 2, 3, 4, 5, 6, 7, 8]), &[7, 6, 5, 4, 3, 2, 1, 0], array!(i32, [8, 7, 6, 5, 4, 3, 2, 1])),
case(array!(i32, [[1, 2], [3, 4], [5, 6], [7, 8]]), &[10], Err(ArrayError::OutOfBounds { value: "indices" })),
case(array!(i32, [[1, 2], [3, 4], [5, 6], [7, 8]]), &[3, 2, 1, 0], array!(i32, [[7, 8], [5, 6], [3, 4], [1, 2]])),
case(array!(i32, [[1, 2], [3, 4], [5, 6], [7, 8]]), &[3, 3], array!(i32, [[7, 8], [7, 8]])),
case(array!(i32, [[1, 2], [3, 4], [5, 6], [7, 8]]), &[4], Err(ArrayError::OutOfBounds { value: "indices" })),
)] fn test_indices_at(array: Result<Array<i32>, ArrayError>, indices: &[usize], expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.indices_at(indices))
}

use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, idx, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), 0, Ok(vec![0])),
case(Array::new(vec![1, 2, 3, 4], vec![4]), 3, Ok(vec![3])),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), 0, Ok(vec![0, 0])),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), 3, Ok(vec![1, 1])),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), 4, Ok(vec![1, 1])),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), 4, Ok(vec![2, 0])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 1, Ok(vec![0, 0, 1])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 7, Ok(vec![1, 1, 1])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 8, Err(ArrayError::ParameterError { param: "idx", message: "index must be smaller than array length", })),
)] fn test_index_to_coord(array: Result<Array<i32>, ArrayError>, idx: usize, expected: Result<Vec<usize>, ArrayError>) {
    assert_eq!(expected, array.unwrap().index_to_coord(idx))
}

#[rstest(
array, coords, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[0], Ok(0)),
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[3], Ok(3)),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[0, 0], Ok(0)),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[1, 1], Ok(3)),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), &[1, 1], Ok(4)),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), &[2, 0], Ok(4)),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[0, 0, 1], Ok(1)),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[1, 1, 1], Ok(7)),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[2, 3, 4], Err(ArrayError::ParameterError { param: "coords", message: "length must match array dimension" })),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[2, 3, 4], Err(ArrayError::ParameterError { param: "coords", message: "value must match array shape" })),
)] fn test_index_at(array: Result<Array<i32>, ArrayError>, coords: &[usize], expected: Result<usize, ArrayError>) {
    assert_eq!(expected, array.unwrap().index_at(coords))
}

#[rstest(
array, coords, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[0], Ok(1)),
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[3], Ok(4)),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[0, 0], Ok(1)),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[1, 1], Ok(4)),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), &[1, 1], Ok(5)),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), &[2, 0], Ok(5)),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[0, 0, 1], Ok(2)),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[1, 1, 1], Ok(8)),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[2, 3, 4], Err(ArrayError::ParameterError { param: "coords", message: "length must match array dimension" })),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[2, 3, 4], Err(ArrayError::ParameterError { param: "coords", message: "value must match array shape" })),
)] fn test_at(array: Result<Array<i32>, ArrayError>, coords: &[usize], expected: Result<i32, ArrayError>) {
    assert_eq!(expected, array.unwrap().at(coords))
}

#[rstest(
array, range, expected,
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![8]), 0 .. 4, array!([1, 2, 3, 4])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![8]), 0 .. 6, array!([1, 2, 3, 4, 5, 6])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 2]), 0 .. 1, array!([1, 2])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 2]), 0 .. 2, array!([[1, 2], [3, 4]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 2]), 1 .. 2, array!([3, 4])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 2]), 2 .. 4, array!([[5, 6], [7, 8]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]), 0 .. 1, array!([1, 2, 3, 4])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]), 0 .. 2, array!([[1, 2, 3, 4], [5, 6, 7, 8]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 0 .. 1, array!([[1, 2], [3, 4]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 2 .. 3, array!([[5, 6], [7, 8]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 3 .. 10, Err(ArrayError::OutOfBounds { value: "slice range" })),
)] fn test_slice(array: Result<Array<i32>, ArrayError>, range: std::ops::Range<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.unwrap().slice(range))
}

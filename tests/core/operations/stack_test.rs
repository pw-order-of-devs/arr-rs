use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arrs, axis, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], None, array!([1, 2, 3, 4, 5, 6])),
case(vec![array!([[1, 2], [3, 4]]), array!([5, 6])], None, array!([1, 2, 3, 4, 5, 6])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6]])], Some(0), array!([[1, 2], [3, 4], [5, 6]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6]]), array!([[7, 8]])], Some(0), array!([[1, 2], [3, 4], [5, 6], [7, 8]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5], [6]])], Some(1), array!([[1, 2, 5], [3, 4, 6]])),
case(vec![array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]]])], Some(0), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
case(vec![array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]])], Some(0), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]], [[1, 2], [3, 4]], [[5, 6], [7, 8]]])),
case(vec![array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]])], Some(1), array!([[[1, 2], [3, 4], [1, 2], [3, 4]], [[1, 2], [3, 4], [1, 2], [3, 4]]])),
case(vec![array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]])], Some(2), array!([[[1, 2, 1, 2], [3, 4, 3, 4]], [[1, 2, 1, 2], [3, 4, 3, 4]]])),
case(vec![array!([[1, 2], [3, 4]]), array!([5, 6])], Some(0), Err(ArrayError::ConcatenateShapeMismatch)),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6]])], Some(1), Err(ArrayError::ConcatenateShapeMismatch)),
)] fn test_concatenate(arrs: Vec<Result<Array<i32>, ArrayError>>, axis: Option<usize>, expected: Result<Array<i32>, ArrayError>) {
    let arrs = arrs.iter().map(|a| a.as_ref().unwrap().clone()).collect();
    assert_eq!(expected, Array::concatenate(arrs, axis))
}

#[rstest(
arrs, axis, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], None, array!([[1, 2, 3], [4, 5, 6]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6], [7, 8]])], None, array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])),
case(vec![array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]])], Some(0), array!([[[[1, 2], [3, 4]]], [[[5, 6], [7, 8]]]])),
case(vec![array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]]), array!([[[9, 10], [11, 12]]])], Some(0), array!([[[[1, 2], [3, 4]]], [[[5, 6], [7, 8]]], [[[9, 10], [11, 12]]]])),
case(vec![array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]])], Some(1), array!([[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]])),
case(vec![array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]])], Some(2), array!([[[[1, 2], [5, 6]], [[3, 4], [7, 8]]]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6]])], None, Err(ArrayError::ParameterError { param: "arrs", message: "all input arrays must have the same shape" })),
)] fn test_stack(arrs: Vec<Result<Array<i32>, ArrayError>>, axis: Option<usize>, expected: Result<Array<i32>, ArrayError>) {
    let arrs = arrs.iter().map(|a| a.as_ref().unwrap().clone()).collect();
    assert_eq!(expected, Array::stack(arrs, axis))
}

#[rstest(
arrs, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], array!([[1, 2, 3], [4, 5, 6]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6], [7, 8], [9, 10]])], array!([[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]])),
case(vec![array!([[1]]), array!([[2]]), array!([[3]])], array!([[1], [2], [3]])),
case(vec![array!([[1, 2, 3], [1, 2, 3]]), Array::empty()], Err(ArrayError::ConcatenateShapeMismatch)),
case(vec![array!([[1, 2, 3], [1, 2, 3]]), array!([[5, 6]])], Err(ArrayError::ConcatenateShapeMismatch)),
)] fn test_vstack(arrs: Vec<Result<Array<i32>, ArrayError>>, expected: Result<Array<i32>, ArrayError>) {
    let arrs = arrs.iter().map(|a| a.as_ref().unwrap().clone()).collect();
    assert_eq!(expected, Array::vstack(arrs))
}

#[rstest(
arrs, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], array!([1, 2, 3, 4, 5, 6])),
case(vec![array!([1, 2, 3]), array!([4, 5, 6]), array!([7, 8])], array!([1, 2, 3, 4, 5, 6, 7, 8])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6], [7, 8]])], array!([[1, 2, 5, 6], [3, 4, 7, 8]])),
case(vec![array!([[1]]), array!([[2]]), array!([[3]])], array!([[1, 2, 3]])),
case(vec![array!([[1, 2], [1, 2]]), array!([5, 6])], Err(ArrayError::ConcatenateShapeMismatch)),
case(vec![array!([[1, 2], [1, 2]]), array!([[5], [6]])], Err(ArrayError::ConcatenateShapeMismatch)),
)] fn test_hstack(arrs: Vec<Result<Array<i32>, ArrayError>>, expected: Result<Array<i32>, ArrayError>) {
    let arrs = arrs.iter().map(|a| a.as_ref().unwrap().clone()).collect();
    assert_eq!(expected, Array::hstack(arrs))
}

#[rstest(
arrs, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], array!([[[1, 4], [2, 5], [3, 6]]])),
case(vec![array!([[1], [2], [3]]), array!([[4], [5], [6]])], array!([[[1, 4]], [[2, 5]], [[3, 6]]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6], [7, 8]])], array!([[[1, 5], [2, 6]], [[3, 7], [4, 8]]])),
case(vec![array!([1, 2, 3]), array!([4, 5, 6]), array!([7, 8])], Err(ArrayError::ConcatenateShapeMismatch)),
)] fn test_dstack(arrs: Vec<Result<Array<i32>, ArrayError>>, expected: Result<Array<i32>, ArrayError>) {
    let arrs = arrs.iter().map(|a| a.as_ref().unwrap().clone()).collect();
    assert_eq!(expected, Array::dstack(arrs))
}

#[rstest(
arrs, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], array!([[1, 4], [2, 5], [3, 6]])),
case(vec![array!([1, 2, 3]), array!([4, 5, 6]), array!([7, 8, 9])], array!([[1, 4, 7], [2, 5, 8], [3, 6, 9]])),
case(vec![array!([1, 2, 3]), array!([[4], [5], [6]])], array!([[1, 4], [2, 5], [3, 6]])),
case(vec![array!([[1, 2], [3, 4]]), array!([5, 6])], array!([[1, 2, 5], [3, 4, 6]])),
case(vec![array!([1, 2, 3]), array!([4, 5])], Err(ArrayError::ParameterError { param: "arrs", message: "all input arrays must have the same first dimension" })),
)] fn test_column_stack(arrs: Vec<Result<Array<i32>, ArrayError>>, expected: Result<Array<i32>, ArrayError>) {
    let arrs = arrs.iter().map(|a| a.as_ref().unwrap().clone()).collect();
    assert_eq!(expected, Array::column_stack(arrs))
}

#[rstest(
arrs, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], array!([[1, 2, 3], [4, 5, 6]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6], [7, 8], [9, 10]])], array!([[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]])),
case(vec![array!([[1]]), array!([[2]]), array!([[3]])], array!([[1], [2], [3]])),
case(vec![array!([[1, 2, 3], [1, 2, 3]]), Array::empty()], Err(ArrayError::ConcatenateShapeMismatch)),
case(vec![array!([[1, 2, 3], [1, 2, 3]]), array!([[5, 6]])], Err(ArrayError::ConcatenateShapeMismatch)),
)] fn test_row_stack(arrs: Vec<Result<Array<i32>, ArrayError>>, expected: Result<Array<i32>, ArrayError>) {
    let arrs = arrs.iter().map(|a| a.as_ref().unwrap().clone()).collect();
    assert_eq!(expected, Array::row_stack(arrs))
}

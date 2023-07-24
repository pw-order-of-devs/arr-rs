use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, indices, values, axis, expected,
case(array!([1, 2, 3]), vec![1], array!([4]), None, array!([1, 4, 2, 3])),
case(array!([1, 2, 3]), vec![3], array!([4]), None, array!([1, 2, 3, 4])),
case(array!([1, 2, 3]), vec![5], array!([4]), None, Err(ArrayError::OutOfBounds { value: "index" })),
case(array!([1, 2, 3]), vec![1, 3], array!([4, 5]), None, array!([1, 4, 2, 3, 5])),
case(array!([[1, 2], [3, 4]]), vec![1], array!([4, 5]), None, array!([1, 4, 5, 2, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![0, 1], array!([4]), None, array!([4, 1, 4, 2, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![0, 1], array!([4, 5]), None, array!([4, 1, 5, 2, 3, 4])),
case(array!([1, 2, 3]), vec![1, 3], array!([4, 5, 6]), None, Err(ArrayError::BroadcastShapeMismatch)),
case(array!([1, 2, 3]), vec![1, 3], array!([[4, 5], [4, 5]]), None, Err(ArrayError::UnsupportedDimension { supported: vec![1] })),
case(array!([[1, 2], [3, 4]]), vec![1], array!([4]), Some(0), array!([[1, 2], [4, 4], [3, 4]])),
case(array!([[1, 2], [3, 4]]), vec![2], array!([4]), Some(1), array!([[1, 2, 4], [3, 4, 4]])),
case(array!([[1, 2], [3, 4]]), vec![1], array!([4, 5]), Some(0), array!([[1, 2], [4, 5], [3, 4]])),
case(array!([[1, 2], [3, 4]]), vec![2], array!([4, 5]), Some(1), array!([[1, 2, 4], [3, 4, 5]])),
case(array!([[1, 2], [3, 4]]), vec![2], array!([[4, 5, 6]]), Some(0), Err(ArrayError::BroadcastShapeMismatch)),
case(array!([[1, 2], [3, 4]]), vec![1], array!([[4], [5]]), Some(0), array!([[1, 2], [4, 4], [5, 5], [3, 4]])),
case(array!([[1, 2], [3, 4]]), vec![2], array!([[4], [5]]), Some(1), array!([[1, 2, 4, 5], [3, 4, 4, 5]])),
case(array!([[1, 2], [3, 4]]), vec![1], array!([[4, 5], [6, 7]]), Some(0), array!([[1, 2], [4, 5], [6, 7], [3, 4]])),
case(array!([[1, 2], [3, 4]]), vec![2], array!([[4, 5], [6, 7]]), Some(1), array!([[1, 2, 4, 6], [3, 4, 5, 7]])),
case(array!([[1, 2], [3, 4]]), vec![2], array!([[4, 5, 6], [6, 7, 8]]), Some(1), Err(ArrayError::BroadcastShapeMismatch)),
case(array!([[1, 2], [3, 4]]), vec![2, 2], array!([4]), Some(0), array!([[1, 2], [3, 4], [4, 4], [4, 4]])),
case(array!([[1, 2], [3, 4]]), vec![2, 2], array!([4, 5]), Some(1), array!([[1, 2, 4, 5], [3, 4, 4, 5]])),
case(array!([[1, 2], [3, 4]]), vec![2, 2], array!([[4, 5], [6, 7]]), Some(1), array!([[1, 2, 4, 5], [3, 4, 6, 7]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![1], array!([4]), Some(0), array!([[1, 2, 3], [4, 4, 4], [3, 4, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![2], array!([4]), Some(1), array!([[1, 2, 4, 3], [3, 4, 4, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![1], array!([4, 5, 6]), Some(0), array!([[1, 2, 3], [4, 5, 6], [3, 4, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![2], array!([4, 5]), Some(1), array!([[1, 2, 4, 3], [3, 4, 5, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![2], array!([[4, 5]]), Some(0), Err(ArrayError::BroadcastShapeMismatch)),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![1], array!([[4], [5]]), Some(0), array!([[1, 2, 3], [4, 4, 4], [5, 5, 5], [3, 4, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![2], array!([[4], [5]]), Some(1), array!([[1, 2, 4, 5, 3], [3, 4, 4, 5, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![1], array!([[4, 5], [6, 7]]), Some(0), Err(ArrayError::BroadcastShapeMismatch)),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![1], array!([[4, 5, 6], [6, 7, 8]]), Some(0), array!([[1, 2, 3], [4, 5, 6], [6, 7, 8], [3, 4, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![2], array!([[4, 5], [6, 7]]), Some(1), array!([[1, 2, 4, 6, 3], [3, 4, 5, 7, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![2, 2], array!([4]), Some(0), array!([[1, 2, 3], [3, 4, 5], [4, 4, 4], [4, 4, 4]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![2, 2], array!([4, 5]), Some(1), array!([[1, 2, 4, 5, 3], [3, 4, 4, 5, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), vec![2, 2], array!([[4, 5], [6, 7]]), Some(1), array!([[1, 2, 4, 5, 3], [3, 4, 6, 7, 5]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([4, 5]), Some(0), array!([[[1, 2], [3, 4]], [[4, 5], [4, 5]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([4, 5]), Some(1), array!([[[1, 2], [4, 5], [3, 4]], [[1, 2], [4, 5], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([4, 5]), Some(2), array!([[[1, 4, 2], [3, 5, 4]], [[1, 4, 2], [3, 5, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([[4, 5], [6, 7]]), Some(0), array!([[[1, 2], [3, 4]], [[4, 5], [6, 7]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([[4, 5], [6, 7]]), Some(1), array!([[[1, 2], [4, 5], [3, 4]], [[1, 2], [6, 7], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([[4, 5], [6, 7]]), Some(2), array!([[[1, 4, 2], [3, 5, 4]], [[1, 6, 2], [3, 7, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([4]), Some(0), array!([[[4, 4], [4, 4]], [[1, 2], [3, 4]], [[4, 4], [4, 4]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([4]), Some(1), array!([[[4, 4], [1, 2], [4, 4], [3, 4]], [[4, 4], [1, 2], [4, 4], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([4]), Some(2), array!([[[4, 1, 4, 2], [4, 3, 4, 4]], [[4, 1, 4, 2], [4, 3, 4, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([4, 5]), Some(0), array!([[[4, 5], [4, 5]], [[1, 2], [3, 4]], [[4, 5], [4, 5]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([4, 5]), Some(1), array!([[[4, 5], [1, 2], [4, 5], [3, 4]], [[4, 5], [1, 2], [4, 5], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([4, 5]), Some(2), array!([[[4, 1, 5, 2], [4, 3, 5, 4]], [[4, 1, 5, 2], [4, 3, 5, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([[4, 5], [6, 7]]), Some(0), array!([[[4, 5], [6, 7]], [[1, 2], [3, 4]], [[4, 5], [6, 7]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([[4, 5], [6, 7]]), Some(1), array!([[[4, 5], [1, 2], [6, 7], [3, 4]], [[4, 5], [1, 2], [6, 7], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([[4, 5], [6, 7]]), Some(2), array!([[[4, 1, 5, 2], [6, 3, 7, 4]], [[4, 1, 5, 2], [6, 3, 7, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([[4, 5, 3], [6, 7, 3]]), Some(1), Err(ArrayError::BroadcastShapeMismatch)),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1, 2], array!([[4, 5], [6, 7]]), Some(2), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_insert(array: Result<Array<i32>, ArrayError>, indices: Vec<usize>, values: Result<Array<i32>, ArrayError>, axis: Option<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.insert(indices, &values.unwrap(), axis))
}

#[rstest(
array, indices, axis, expected,
case(array!([1, 2, 3]), vec![1], None, array!([1, 3])),
case(array!([1, 2, 3]), vec![0, 2], None, array!([2])),
case(array!([[1, 2], [3, 4]]), vec![1], None, array!([1, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![4], None, Err(ArrayError::OutOfBounds { value: "index" })),
case(array!([[1, 2], [3, 4]]), vec![1], Some(0), array!([[1, 2]])),
case(array!([[1, 2], [3, 4]]), vec![1], Some(1), array!([[1], [3]])),
case(array!([[1, 2, 3, 4], [3, 4, 5, 6]]), vec![0, 1], Some(0), Array::empty().reshape(vec![0, 4])),
case(array!([[1, 2, 3, 4], [3, 4, 5, 6]]), vec![1], Some(0), array!([[1, 2, 3, 4]])),
case(array!([[1, 2, 3, 4], [3, 4, 5, 6]]), vec![1], Some(1), array!([[1, 3, 4], [3, 5, 6]])),
case(array!([[1, 2, 3, 4], [3, 4, 5, 6]]), vec![1, 2], Some(1), array!([[1, 4], [3, 6]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], Some(0), Array::empty().reshape(vec![0, 2, 2])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], Some(0), array!([[[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], Some(1), array!([[[1, 2]], [[1, 2]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], Some(2), Array::empty().reshape(vec![2, 2, 0])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], Some(2), array!([[[1], [3]], [[1], [3]]])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![0, 1], Some(0), Array::empty().reshape(vec![0, 2, 4])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![1], Some(0), array!([[[1, 2, 3, 4], [3, 4, 5, 6]]])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![0, 1], Some(1), Array::empty().reshape(vec![2, 0, 4])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![1], Some(1), array!([[[1, 2, 3, 4]], [[2, 3, 4, 5]]])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![1], Some(2), array!([[[1, 3, 4], [3, 5, 6]], [[2, 4, 5], [5, 7, 8]]])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![0, 1], Some(2), array!([[[3, 4], [5, 6]], [[4, 5], [7, 8]]])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![1, 3], Some(2), array!([[[1, 3], [3, 5]], [[2, 4], [5, 7]]])),
)] fn test_delete(array: Result<Array<i32>, ArrayError>, indices: Vec<usize>, axis: Option<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.delete(indices, axis))
}

#[rstest(
array, values, axis, expected,
case(array!([1, 2, 3]), array!([1]), None, array!([1, 2, 3, 1])),
case(array!([1, 2, 3]), array!([0, 2]), None, array!([1, 2, 3, 0, 2])),
case(array!([[1, 2], [3, 4]]), array!([1]), None, array!([1, 2, 3, 4, 1])),
case(array!([[1, 2], [3, 4]]), array!([4, 5]), None, array!([1, 2, 3, 4, 4, 5])),
case(array!([[1, 2], [3, 4]]), array!([4]), Some(0), Err(ArrayError::ParameterError { param: "values", message: "input array should have the same dimension as the original one", })),
case(array!([[1, 2], [3, 4]]), array!([[1, 2]]), Some(0), array!([[1, 2], [3, 4], [1, 2]])),
case(array!([[1, 2], [3, 4]]), array!([[1], [2]]), Some(1), array!([[1, 2, 1], [3, 4, 2]])),
case(array!([[1, 2], [3, 4]]), array!([[1, 2]]), Some(1), Err(ArrayError::ParameterError { param: "axis", message: "input array dimensions for the concatenation axis must match exactly", })),
case(array!([[1, 2], [3, 4]]), array!([[1, 2], [3, 4]]), Some(0), array!([[1, 2], [3, 4], [1, 2], [3, 4]])),
case(array!([[1, 2], [3, 4]]), array!([[1, 2], [3, 4]]), Some(1), array!([[1, 2, 1, 2], [3, 4, 3, 4]])),
case(array!([[[1], [2], [3]]]), array!([[[4], [5], [6]]]), Some(2), array!([[[1, 4], [2, 5], [3, 6]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]]]), Some(0), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), Some(1), array!([[[1, 2], [3, 4], [1, 2], [3, 4]], [[1, 2], [3, 4], [1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), Some(2), array!([[[1, 2, 1, 2], [3, 4, 3, 4]], [[1, 2, 1, 2], [3, 4, 3, 4]]])),
case(array!([[1, 2], [3, 4]]), array!([[[1, 1], [1, 1]], [[1, 1], [1, 1]]]), Some(0), Err(ArrayError::ParameterError { param: "values", message: "input array should have the same dimension as the original one", })),
case(array!([[[1, 2], [3, 4]], [[4, 5], [6, 7]]]), array!([[[7, 8]]]), Some(0), Err(ArrayError::ParameterError { param: "axis", message: "input array dimensions for the concatenation axis must match exactly", })),
)] fn test_append(array: Result<Array<i32>, ArrayError>, values: Result<Array<i32>, ArrayError>, axis: Option<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.append(&values.unwrap(), axis))
}

#[rstest(
array, new_shape, expected,
case(array!([1, 2, 3, 4]), vec![2, 2], array!([[1, 2], [3, 4]])),
case(array!([[1, 2], [3, 4]]), vec![4], array!([1, 2, 3, 4])),
case(array!([[1, 2, 3], [4, 5, 6]]), vec![3, 2], array!([[1, 2], [3, 4], [5, 6]])),
case(array!([[1, 2], [3, 4], [5, 6]]), vec![6], array!([1, 2, 3, 4, 5, 6])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![2, 4], array!([[1, 2, 3, 4], [5, 6, 7, 8]])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![8], array!([1, 2, 3, 4, 5, 6, 7, 8])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![10], Err(ArrayError::ShapeMustMatchValuesLength)),
)] fn test_reshape(array: Result<Array<i32>, ArrayError>, new_shape: Vec<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.reshape(new_shape))
}

#[rstest(
array, new_shape, expected,
case(array!([[1, 2], [3, 4]]), vec![2], array!([1, 2])),
case(array!([[1, 2], [3, 4]]), vec![4], array!([1, 2, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![8], array!([1, 2, 3, 4, 1, 2, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![2, 4], array!([[1, 2, 3, 4], [1, 2, 3, 4]])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![2], array!([1, 2])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![8], array!([1, 2, 3, 4, 5, 6, 7, 8])),
)] fn test_resize(array: Result<Array<i32>, ArrayError>, new_shape: Vec<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.resize(new_shape))
}

#[rstest(
array, axis, expected,
case(array!([1, 2, 3, 4, 5]), None, array!([1, 2, 3, 4, 5])),
case(array!([1, 2, 3, 4, 5, 3, 2, 1]), None, array!([1, 2, 3, 4, 5])),
case(array!([[1, 2], [3, 4]]), None, array!([1, 2, 3, 4])),
case(array!([[1, 2], [3, 4], [1, 2]]), None, array!([1, 2, 3, 4])),
case(array!([[1, 2], [3, 4], [1, 2]]), Some(0), array!([[1, 2], [3, 4]])),
case(array!([[1, 2], [3, 4], [1, 2]]), Some(1), array!([[1, 2], [3, 4], [1, 2]])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), None, array!([1, 2, 3, 4, 5, 6, 7, 8])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[1, 2], [3, 4]]]), None, array!([1, 2, 3, 4, 5, 6, 7, 8])),
case(array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]]), None, array!([1, 2, 3, 4, 5, 6, 7, 8])),
case(array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]]), Some(0), array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]])),
case(array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]]), Some(1), array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])),
case(array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]]), Some(2), array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]])),
case(array!([[[1, 1], [3, 3], [1, 1]], [[2, 2], [4, 4], [2, 2]]]), Some(2), array!([[[1], [3], [1]], [[2], [4], [2]]])),
)] fn test_unique(array: Result<Array<i32>, ArrayError>, axis: Option<isize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.unique(axis))
}

#[rstest(
array, expected,
case(array!([[1, 2], [3, 4]]), vec![4]),
case(array!([[1, 2], [3, 4], [5, 6]]), vec![6]),
case(array!([[1, 2, 3], [4, 5, 6]]), vec![6]),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![8]),
)] fn test_ravel(array: Result<Array<i32>, ArrayError>, expected: Vec<usize>) {
    assert_eq!(expected, array.ravel().get_shape().unwrap())
}

#[rstest(
arr, dim, expected,
case(array!([1]), 0, array!([1])),
case(array!([1]), 1, array!([1])),
case(array!([1, 2]), 2, array!([[1, 2]])),
case(array!([1, 2]), 3, array!([[[1], [2]]])),
case(array!([[[1], [2]]]), 3, array!([[[1], [2]]])),
case(array!([[[2, 2], [2, 2]], [[2, 2], [2, 2]]]), 4, Err(ArrayError::UnsupportedDimension { supported: vec![0, 1, 2, 3] })),
)] fn test_atleast(arr: Result<Array<i32>, ArrayError>, dim: usize, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, arr.atleast(dim))
}

#[rstest(
arr, expected,
case(array!([1, 2, 3, 4]), array!([1, 2, 3, 4])),
case(array!([0, 0, 1, 2, 3, 4]), array!([1, 2, 3, 4])),
case(array!([0, 0, 1, 2, 3, 4]), array!([1, 2, 3, 4])),
case(array!([0, 0, 1, 2, 3, 4, 0, 0]), array!([1, 2, 3, 4])),
case(array!([0, 0, 0, 0]), Array::empty()),
case(array!([[1, 2]]), Err(ArrayError::UnsupportedDimension { supported: vec![1] })),
)] fn test_trim_zeros(arr: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, arr.trim_zeros())
}

#[rstest(
arr, n, expected,
case(array!([1, 2, 3, 4]), 0, Array::empty()),
case(array!([1, 2, 3, 4]), 2, array!([1, 2])),
case(array!([1, 2, 3, 4]), 4, array!([1, 2, 3, 4])),
case(array!([[1, 2], [3, 4]]), 2, array!([1, 2])),
case(array!([[1, 2], [3, 4]]), 4, array!([1, 2, 3, 4])),
)] fn test_cycle_take(arr: Result<Array<i32>, ArrayError>, n: usize, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, arr.cycle_take(n))
}

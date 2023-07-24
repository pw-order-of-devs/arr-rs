use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], array!([1, 2, 3, 4])),
case(vec![1, 2, 3, 4], vec![2, 2], array!([[1, 2], [3, 4]])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2], array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![10], Err(ArrayError::ShapeMustMatchValuesLength)),
)] fn test_new(elements: Vec<i32>, shape: Vec<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, Array::new(elements, shape))
}

#[rstest(
elements, shape, ndmin, expected,
case(vec![1, 2, 3, 4], vec![4], None, array!([1, 2, 3, 4])),
case(vec![1, 2, 3, 4], vec![2, 2], Some(2), array!([[1, 2], [3, 4]])),
case(vec![1, 2, 3, 4], vec![2, 2], Some(3), array!([[[1, 2], [3, 4]]])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2], Some(2), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2], Some(3), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2], Some(5), array!([[[[[1, 2], [3, 4]], [[1, 2], [3, 4]]]]])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![10], None, Err(ArrayError::ShapeMustMatchValuesLength)),
)] fn test_create(elements: Vec<i32>, shape: Vec<usize>, ndmin: Option<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, Array::create(elements, shape, ndmin))
}

#[rstest(
element, expected,
case(2, array!([2])),
case(4, array!([4])),
)] fn test_single(element: i32, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, Array::single(element))
}

#[rstest(
elements, expected,
case(vec![1, 2, 3, 4], array!([1, 2, 3, 4])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], array!([1, 2, 3, 4, 1, 2, 3, 4])),
)] fn test_flat(elements: Vec<i32>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, Array::flat(elements))
}

#[rstest(
array, expected,
case(Array::empty(), Array::new(vec![], vec![0])),
)] fn test_empty(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array)
}

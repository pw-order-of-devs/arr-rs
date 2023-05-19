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
shape, expected,
case(vec![4], 4),
case(vec![4, 4], 16),
case(vec![4, 4, 4], 64),
)] fn test_rand(shape: Vec<usize>, expected: usize) {
    assert_eq!(expected, Array::<f64>::rand(shape).len().unwrap())
}

#[rstest(
array, expected,
case(Array::empty(), Array::new(vec![], vec![0])),
)] fn test_empty(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array)
}

#[rstest(
n, m, k, expected,
case(2, Some(3), Some(0), array!([[1., 0., 0.], [0., 1., 0.]])),
case(3, Some(2), Some(1), array!([[0., 1.], [0., 0.], [0., 0.]])),
case(4, Some(3), Some(0), array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [0., 0., 0.]])),
case(4, Some(3), Some(1), array!([[0., 1., 0.], [0., 0., 1.], [0., 0., 0.], [0., 0., 0.]])),
)] fn test_eye(n: usize, m: Option<usize>, k: Option<usize>, expected: Result<Array<f64>, ArrayError>) {
     assert_eq!(expected, Array::<f64>::eye(n, m, k))
}

#[rstest(
n, expected,
case(2, array!([[1., 0.], [0., 1.]])),
case(3, array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])),
case(4, array!([[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]])),
)] fn test_identity(n: usize, expected: Result<Array<f64>, ArrayError>) {
     assert_eq!(expected, Array::<f64>::identity(n))
}

#[rstest(
shape, expected,
case(vec![4], array!([0, 0, 0, 0])),
case(vec![2, 2], array!([[0, 0], [0, 0]])),
)] fn test_zeros(shape: Vec<usize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::zeros(shape))
}

#[rstest(
other, expected,
case(array!(1, 2, 3, 4), array!([0., 0., 0., 0.])),
case(array!([[1, 2], [1, 2]]), array!([[0., 0.], [0., 0.]])),
)] fn test_zeros_like(other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::zeros_like(&other.unwrap()))
}

#[rstest(
shape, expected,
case(vec![4], array!([1., 1., 1., 1.])),
case(vec![2, 2], array!([[1., 1.], [1., 1.]])),
)] fn test_ones(shape: Vec<usize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::ones(shape))
}

#[rstest(
other, expected,
case(array!(1, 2, 3, 4), array!([1., 1., 1., 1.])),
case(array!([[1, 2], [1, 2]]), array!([[1., 1.], [1., 1.]])),
)] fn test_ones_like(other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::ones_like(&other.unwrap()))
}

#[rstest(
shape, fill_value, expected,
case(vec![4], 2., array!([2., 2., 2., 2.])),
case(vec![2, 2], 2., array!([[2., 2.], [2., 2.]])),
)] fn test_full(shape: Vec<usize>, fill_value: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::full(shape, fill_value))
}

#[rstest(
other, fill_value, expected,
case(array!(1, 2, 3, 4), 2., array!([2., 2., 2., 2.])),
case(array!([[1, 2], [1, 2]]), 2., array!([[2., 2.], [2., 2.]])),
)] fn test_full_like(other: Result<Array<f64>, ArrayError>, fill_value: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::full_like(&other.unwrap(), fill_value))
}

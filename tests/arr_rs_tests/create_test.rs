use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], array!([1, 2, 3, 4])),
case(vec![1, 2, 3, 4], vec![2, 2], array!([[1, 2], [3, 4]])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2], array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
#[should_panic(expected = "Shape must match values length")]
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![10], array!([1])),
)] fn test_new(elements: Vec<i32>, shape: Vec<usize>, expected: Array<i32>) {
    assert_eq!(expected, Array::new(elements, shape))
}

#[rstest(
elements, expected,
case(vec![1, 2, 3, 4], array!([1, 2, 3, 4])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], array!([1, 2, 3, 4, 1, 2, 3, 4])),
)] fn test_flat(elements: Vec<i32>, expected: Array<i32>) {
    assert_eq!(expected, Array::flat(elements))
}

#[rstest(
shape, expected,
case(vec![4], 4),
case(vec![4, 4], 16),
case(vec![4, 4, 4], 64),
)] fn test_rand(shape: Vec<usize>, expected: usize) {
    assert_eq!(expected, Array::<f64>::rand(shape).len())
}

#[rstest(
array, expected,
case(Array::empty(), Array::new(vec![], vec![0])),
)] fn test_empty(array: Array<f64>, expected: Array<f64>) {
    assert_eq!(expected, array)
}

#[rstest(
n, m, k, expected,
case(2, Some(3), Some(0), array!([[1., 0., 0.], [0., 1., 0.]])),
case(3, Some(2), Some(1), array!([[0., 1.], [0., 0.], [0., 0.]])),
case(4, Some(3), Some(0), array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [0., 0., 0.]])),
case(4, Some(3), Some(1), array!([[0., 1., 0.], [0., 0., 1.], [0., 0., 0.], [0., 0., 0.]])),
)] fn test_eye(n: usize, m: Option<usize>, k: Option<usize>, expected: Array<f64>) {
     assert_eq!(expected, Array::<f64>::eye(n, m, k))
}

#[rstest(
n, expected,
case(2, array!([[1., 0.], [0., 1.]])),
case(3, array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])),
case(4, array!([[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]])),
)] fn test_identity(n: usize, expected: Array<f64>) {
     assert_eq!(expected, Array::<f64>::identity(n))
}

#[rstest(
shape, expected,
case(vec![4], array!([0, 0, 0, 0])),
case(vec![2, 2], array!([[0, 0], [0, 0]])),
)] fn test_zeros(shape: Vec<usize>, expected: Array<f64>) {
    assert_eq!(expected, Array::<f64>::zeros(shape))
}

#[rstest(
shape, expected,
case(vec![4], array!([1., 1., 1., 1.])),
case(vec![2, 2], array!([[1., 1.], [1., 1.]])),
)] fn test_ones(shape: Vec<usize>, expected: Array<f64>) {
    assert_eq!(expected, Array::<f64>::ones(shape))
}

#[rstest(
shape, fill_value, expected,
case(vec![4], 2., array!([2., 2., 2., 2.])),
case(vec![2, 2], 2., array!([[2., 2.], [2., 2.]])),
)] fn test_full(shape: Vec<usize>, fill_value: f64, expected: Array<f64>) {
    assert_eq!(expected, Array::<f64>::full(shape, fill_value))
}

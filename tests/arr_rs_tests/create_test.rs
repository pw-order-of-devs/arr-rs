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

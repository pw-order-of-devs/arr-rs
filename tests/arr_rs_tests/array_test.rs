use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], "[1, 2, 3, 4]"),
case(vec![1, 2, 3, 4], vec![2, 2], "[[1, 2], [3, 4]]"),
)] fn test_new(elements: Vec<i32>, shape: Vec<usize>, expected: &str) {
    assert_eq!(expected, format!("{}", Array::new(elements, shape)))
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
case(Array::empty(), "[]"),
)] fn test_empty(array: Array<f64>, expected: &str) {
    assert_eq!(expected, format!("{array}"))
}

#[rstest(
shape, expected,
case(vec![4], "[0, 0, 0, 0]"),
case(vec![2, 2], "[[0, 0], [0, 0]]"),
)] fn test_zeros(shape: Vec<usize>, expected: &str) {
    let arr: Array<f64> = Array::zeros(shape);
    assert_eq!(expected, format!("{arr}"))
}

#[rstest(
shape, expected,
case(vec![4], "[1, 1, 1, 1]"),
case(vec![2, 2], "[[1, 1], [1, 1]]"),
)] fn test_ones(shape: Vec<usize>, expected: &str) {
    let arr: Array<f64> = Array::ones(shape);
    assert_eq!(expected, format!("{arr}"))
}

#[rstest(
array, new_shape, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 2], "[[1, 2], [3, 4]]"),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), vec![4], "[1, 2, 3, 4]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), vec![3, 2], "[[1, 2, 3], [4, 5, 6]]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), vec![6], "[1, 2, 3, 4, 5, 6]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), vec![2, 4], "[[1, 2], [3, 4], [5, 6], [7, 8]]"),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), vec![8], "[1, 2, 3, 4, 5, 6, 7, 8]"),
)] fn test_reshape(array: Array<i32>, new_shape: Vec<usize>, expected: &str) {
    assert_eq!(expected, format!("{}", array.reshape(new_shape)))
}

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], 24),
case(vec![1, 2, 3, 4], vec![2, 2], 24),
)] fn test_product(elements: Vec<i32>, shape: Vec<usize>, expected: i32) {
    assert_eq!(expected, Array::new(elements, shape).product())
}

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], 10),
case(vec![1, 2, 3, 4], vec![2, 2], 10),
)] fn test_sum(elements: Vec<i32>, shape: Vec<usize>, expected: i32) {
    assert_eq!(expected, Array::new(elements, shape).sum())
}

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], 1),
case(vec![1, 2, 3, 4], vec![2, 2], 2),
)] fn test_ndim(elements: Vec<i32>, shape: Vec<usize>, expected: usize) {
    assert_eq!(expected, Array::new(elements, shape).ndim())
}

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], 4),
case(vec![1, 2, 3, 4], vec![2, 2], 4),
)] fn test_len(elements: Vec<i32>, shape: Vec<usize>, expected: usize) {
    assert_eq!(expected, Array::new(elements, shape).len())
}

#[rstest(
elements, shape, expected,
case(vec![], vec![0], true),
case(vec![1, 2, 3, 4], vec![2, 2], false),
)] fn test_is_empty(elements: Vec<i32>, shape: Vec<usize>, expected: bool) {
    assert_eq!(expected, Array::new(elements, shape).is_empty())
}

#[rstest(
array, coords, expected,
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[0, 0], 0),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[1, 1], 3),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), &[1, 1], 4),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), &[2, 0], 4),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[0, 0, 1], 1),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[1, 1, 1], 7),
)] fn test_index(array: Array<i32>, coords: &[usize], expected: usize) {
    assert_eq!(expected, array.index(coords))
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), vec![4]),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), vec![6]),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), vec![6]),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), vec![8]),
)] fn test_ravel(array: Array<i32>, expected: Vec<usize>) {
    assert_eq!(expected, array.ravel().get_shape())
}

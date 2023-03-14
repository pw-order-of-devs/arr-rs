use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], "Array { elements: [1, 2, 3, 4], shape: [4] }"),
case(vec![1, 2, 3, 4], vec![2, 2], "Array { elements: [1, 2, 3, 4], shape: [2, 2] }"),
)] fn test_new(elements: Vec<i32>, shape: Vec<usize>, expected: &str) {
    assert_eq!(expected, format!("{:?}", Array::new(elements, shape)))
}

#[test] fn test_empty() {
    let arr: Array<f64> = Array::empty();
    assert_eq!("Array { elements: [], shape: [0] }", format!("{arr:?}"))
}

#[rstest(
shape, expected,
case(vec![4], "Array { elements: [0.0, 0.0, 0.0, 0.0], shape: [4] }"),
case(vec![2, 2], "Array { elements: [0.0, 0.0, 0.0, 0.0], shape: [2, 2] }"),
)] fn test_zeros(shape: Vec<usize>, expected: &str) {
    let arr: Array<f64> = Array::zeros(shape);
    assert_eq!(expected, format!("{arr:?}"))
}

#[rstest(
shape, expected,
case(vec![4], "Array { elements: [1.0, 1.0, 1.0, 1.0], shape: [4] }"),
case(vec![2, 2], "Array { elements: [1.0, 1.0, 1.0, 1.0], shape: [2, 2] }"),
)] fn test_ones(shape: Vec<usize>, expected: &str) {
    let arr: Array<f64> = Array::ones(shape);
    assert_eq!(expected, format!("{arr:?}"))
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
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), vec![4]),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), vec![6]),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), vec![6]),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), vec![8]),
)] fn test_ravel(array: Array<i32>, expected: Vec<usize>) {
    println!("{:?}", array.ravel());
    assert_eq!(expected, array.ravel().get_shape())
}

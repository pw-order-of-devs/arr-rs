use rstest::rstest;
use arr_rs::prelude::*;

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
array, expected,
case(array!([1, 2, 3, 4]), array!([1, 3, 6, 10])),
case(array!([[1, 2], [3, 4]]), array!([1, 3, 6, 10])),
)] fn test_cumsum(array: Array<i32>, expected: Array<i32>) {
    assert_eq!(expected, array.cumsum())
}

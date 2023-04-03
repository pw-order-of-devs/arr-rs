use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![1, 2, 3, 4]),
case(array!([[1, 2], [3, 4]]), vec![1, 2, 3, 4]),
)] fn test_get_elements(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.get_elements())
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![4]),
case(array!([[1, 2], [3, 4]]), vec![2, 2]),
)] fn test_get_shape(array: Array<i32>, expected: Vec<usize>) {
    assert_eq!(expected, array.get_shape())
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

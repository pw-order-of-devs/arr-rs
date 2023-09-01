use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array!(i32, [1, 2, 3, 4]), vec![1, 2, 3, 4]),
case(array!(i32, [[1, 2], [3, 4]]), vec![1, 2, 3, 4]),
)] fn test_get_elements(array: Result<Array<i32>, ArrayError>, expected: Vec<i32>) {
    assert_eq!(expected, array.get_elements().unwrap())
}

#[rstest(
array, expected,
case(array!(i32, [1, 2, 3, 4]), vec![4]),
case(array!(i32, [[1, 2], [3, 4]]), vec![2, 2]),
)] fn test_get_shape(array: Result<Array<i32>, ArrayError>, expected: Vec<usize>) {
    assert_eq!(expected, array.get_shape().unwrap())
}

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], 1),
case(vec![1, 2, 3, 4], vec![2, 2], 2),
)] fn test_ndim(elements: Vec<i32>, shape: Vec<usize>, expected: usize) {
    assert_eq!(expected, Array::new(elements, shape).ndim().unwrap())
}

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], 4),
case(vec![1, 2, 3, 4], vec![2, 2], 4),
)] fn test_len(elements: Vec<i32>, shape: Vec<usize>, expected: usize) {
    assert_eq!(expected, Array::new(elements, shape).len().unwrap())
}

#[rstest(
elements, shape, expected,
case(vec![], vec![0], true),
case(vec![1, 2, 3, 4], vec![2, 2], false),
)] fn test_is_empty(elements: Vec<i32>, shape: Vec<usize>, expected: bool) {
    assert_eq!(expected, Array::new(elements, shape).is_empty().unwrap())
}

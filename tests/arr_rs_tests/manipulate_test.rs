use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, new_shape, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 2], array!([[1, 2], [3, 4]])),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), vec![4], array!([1, 2, 3, 4])),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), vec![3, 2], array!([[1, 2], [3, 4], [5, 6]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), vec![6], array!([1, 2, 3, 4, 5, 6])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), vec![2, 4], array!([[1, 2, 3, 4], [5, 6, 7, 8]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), vec![8], array!([1, 2, 3, 4, 5, 6, 7, 8])),
#[should_panic(expected = "Shape must match values length")]
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), vec![10], array!([1])),
)] fn test_reshape(array: Array<i32>, new_shape: Vec<usize>, expected: Array<i32>) {
    assert_eq!(expected, array.reshape(new_shape))
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

#[rstest(
start, stop, shape, expected,
case(1, 5, vec![4], array!([1, 2, 3, 4])),
case(1, 7, vec![2, 3], array!([[1, 4], [2, 5], [3, 6]])),
case(1, 9, vec![2, 2, 2], array!([[[1, 5], [3, 7]], [[2, 6], [4, 8]]])),
case(1, 17, vec![2, 2, 2, 2], array!([[[[1, 9], [5, 13]], [[3, 11], [7, 15]]], [[[2, 10], [6, 14]], [[4, 12], [8, 16]]]])),
)] fn test_transpose(start: i32, stop: i32, shape: Vec<usize>, expected: Array<i32>) {
    assert_eq!(expected, Array::arange(start, stop, None).reshape(shape).transpose())
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 4, 6, 8]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
)] fn test_for_each(array: Array<i32>, expected: Vec<i32>) {
    let mut items = vec![];
    array.for_each(|item| items.push(*item * 2));
    assert_eq!(expected, items);
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![(0, 2), (1, 4), (2, 6), (3, 8)]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![(0, 2), (1, 4), (2, 6), (3, 8), (4, 2), (5, 4), (6, 6), (7, 8)]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![(0, 2), (1, 4), (2, 6), (3, 8), (4, 2), (5, 4), (6, 6), (7, 8)]),
)] fn test_for_each_enumerated(array: Array<i32>, expected: Vec<(usize, i32)>) {
    let mut items = vec![];
    array.for_each_e(|idx, item| items.push((idx, *item * 2)));
    assert_eq!(expected, items);
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 4, 6, 8]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
)] fn test_map(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.map(|item| *item * 2).get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![0, 2, 6, 12]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![0, 2, 6, 12, 4, 10, 18, 28]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![0, 2, 6, 12, 4, 10, 18, 28]),
)] fn test_map_e(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.map_e(|idx, item| *item * idx as i32).get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![2, 4, 2, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![2, 4, 2, 4]),
)] fn test_filter(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.filter(|item| item % 2 == 0).get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![1, 2, 3, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![1, 2, 3, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![1, 2, 3, 4]),
)] fn test_filter_e(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.filter_e(|idx, item| item % (idx + 1) as i32 == 0).get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![2, 4, 2, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![2, 4, 2, 4]),
)] fn test_filter_map(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.filter_map(|item|
        if item % 2 == 0 { Some(*item) } else { None }
    ).get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![1, 2, 3, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![1, 2, 3, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![1, 2, 3, 4]),
)] fn test_filter_map_e(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.filter_map_e(|idx, item|
        if item % (idx + 1) as i32 == 0 { Some(*item) } else { None }
    ).get_elements());
}

#[rstest(
array, init, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), 0, 10),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), 0, 20),
)] fn test_fold_sum(array: Array<i32>, init: i32, expected: i32) {
    assert_eq!(expected, array.fold(init, |a, b| a + b));
}

#[rstest(
array, init, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), 1, 24),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), 1, 576),
)] fn test_fold_mul(array: Array<i32>, init: i32, expected: i32) {
    assert_eq!(expected, array.fold(init, |a, b| a * b));
}

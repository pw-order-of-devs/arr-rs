use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), "[3, 6, 9, 12]"),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), "[[3, 6], [9, 12]]"),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![8]), "[3, 6, 9, 12, 3, 6, 9, 12]"),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), "[[3, 6], [9, 12], [3, 6], [9, 12]]"),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), "[[[3, 6], [9, 12]], [[3, 6], [9, 12]]]"),
)] fn test_iter_array(array: Array<i32>, expected: &str) {
    let iterated: Array<i32> = array.clone().into_iter()
        .map(|i| i * 3)
        .collect::<Array<i32>>()
        .reshape(array.get_shape());
    assert_eq!(expected, format!("{iterated}"))
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 4, 6, 8]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
)] fn test_array_for_each(array: Array<i32>, expected: Vec<i32>) {
    let mut items = vec![];
    array.for_each(|item| items.push(*item * 2));
    assert_eq!(expected, items);
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![(0, 2), (1, 4), (2, 6), (3, 8)]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![(0, 2), (1, 4), (2, 6), (3, 8), (4, 2), (5, 4), (6, 6), (7, 8)]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![(0, 2), (1, 4), (2, 6), (3, 8), (4, 2), (5, 4), (6, 6), (7, 8)]),
)] fn test_array_for_each_enumerated(array: Array<i32>, expected: Vec<(usize, i32)>) {
    let mut items = vec![];
    array.for_each_e(|idx, item| items.push((idx, *item * 2)));
    assert_eq!(expected, items);
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 4, 6, 8]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
)] fn test_array_map(array: Array<i32>, expected: Vec<i32>) {
    let arr = array.map(|item| *item * 2);
    assert_eq!(expected, arr.get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![0, 2, 6, 12]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![0, 2, 6, 12, 4, 10, 18, 28]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![0, 2, 6, 12, 4, 10, 18, 28]),
)] fn test_array_map_e(array: Array<i32>, expected: Vec<i32>) {
    let arr = array.map_e(|idx, item| *item * idx as i32);
    assert_eq!(expected, arr.get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![2, 4, 2, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![2, 4, 2, 4]),
)] fn test_array_filter(array: Array<i32>, expected: Vec<i32>) {
    let arr = array.filter(|item| item % 2 == 0);
    assert_eq!(expected, arr.get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![1, 2, 3, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![1, 2, 3, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![1, 2, 3, 4]),
)] fn test_array_filter_e(array: Array<i32>, expected: Vec<i32>) {
    let arr = array.filter_e(|idx, item| item % (idx + 1) as i32 == 0);
    assert_eq!(expected, arr.get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![2, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![2, 4, 2, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![2, 4, 2, 4]),
)] fn test_array_filter_map(array: Array<i32>, expected: Vec<i32>) {
    let arr = array.filter_map(|item|
        if item % 2 == 0 { Some(*item) } else { None }
    );
    assert_eq!(expected, arr.get_elements());
}

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), vec![1, 2, 3, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), vec![1, 2, 3, 4]),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), vec![1, 2, 3, 4]),
)] fn test_array_filter_map_e(array: Array<i32>, expected: Vec<i32>) {
    let arr = array.filter_map_e(|idx, item|
        if item % (idx + 1) as i32 == 0 { Some(*item) } else { None }
    );
    assert_eq!(expected, arr.get_elements());
}

#[rstest(
array, init, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), 0, 10),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), 0, 20),
)] fn test_array_fold_sum(array: Array<i32>, init: i32, expected: i32) {
    assert_eq!(expected, array.fold(init, |a, b| a + b));
}

#[rstest(
array, init, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), 1, 24),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), 1, 576),
)] fn test_array_fold_mul(array: Array<i32>, init: i32, expected: i32) {
    assert_eq!(expected, array.fold(init, |a, b| a * b));
}

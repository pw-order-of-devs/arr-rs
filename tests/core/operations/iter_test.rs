use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), array!([3, 6, 9, 12])),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), array!([[3, 6], [9, 12]])),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![8]), array!([3, 6, 9, 12, 3, 6, 9, 12])),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), array!([[3, 6, 9, 12], [3, 6, 9, 12]])),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), array!([[[3, 6], [9, 12]], [[3, 6], [9, 12]]])),
)] fn test_iter_array(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    let shape = array.clone().unwrap().get_shape().unwrap();
    let iterated = array.unwrap().into_iter()
        .map(|i| i * 3)
        .collect::<Array<i32>>()
        .reshape(&shape);
    assert_eq!(expected, iterated)
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![2, 4, 6, 8]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
)] fn test_for_each(array: Result<Array<i32>, ArrayError>, expected: Vec<i32>) {
    let mut items = vec![];
    array.unwrap().for_each(|item| items.push(*item * 2)).unwrap();
    assert_eq!(expected, items);
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![(0, 2), (1, 4), (2, 6), (3, 8)]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![(0, 2), (1, 4), (2, 6), (3, 8), (4, 2), (5, 4), (6, 6), (7, 8)]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![(0, 2), (1, 4), (2, 6), (3, 8), (4, 2), (5, 4), (6, 6), (7, 8)]),
)] fn test_for_each_e(array: Result<Array<i32>, ArrayError>, expected: Vec<(usize, i32)>) {
    let mut items = vec![];
    array.unwrap().for_each_e(|idx, item| items.push((idx, *item * 2))).unwrap();
    assert_eq!(expected, items);
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![2, 4, 6, 8]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
)] fn test_map(array: Result<Array<i32>, ArrayError>, expected: Vec<i32>) {
    assert_eq!(expected, array.unwrap().map(|item| *item * 2).get_elements().unwrap());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![0, 2, 6, 12]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![0, 2, 6, 12, 4, 10, 18, 28]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 2, 6, 12, 4, 10, 18, 28]),
)] fn test_map_e(array: Result<Array<i32>, ArrayError>, expected: Vec<i32>) {
    assert_eq!(expected, array.unwrap().map_e(|idx, item| *item * idx as i32).get_elements().unwrap());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![2, 4]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![2, 4, 2, 4]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![2, 4, 2, 4]),
)] fn test_filter(array: Result<Array<i32>, ArrayError>, expected: Vec<i32>) {
    assert_eq!(expected, array.unwrap().filter(|item| item % 2 == 0).get_elements().unwrap());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![1, 2, 3, 4]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![1, 2, 3, 4]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1, 2, 3, 4]),
)] fn test_filter_e(array: Result<Array<i32>, ArrayError>, expected: Vec<i32>) {
    assert_eq!(expected, array.unwrap().filter_e(|idx, item| item % (idx + 1) as i32 == 0).get_elements().unwrap());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![2, 4]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![2, 4, 2, 4]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![2, 4, 2, 4]),
)] fn test_filter_map(array: Result<Array<i32>, ArrayError>, expected: Vec<i32>) {
    assert_eq!(expected, array.unwrap().filter_map(|item|
        if item % 2 == 0 { Some(*item) } else { None }
    ).get_elements().unwrap());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![1, 2, 3, 4]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![1, 2, 3, 4]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1, 2, 3, 4]),
)] fn test_filter_map_e(array: Result<Array<i32>, ArrayError>, expected: Vec<i32>) {
    assert_eq!(expected, array.unwrap().filter_map_e(|idx, item|
        if item % (idx + 1) as i32 == 0 { Some(*item) } else { None }
    ).get_elements().unwrap());
}

#[rstest(
array, init, expected,
case(array!([1, 2, 3, 4]), 0, 10),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), 0, 20),
)] fn test_fold_sum(array: Result<Array<i32>, ArrayError>, init: i32, expected: i32) {
    assert_eq!(expected, array.unwrap().fold(init, |a, b| a + b).unwrap());
}

#[rstest(
array, init, expected,
case(array!([1, 2, 3, 4]), 1, 24),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), 1, 576),
)] fn test_fold_mul(array: Result<Array<i32>, ArrayError>, init: i32, expected: i32) {
    assert_eq!(expected, array.unwrap().fold(init, |a, b| a * b).unwrap());
}

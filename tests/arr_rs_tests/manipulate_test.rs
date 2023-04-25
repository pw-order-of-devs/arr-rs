use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, indices, values, axis, expected,
case(array!([1, 2, 3]), vec![1], array!([4]), None, array!([1, 4, 2, 3])),
case(array!([1, 2, 3]), vec![1, 3], array!([4, 5]), None, array!([1, 4, 2, 3, 5])),
case(array!([[1, 2], [3, 4]]), vec![1], array!([4, 5]), None, array!([1, 4, 5, 2, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![0, 1], array!([4]), None, array!([4, 1, 4, 2, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![0, 1], array!([4, 5]), None, array!([4, 1, 5, 2, 3, 4])),
#[should_panic(expected = "incompatible shapes for broadcasting")]
case(array!([1, 2, 3]), vec![1, 3], array!([4, 5, 6]), None, array!([1, 4, 2, 3, 5, 6])),
#[should_panic(expected = "values and indices don't match for insert")]
case(array!([1, 2, 3]), vec![1, 3], array!([[4, 5], [4, 5]]), None, array!([1, 4, 2, 3, 5])),
case(array!([[1, 2], [3, 4]]), vec![1], array!([4, 5]), Some(0), array!([[1, 2], [4, 5], [3, 4]])),
case(array!([[1, 2], [3, 4]]), vec![1], array!([[4, 5], [6, 7]]), Some(0), array!([[1, 2], [4, 5], [6, 7], [3, 4]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([[4, 5], [6, 7]]), Some(0), array!([[[1, 2], [3, 4]], [[4, 5], [6, 7]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([[4, 5], [6, 7]]), Some(1), array!([[[1, 2], [4, 5], [3, 4]], [[1, 2], [6, 7], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([[4, 5], [6, 7]]), Some(2), array!([[[1, 4, 2], [3, 5, 4]], [[1, 6, 2], [3, 7, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([4]), Some(0), array!([[[4, 4], [4, 4]], [[1, 2], [3, 4]], [[4, 4], [4, 4]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([4]), Some(1), array!([[[4, 4], [1, 2], [4, 4], [3, 4]], [[4, 4], [1, 2], [4, 4], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([4]), Some(2), array!([[[4, 1, 4, 2], [4, 3, 4, 4]], [[4, 1, 4, 2], [4, 3, 4, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([[4, 5], [6, 7]]), Some(0), array!([[[4, 5], [6, 7]], [[1, 2], [3, 4]], [[4, 5], [6, 7]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([[4, 5], [6, 7]]), Some(1), array!([[[4, 5], [1, 2], [6, 7], [3, 4]], [[4, 5], [1, 2], [6, 7], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 1], array!([[4, 5], [6, 7]]), Some(2), array!([[[4, 1, 5, 2], [6, 3, 7, 4]], [[4, 1, 5, 2], [6, 3, 7, 4]]])),
#[should_panic(expected = "incompatible shapes for broadcasting")]
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], array!([[4, 5, 3], [6, 7, 3]]), Some(1), array!([1])),
)] fn test_insert(array: Array<i32>, indices: Vec<usize>, values: Array<i32>, axis: Option<usize>, expected: Array<i32>) {
    assert_eq!(expected, array.insert(indices, &values, axis))
}

#[rstest(
array, indices, axis, expected,
case(array!([1, 2, 3]), vec![1], None, array!([1, 3])),
case(array!([1, 2, 3]), vec![0, 2], None, array!([2])),
case(array!([[1, 2], [3, 4]]), vec![1], None, array!([1, 3, 4])),
#[should_panic(expected = "delete index out of the bounds of array")]
case(array!([[1, 2], [3, 4]]), vec![4], None, array!([1, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![1], Some(0), array!([[1, 2]])),
case(array!([[1, 2], [3, 4]]), vec![1], Some(1), array!([[1], [3]])),
case(array!([[1, 2, 3, 4], [3, 4, 5, 6]]), vec![1], Some(0), array!([[1, 2, 3, 4]])),
case(array!([[1, 2, 3, 4], [3, 4, 5, 6]]), vec![1], Some(1), array!([[1, 3, 4], [3, 5, 6]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], Some(0), array!([[[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], Some(1), array!([[[1, 2]], [[1, 2]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1], Some(2), array!([[[1], [3]], [[1], [3]]])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![1], Some(0), array!([[[1, 2, 3, 4], [3, 4, 5, 6]]])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![1], Some(1), array!([[[1, 2, 3, 4]], [[2, 3, 4, 5]]])),
case(array!([[[1, 2, 3, 4], [3, 4, 5, 6]], [[2, 3, 4, 5], [5, 6, 7, 8]]]), vec![1], Some(2), array!([[[1, 3, 4], [3, 5, 6]], [[2, 4, 5], [5, 7, 8]]])),
)] fn test_delete(array: Array<i32>, indices: Vec<usize>, axis: Option<usize>, expected: Array<i32>) {
    assert_eq!(expected, array.delete(indices, axis))
}

#[rstest(
array, values, axis, expected,
case(array!([1, 2, 3]), array!([1]), None, array!([1, 2, 3, 1])),
case(array!([1, 2, 3]), array!([0, 2]), None, array!([1, 2, 3, 0, 2])),
case(array!([[1, 2], [3, 4]]), array!([1]), None, array!([1, 2, 3, 4, 1])),
#[should_panic(expected = "input array should have the same dimension as the original one")]
case(array!([[1, 2], [3, 4]]), array!([4]), Some(0), array!([1, 3, 4])),
case(array!([[1, 2], [3, 4]]), array!([[1, 2]]), Some(0), array!([[1, 2], [3, 4], [1, 2]])),
case(array!([[1, 2], [3, 4]]), array!([[1], [2]]), Some(1), array!([[1, 2, 1], [3, 4, 2]])),
#[should_panic(expected = "input array dimensions for the concatenation axis must match exactly")]
case(array!([[1, 2], [3, 4]]), array!([[1, 2]]), Some(1), array!([[1, 2], [3, 4], [1, 1]])),
case(array!([[1, 2], [3, 4]]), array!([[1, 2], [3, 4]]), Some(0), array!([[1, 2], [3, 4], [1, 2], [3, 4]])),
case(array!([[1, 2], [3, 4]]), array!([[1, 2], [3, 4]]), Some(1), array!([[1, 2, 1, 2], [3, 4, 3, 4]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]]]), Some(0), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), Some(1), array!([[[1, 2], [3, 4], [1, 2], [3, 4]], [[1, 2], [3, 4], [1, 2], [3, 4]]])),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), Some(2), array!([[[1, 2, 1, 2], [3, 4, 3, 4]], [[1, 2, 1, 2], [3, 4, 3, 4]]])),
#[should_panic(expected = "input array should have the same dimension as the original one")]
case(array!([[1, 2], [3, 4]]), array!([[[1, 1], [1, 1]], [[1, 1], [1, 1]]]), Some(0), array!([1, 3, 4])),
#[should_panic(expected = "input array dimensions for the concatenation axis must match exactly")]
case(array!([[[1, 2], [3, 4]], [[4, 5], [6, 7]]]), array!([[[7, 8]]]), Some(0), array!([1, 3, 4])),
)] fn test_append(array: Array<i32>, values: Array<i32>, axis: Option<usize>, expected: Array<i32>) {
    assert_eq!(expected, array.append(&values, axis))
}

#[rstest(
array, new_shape, expected,
case(array!([1, 2, 3, 4]), vec![2, 2], array!([[1, 2], [3, 4]])),
case(array!([[1, 2], [3, 4]]), vec![4], array!([1, 2, 3, 4])),
case(array!([[1, 2, 3], [4, 5, 6]]), vec![3, 2], array!([[1, 2], [3, 4], [5, 6]])),
case(array!([[1, 2], [3, 4], [5, 6]]), vec![6], array!([1, 2, 3, 4, 5, 6])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![2, 4], array!([[1, 2, 3, 4], [5, 6, 7, 8]])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![8], array!([1, 2, 3, 4, 5, 6, 7, 8])),
#[should_panic(expected = "Shape must match values length")]
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![10], array!([1])),
)] fn test_reshape(array: Array<i32>, new_shape: Vec<usize>, expected: Array<i32>) {
    assert_eq!(expected, array.reshape(new_shape))
}

#[rstest(
array, new_shape, expected,
case(array!([[1, 2], [3, 4]]), vec![2], array!([1, 2])),
case(array!([[1, 2], [3, 4]]), vec![4], array!([1, 2, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![8], array!([1, 2, 3, 4, 1, 2, 3, 4])),
case(array!([[1, 2], [3, 4]]), vec![2, 4], array!([[1, 2, 3, 4], [1, 2, 3, 4]])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![2], array!([1, 2])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![8], array!([1, 2, 3, 4, 5, 6, 7, 8])),
)] fn test_resize(array: Array<i32>, new_shape: Vec<usize>, expected: Array<i32>) {
    assert_eq!(expected, array.resize(new_shape))
}

#[rstest(
array, axis, expected,
case(array!([1, 2, 3, 4, 5]), None, array!([1, 2, 3, 4, 5])),
case(array!([1, 2, 3, 4, 5, 3, 2, 1]), None, array!([1, 2, 3, 4, 5])),
case(array!([[1, 2], [3, 4]]), None, array!([1, 2, 3, 4])),
case(array!([[1, 2], [3, 4], [1, 2]]), None, array!([1, 2, 3, 4])),
case(array!([[1, 2], [3, 4], [1, 2]]), Some(0), array!([[1, 2], [3, 4]])),
case(array!([[1, 2], [3, 4], [1, 2]]), Some(1), array!([[1, 2], [3, 4], [1, 2]])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), None, array!([1, 2, 3, 4, 5, 6, 7, 8])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[1, 2], [3, 4]]]), None, array!([1, 2, 3, 4, 5, 6, 7, 8])),
case(array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]]), None, array!([1, 2, 3, 4, 5, 6, 7, 8])),
case(array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]]), Some(0), array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]])),
case(array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]]), Some(1), array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])),
case(array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]]), Some(2), array!([[[1, 2], [3, 4], [1, 2]], [[5, 6], [7, 8], [5, 6]]])),
case(array!([[[1, 1], [3, 3], [1, 1]], [[2, 2], [4, 4], [2, 2]]]), Some(2), array!([[[1], [3], [1]], [[2], [4], [2]]])),
)] fn test_unique(array: Array<i32>, axis: Option<usize>, expected: Array<i32>) {
    assert_eq!(expected, array.unique(axis))
}

#[rstest(
array, expected,
case(array!([[1, 2], [3, 4]]), vec![4]),
case(array!([[1, 2], [3, 4], [5, 6]]), vec![6]),
case(array!([[1, 2, 3], [4, 5, 6]]), vec![6]),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![8]),
)] fn test_ravel(array: Array<i32>, expected: Vec<usize>) {
    assert_eq!(expected, array.ravel().get_shape())
}

#[rstest(
arr, dim, expected,
case(array!([1]), 1, array!([1])),
case(array!([1, 2]), 2, array!([[1, 2]])),
case(array!([1, 2]), 3, array!([[[1, 2]]])),
case(array!([[[1], [2]]]), 3, array!([[[1], [2]]])),
case(array!([[[2, 2], [2, 2]], [[2, 2], [2, 2]]]), 4, array!([[[[2, 2], [2, 2]], [[2, 2], [2, 2]]]])),
#[should_panic(expected = "dimension cannot be zero")]
case(array!([1]), 0, array!([1])),
)] fn test_atleast(arr: Array<i32>, dim: usize, expected: Array<i32>) {
    assert_eq!(expected, arr.atleast(dim))
}

#[rstest(
arr, expected,
case(array!([1, 2, 3, 4]), array!([1, 2, 3, 4])),
case(array!([0, 0, 1, 2, 3, 4]), array!([1, 2, 3, 4])),
case(array!([0, 0, 1, 2, 3, 4]), array!([1, 2, 3, 4])),
case(array!([0, 0, 1, 2, 3, 4, 0, 0]), array!([1, 2, 3, 4])),
case(array!([0, 0, 0, 0]), Array::empty()),
#[should_panic(expected = "trim_zeros is defined only for 1d arrays")]
case(array!([[1, 2]]), array!([1])),
)] fn test_trim_zeros(arr: Array<i32>, expected: Array<i32>) {
    assert_eq!(expected, arr.trim_zeros())
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![2, 4, 6, 8]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
)] fn test_for_each(array: Array<i32>, expected: Vec<i32>) {
    let mut items = vec![];
    array.for_each(|item| items.push(*item * 2));
    assert_eq!(expected, items);
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![(0, 2), (1, 4), (2, 6), (3, 8)]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![(0, 2), (1, 4), (2, 6), (3, 8), (4, 2), (5, 4), (6, 6), (7, 8)]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![(0, 2), (1, 4), (2, 6), (3, 8), (4, 2), (5, 4), (6, 6), (7, 8)]),
)] fn test_for_each_e(array: Array<i32>, expected: Vec<(usize, i32)>) {
    let mut items = vec![];
    array.for_each_e(|idx, item| items.push((idx, *item * 2)));
    assert_eq!(expected, items);
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![2, 4, 6, 8]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![2, 4, 6, 8, 2, 4, 6, 8]),
)] fn test_map(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.map(|item| *item * 2).get_elements());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![0, 2, 6, 12]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![0, 2, 6, 12, 4, 10, 18, 28]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![0, 2, 6, 12, 4, 10, 18, 28]),
)] fn test_map_e(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.map_e(|idx, item| *item * idx as i32).get_elements());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![2, 4]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![2, 4, 2, 4]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![2, 4, 2, 4]),
)] fn test_filter(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.filter(|item| item % 2 == 0).get_elements());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![1, 2, 3, 4]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![1, 2, 3, 4]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1, 2, 3, 4]),
)] fn test_filter_e(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.filter_e(|idx, item| item % (idx + 1) as i32 == 0).get_elements());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![2, 4]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![2, 4, 2, 4]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![2, 4, 2, 4]),
)] fn test_filter_map(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.filter_map(|item|
        if item % 2 == 0 { Some(*item) } else { None }
    ).get_elements());
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), vec![1, 2, 3, 4]),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![1, 2, 3, 4]),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![1, 2, 3, 4]),
)] fn test_filter_map_e(array: Array<i32>, expected: Vec<i32>) {
    assert_eq!(expected, array.filter_map_e(|idx, item|
        if item % (idx + 1) as i32 == 0 { Some(*item) } else { None }
    ).get_elements());
}

#[rstest(
array, init, expected,
case(array!([1, 2, 3, 4]), 0, 10),
case(array!([[1, 2, 3, 4], [1, 2, 3, 4]]), 0, 20),
)] fn test_fold_sum(array: Array<i32>, init: i32, expected: i32) {
    assert_eq!(expected, array.fold(init, |a, b| a + b));
}

#[rstest(
array, init, expected,
case(array!([1, 2, 3, 4]), 1, 24),
case(array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), 1, 576),
)] fn test_fold_mul(array: Array<i32>, init: i32, expected: i32) {
    assert_eq!(expected, array.fold(init, |a, b| a * b));
}

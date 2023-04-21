use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, parts, axis, expected,
case(Array::empty(), 1, None, vec![Array::empty()]),
case(array!(1), 1, None, vec![array!(1)]),
case(array_arange!(0, 7), 1, None, vec![array_arange!(0, 7)]),
case(array_arange!(0, 7), 3, None, vec![array![0, 1, 2], array![3, 4, 5], array![6, 7]]),
case(array_arange!(0, 8), 4, None, vec![array![0, 1, 2], array![3, 4], array![5, 6], array![7, 8]]),
case(array_arange!(0, 7).reshape(vec![4, 2]), 2, Some(0), vec![array!([[0, 1], [2, 3]]), array!([[4, 5], [6, 7]])]),
case(array_arange!(0, 8).reshape(vec![3, 3]), 3, Some(1), vec![array!([[0], [3], [6]]), array!([[1], [4], [7]]), array!([[2], [5], [8]])]),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), 2, None, vec![array!([[[0, 1], [2, 3]]]), array!([[[4, 5], [6, 7]]])]),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), 2, Some(2), vec![array!([[[0], [2]], [[4], [6]]]), array!([[[1], [3]], [[5], [7]]])]),
#[should_panic(expected = "number of sections must be larger than 0")]
case(array_arange!(0, 7), 0, None, vec![]),
#[should_panic(expected = "axis out of bounds")]
case(array_arange!(0, 7), 3, Some(2), vec![]),
)] fn test_array_split(array: Array<i32>, parts: usize, axis: Option<usize>, expected: Vec<Array<i32>>) {
    assert_eq!(expected, array.array_split(parts, axis))
}

#[rstest(
array, parts, axis, expected,
case(Array::empty(), 1, None, vec![Array::empty()]),
case(array!(1), 1, None, vec![array!(1)]),
case(array_arange!(0, 7), 1, None, vec![array_arange!(0, 7)]),
case(array_arange!(0, 8), 3, None, vec![array![0, 1, 2], array![3, 4, 5], array![6, 7, 8]]),
case(array_arange!(0, 7).reshape(vec![4, 2]), 2, Some(0), vec![array!([[0, 1], [2, 3]]), array!([[4, 5], [6, 7]])]),
case(array_arange!(0, 8).reshape(vec![3, 3]), 3, Some(1), vec![array!([[0], [3], [6]]), array!([[1], [4], [7]]), array!([[2], [5], [8]])]),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), 2, None, vec![array!([[[0, 1], [2, 3]]]), array!([[[4, 5], [6, 7]]])]),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), 2, Some(2), vec![array!([[[0], [2]], [[4], [6]]]), array!([[[1], [3]], [[5], [7]]])]),
#[should_panic(expected = "array split does not result in an equal division")]
case(array_arange!(0, 7), 3, None, vec![array![0, 1, 2], array![3, 4, 5], array![6, 7]]),
#[should_panic(expected = "number of sections must be larger than 0")]
case(array_arange!(0, 7), 0, None, vec![]),
#[should_panic(expected = "axis out of bounds")]
case(array_arange!(0, 7), 3, Some(2), vec![]),
)] fn test_split(array: Array<i32>, parts: usize, axis: Option<usize>, expected: Vec<Array<i32>>) {
    assert_eq!(expected, array.split(parts, axis))
}

#[rstest(
array, parts, expected,
case(array_arange!(0, 5), 3, vec![array!([0, 1]), array!([2, 3]), array!([4, 5])]),
case(array_arange!(0, 5).reshape(vec![3, 2]), 2, vec![array!([[0], [2], [4]]), array!([[1], [3], [5]])]),
case(array_arange!(0, 5).reshape(vec![2, 3]), 3, vec![array!([[0], [3]]), array!([[1], [4]]), array!([[2], [5]])]),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), 2, vec![array!([[[0, 1]], [[4, 5]]]), array!([[[2, 3]], [[6, 7]]])]),
#[should_panic(expected = "array split does not result in an equal division")]
case(array_arange!(0, 6), 3, vec![]),
)] fn test_hsplit(array: Array<i32>, parts: usize, expected: Vec<Array<i32>>) {
    assert_eq!(expected, array.hsplit(parts))
}

#[rstest(
array, parts, expected,
case(array_arange!(0, 5).reshape(vec![3, 2]), 3, vec![array!([[0, 1]]), array!([[2, 3]]), array!([[4, 5]])]),
case(array_arange!(0, 5).reshape(vec![2, 3]), 2, vec![array!([[0, 1, 2]]), array!([[3, 4, 5]])]),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), 2, vec![array!([[[0, 1], [2, 3]]]), array!([[[4, 5], [6, 7]]])]),
#[should_panic(expected = "array split does not result in an equal division")]
case(array_arange!(0, 5).reshape(vec![2, 3]), 4, vec![]),
)] fn test_vsplit(array: Array<i32>, parts: usize, expected: Vec<Array<i32>>) {
    assert_eq!(expected, array.vsplit(parts))
}

#[rstest(
array, parts, expected,
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), 2, vec![array!([[[0], [2]], [[4], [6]]]), array!([[[1], [3]], [[5], [7]]])]),
case(array_arange!(0, 11).reshape(vec![3, 2, 2]), 2, vec![array!([[[0], [2]], [[4], [6]], [[8], [10]]]), array!([[[1], [3]], [[5], [7]], [[9], [11]]])]),
case(array_arange!(0, 11).reshape(vec![2, 3, 2]), 2, vec![array!([[[0], [2], [4]], [[6], [8], [10]]]), array!([[[1], [3], [5]], [[7], [9], [11]]])]),
case(array_arange!(0, 11).reshape(vec![2, 2, 3]), 3, vec![array!([[[0], [3]], [[6], [9]]]), array!([[[1], [4]], [[7], [10]]]), array!([[[2], [5]], [[8], [11]]])]),
#[should_panic(expected = "array split does not result in an equal division")]
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), 4, vec![array!([[[0], [2]], [[4], [6]]]), array!([[[1], [3]], [[5], [7]]])]),
)] fn test_dsplit(array: Array<i32>, parts: usize, expected: Vec<Array<i32>>) {
    assert_eq!(expected, array.dsplit(parts))
}

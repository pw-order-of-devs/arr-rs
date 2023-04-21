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
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), 2, None, vec![array!([[[0, 1], [2, 3]]]), array!([[[2, 3], [4, 5]]])]),
#[should_panic(expected = "number of sections must be larger than 0")]
case(array_arange!(0, 7), 0, None, vec![]),
#[should_panic(expected = "axis out of bounds")]
case(array_arange!(0, 7), 3, Some(2), vec![]),
)] fn test_array_split(array: Array<i32>, parts: usize, axis: Option<usize>, expected: Vec<Array<i32>>) {
    assert_eq!(expected, array.array_split(parts, axis))
}

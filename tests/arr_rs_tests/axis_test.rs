use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, axes, expected,
case(array_arange!(1, 5).reshape(vec![4]), None, array!([1, 2, 3, 4])),
case(array_arange!(1, 7).reshape(vec![2, 3]), None, array!([[1, 4], [2, 5], [3, 6]])),
case(array_arange!(1, 9).reshape(vec![2, 2, 2]), None, array!([[[1, 5], [3, 7]], [[2, 6], [4, 8]]])),
case(array_arange!(1, 17).reshape(vec![2, 2, 2, 2]), None, array!([[[[1, 9], [5, 13]], [[3, 11], [7, 15]]], [[[2, 10], [6, 14]], [[4, 12], [8, 16]]]])),
case(array_arange!(1, 7).reshape(vec![2, 3]), Some(vec![1, 0]), array!([[1, 4], [2, 5], [3, 6]])),
case(array_arange!(1, 9).reshape(vec![2, 2, 2]), Some(vec![0, 2, 1]), array!([[[1, 3], [2, 4]], [[5, 7], [6, 8]]])),
case(array_arange!(1, 7).reshape(vec![2, 3]), Some(vec![-1, -2]), array!([[1, 4], [2, 5], [3, 6]])),
case(array_arange!(1, 9).reshape(vec![2, 2, 2]), Some(vec![-3, -1, -2]), array!([[[1, 3], [2, 4]], [[5, 7], [6, 8]]])),
)] fn test_transpose(arr: Array<i32>, axes: Option<Vec<isize>>, expected: Array<i32>) {
    assert_eq!(expected, arr.transpose(axes))
}

#[rstest(
arr, source, destination, expected_shape,
case(array_zeros!(3, 4, 5), vec![0], vec![2], vec![4, 5, 3]),
case(array_zeros!(3, 4, 5), vec![2], vec![0], vec![5, 3, 4]),
case(array_zeros!(3, 4, 5), vec![0, 1, 2], vec![2, 0, 1], vec![4, 5, 3]),
case(array_zeros!(2, 3, 4, 5), vec![0, 1], vec![2, 3], vec![4, 5, 2, 3]),
case(array_zeros!(3, 4, 5), vec![-3], vec![-1], vec![4, 5, 3]),
case(array_zeros!(3, 4, 5), vec![-1], vec![-3], vec![5, 3, 4]),
case(array_zeros!(3, 4, 5), vec![-3, -2, -1], vec![-1, -3, -2], vec![4, 5, 3]),
case(array_zeros!(2, 3, 4, 5), vec![-4, -3], vec![-2, -1], vec![4, 5, 2, 3]),
)] fn test_moveaxis(arr: Array<i32>, source: Vec<isize>, destination: Vec<isize>, expected_shape: Vec<usize>) {
    assert_eq!(expected_shape, arr.moveaxis(source, destination).get_shape());
}

#[rstest(
arr, axis, start, expected_shape,
case(array_arange!(0, 24).reshape(vec![2, 3, 4]), 1, None, vec![3, 2, 4]),
case(array_arange!(0, 24).reshape(vec![2, 3, 4]), 2, None, vec![4, 2, 3]),
case(array_arange!(0, 24).reshape(vec![2, 3, 4]), 2, Some(1), vec![2, 4, 3]),
case(array_arange!(0, 24).reshape(vec![2, 3, 4]), -1, Some(-2), vec![2, 4, 3]),
case(array_arange!(0, 120).reshape(vec![2, 3, 4, 5]), 1, None, vec![3, 2, 4, 5]),
case(array_arange!(0, 120).reshape(vec![2, 3, 4, 5]), 3, None, vec![5, 2, 3, 4]),
case(array_arange!(0, 120).reshape(vec![2, 3, 4, 5]), 2, Some(1), vec![2, 4, 3, 5]),
case(array_arange!(0, 120).reshape(vec![2, 3, 4, 5]), 2, Some(2), vec![2, 3, 4, 5]),
case(array_arange!(0, 120).reshape(vec![2, 3, 4, 5]), -1, Some(-1), vec![2, 3, 4, 5]),
)] fn test_rollaxis(arr: Array<i32>, axis: isize, start: Option<isize>, expected_shape: Vec<usize>) {
    assert_eq!(expected_shape, arr.rollaxis(axis, start).get_shape());
}

#[rstest(
arr, axis_1, axis_2, expected_shape,
case(array_arange!(0, 24).reshape(vec![2, 3, 4]), 0, 2, vec![4, 3, 2]),
case(array_arange!(0, 24).reshape(vec![2, 3, 4]), 2, 1, vec![2, 4, 3]),
case(array_arange!(0, 24).reshape(vec![2, 3, 4]), -1, -2, vec![2, 4, 3]),
case(array_arange!(0, 120).reshape(vec![2, 3, 4, 5]), 0, 2, vec![4, 3, 2, 5]),
case(array_arange!(0, 120).reshape(vec![2, 3, 4, 5]), 1, 3, vec![2, 5, 4, 3]),
case(array_arange!(0, 120).reshape(vec![2, 3, 4, 5]), -3, -2, vec![2, 4, 3, 5]),
)] fn test_swapaxes(arr: Array<i32>, axis_1: isize, axis_2: isize, expected_shape: Vec<usize>) {
    assert_eq!(expected_shape, arr.swapaxes(axis_1, axis_2).get_shape());
}

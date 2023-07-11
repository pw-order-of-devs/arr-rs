use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, axes, expected,
case(array_arange!(1, 4).reshape(vec![4]), None, array!([1, 2, 3, 4])),
case(array_arange!(1, 6).reshape(vec![2, 3]), None, array!([[1, 4], [2, 5], [3, 6]])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), None, array!([[[1, 5], [3, 7]], [[2, 6], [4, 8]]])),
case(array_arange!(1, 16).reshape(vec![2, 2, 2, 2]), None, array!([[[[1, 9], [5, 13]], [[3, 11], [7, 15]]], [[[2, 10], [6, 14]], [[4, 12], [8, 16]]]])),
case(array_arange!(1, 6).reshape(vec![2, 3]), Some(vec![1, 0]), array!([[1, 4], [2, 5], [3, 6]])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), Some(vec![0, 2, 1]), array!([[[1, 3], [2, 4]], [[5, 7], [6, 8]]])),
case(array_arange!(1, 6).reshape(vec![2, 3]), Some(vec![-1, -2]), array!([[1, 4], [2, 5], [3, 6]])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), Some(vec![-3, -1, -2]), array!([[[1, 3], [2, 4]], [[5, 7], [6, 8]]])),
)] fn test_transpose(arr: Result<Array<i32>, ArrayError>, axes: Option<Vec<isize>>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, arr.unwrap().transpose(axes))
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
)] fn test_moveaxis(arr: Result<Array<i32>, ArrayError>, source: Vec<isize>, destination: Vec<isize>, expected_shape: Vec<usize>) {
    assert_eq!(expected_shape, arr.moveaxis(source, destination).unwrap().get_shape().unwrap());
}

#[rstest(
arr, axis, start, expected_shape,
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), 1, None, vec![3, 2, 4]),
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), 2, None, vec![4, 2, 3]),
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), 2, Some(1), vec![2, 4, 3]),
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), -1, Some(-2), vec![2, 4, 3]),
case(array_arange!(1, 120).reshape(vec![2, 3, 4, 5]), 1, None, vec![3, 2, 4, 5]),
case(array_arange!(1, 120).reshape(vec![2, 3, 4, 5]), 3, None, vec![5, 2, 3, 4]),
case(array_arange!(1, 120).reshape(vec![2, 3, 4, 5]), 2, Some(1), vec![2, 4, 3, 5]),
case(array_arange!(1, 120).reshape(vec![2, 3, 4, 5]), 2, Some(2), vec![2, 3, 4, 5]),
case(array_arange!(1, 120).reshape(vec![2, 3, 4, 5]), -1, Some(-1), vec![2, 3, 4, 5]),
)] fn test_rollaxis(arr: Result<Array<i32>, ArrayError>, axis: isize, start: Option<isize>, expected_shape: Vec<usize>) {
    assert_eq!(expected_shape, arr.unwrap().rollaxis(axis, start).unwrap().get_shape().unwrap());
}

#[rstest(
arr, axis_1, axis_2, expected,
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), 0, 2, vec![4, 3, 2]),
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), 2, 1, vec![2, 4, 3]),
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), -1, -2, vec![2, 4, 3]),
case(array_arange!(1, 120).reshape(vec![2, 3, 4, 5]), 0, 2, vec![4, 3, 2, 5]),
case(array_arange!(1, 120).reshape(vec![2, 3, 4, 5]), 1, 3, vec![2, 5, 4, 3]),
case(array_arange!(1, 120).reshape(vec![2, 3, 4, 5]), -3, -2, vec![2, 4, 3, 5]),
)] fn test_swapaxes(arr: Result<Array<i32>, ArrayError>, axis_1: isize, axis_2: isize, expected: Vec<usize>) {
    assert_eq!(expected, arr.unwrap().swapaxes(axis_1, axis_2).unwrap().get_shape().unwrap());
}

#[rstest(
arr, axis, expected,
case(array_arange!(1, 4).reshape(vec![2, 2]), vec![0], vec![1, 2, 2]),
case(array_arange!(1, 4).reshape(vec![2, 2]), vec![1], vec![2, 1, 2]),
case(array_arange!(1, 4).reshape(vec![2, 2]), vec![2], vec![2, 2, 1]),
case(array_arange!(1, 4).reshape(vec![2, 2]), vec![0, 2], vec![1, 2, 1, 2]),
case(array_arange!(1, 4).reshape(vec![2, 2]), vec![0, 1, 2], vec![1, 1, 1, 2, 2]),
case(array_arange!(1, 4).reshape(vec![2, 2]), vec![-3], vec![1, 2, 2]),
case(array_arange!(1, 4).reshape(vec![2, 2]), vec![-2], vec![2, 1, 2]),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), vec![3], vec![2, 2, 2, 1]),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), vec![-3, 3], vec![2, 2, 1, 1, 2]),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), vec![], vec![2, 2, 2]),
)] fn test_expand_dims(arr: Result<Array<i32>, ArrayError>, axis: Vec<isize>, expected: Vec<usize>) {
    assert_eq!(expected, arr.unwrap().expand_dims(axis).unwrap().get_shape().unwrap());
}

#[rstest(
arr, axis, expected,
case(array_arange!(1, 4).reshape(vec![1, 2, 2]), None, Ok(vec![2, 2])),
case(array_arange!(1, 4).reshape(vec![2, 1, 1, 1, 2]), None, Ok(vec![2, 2])),
case(array_arange!(1, 4).reshape(vec![1, 2, 1, 2, 1]), None, Ok(vec![2, 2])),
case(array_arange!(1, 4).reshape(vec![1, 2, 1, 2, 1]), Some(vec![0]), Ok(vec![2, 1, 2, 1])),
case(array_arange!(1, 4).reshape(vec![1, 2, 1, 2, 1]), Some(vec![0, 2]), Ok(vec![2, 2, 1])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2, 1]), Some(vec![3]), Ok(vec![2, 2, 2])),
case(array_arange!(1, 8).reshape(vec![2, 2, 1, 2, 1]), Some(vec![-3]), Ok(vec![2, 2, 2, 1])),
case(array_arange!(1, 8).reshape(vec![2, 2, 1, 2, 1]), Some(vec![1]), Err(ArrayError::SqueezeShapeOfAxisMustBeOne)),
case(array_arange!(1, 8).reshape(vec![2, 2, 1, 2, 1]), Some(vec![2, 3, 4]), Err(ArrayError::SqueezeShapeOfAxisMustBeOne)),
)] fn test_squeeze(arr: Result<Array<i32>, ArrayError>, axis: Option<Vec<isize>>, expected: Result<Vec<usize>, ArrayError>) {
    match expected {
        Ok(expected) => assert_eq!(expected, arr.squeeze(axis).get_shape().unwrap()),
        Err(err) => assert_eq!(Err(err), arr.squeeze(axis)),
    }
}

use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, axis, keepdims, expected,
case(array!(f64, [[10., 11., 12.], [13., 14., 15.]]), None, None, array!(usize, 5)),
case(array!(f64, [[10., 11., 12.], [13., 14., 15.]]), Some(0), None, array!(usize, 1, 1, 1)),
case(array!(f64, [[10., 11., 12.], [13., 14., 15.]]), Some(1), None, array!(usize, 2, 2)),
case(array!(f64, [[f64::NAN, 4.], [2., 3.]]), None, None, array!(usize, 0)),
case(array!(f64, [[f64::NAN, 4.], [2., 3.]]), Some(0), None, array!(usize, 0, 0)),
case(array!(f64, [[f64::NAN, 4.], [2., 3.]]), Some(1), None, array!(usize, 0, 1)),
)] fn test_argmax(arr: Result<Array<f64>, ArrayError>, axis: Option<isize>, keepdims: Option<bool>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, arr.argmax(axis, keepdims))
}

#[rstest(
arr, axis, keepdims, expected,
case(array!(f64, [[10., 11., 12.], [13., 14., 15.]]), None, None, array!(usize, 0)),
case(array!(f64, [[10., 11., 12.], [13., 14., 15.]]), Some(0), None, array!(usize, 0, 0, 0)),
case(array!(f64, [[10., 11., 12.], [13., 14., 15.]]), Some(1), None, array!(usize, 0, 0)),
case(array!(f64, [[f64::NAN, 4.], [2., 3.]]), None, None, array!(usize, 0)),
case(array!(f64, [[f64::NAN, 4.], [2., 3.]]), Some(0), None, array!(usize, 0, 1)),
case(array!(f64, [[f64::NAN, 4.], [2., 3.]]), Some(1), None, array!(usize, 0, 0)),
)] fn test_argmin(arr: Result<Array<f64>, ArrayError>, axis: Option<isize>, keepdims: Option<bool>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, arr.argmin(axis, keepdims))
}

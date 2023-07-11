use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_eye!(2), array!([[1., 0.], [0., 1.]])),
case(array_eye!(2, 3), array!([[1., 0., 0.], [0., 1., 0.]])),
case(array_eye!(3, 2, 1), array!([[0., 1.], [0., 0.], [0., 0.]])),
case(array_eye!(4, 3, 0), array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [0., 0., 0.]])),
case(array_eye!(4, 3, 1), array!([[0., 1., 0.], [0., 0., 1.], [0., 0., 0.], [0., 0., 0.]])),
)] fn test_eye_array_macro(arr: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr);
}

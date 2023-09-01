use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_identity!(f64, 2), array!(f64, [[1., 0.], [0., 1.]])),
case(array_identity!(f64, 3), array!(f64, [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])),
case(array_identity!(f64, 4), array!(f64, [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]])),
case(array_identity!(f64, 5), array!(f64, [[1., 0., 0., 0., 0.], [0., 1., 0., 0., 0.], [0., 0., 1., 0., 0.], [0., 0., 0., 1., 0.], [0., 0., 0., 0., 1.]])),
)] fn test_identity_array_macro(arr: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr);
}

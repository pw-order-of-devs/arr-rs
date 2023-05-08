use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_identity!(2), array!([[1., 0.], [0., 1.]])),
case(array_identity!(3), array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])),
case(array_identity!(4), array!([[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]])),
case(array_identity!(5), array!([[1., 0., 0., 0., 0.], [0., 1., 0., 0., 0.], [0., 0., 1., 0., 0.], [0., 0., 0., 1., 0.], [0., 0., 0., 0., 1.]])),
)] fn test_identity_array_macro(arr: Array<f64>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr);
}

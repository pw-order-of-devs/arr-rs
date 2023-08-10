use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, mode, expected,
case(array_arange![1., 4.], array_arange![1., 3.], None, array![1., 4., 10., 16., 17., 12.]),
case(array_arange![1., 4.], array_arange![1., 3.], Some(ConvolveMode::Full), array![1., 4., 10., 16., 17., 12.]),
case(array_arange![1., 4.], array_arange![1., 3.], Some(ConvolveMode::Valid), array![10., 16.]),
case(array_arange![1., 4.], array_arange![1., 3.], Some(ConvolveMode::Same), array![4., 10., 16., 17.]),
case(array_arange![1., 8.], array_arange![1., 6.], None, array![1., 4., 10., 20., 35., 56., 77., 98., 110., 112., 103., 82., 48.]),
case(array_arange![1., 8.], array_arange![1., 6.], Some(ConvolveMode::Full), array![1., 4., 10., 20., 35., 56., 77., 98., 110., 112., 103., 82., 48.]),
case(array_arange![1., 8.], array_arange![1., 6.], Some(ConvolveMode::Valid), array![56., 77., 98.]),
case(array_arange![1., 8.], array_arange![1., 6.], Some(ConvolveMode::Same), array![10., 20., 35., 56., 77., 98., 110., 112.]),
case(array_arange![-1., 2.], array_arange![1., 3.], None, array![-1., -2., -2., 4., 7., 6.]),
case(array_arange![-1., 2.], array_arange![1., 3.], Some(ConvolveMode::Full), array![-1., -2., -2., 4., 7., 6.]),
case(array_arange![-1., 2.], array_arange![1., 3.], Some(ConvolveMode::Valid), array![-2., 4.]),
case(array_arange![-1., 2.], array_arange![1., 3.], Some(ConvolveMode::Same), array![-2., -2., 4., 7.]),
case(array_arange![-8., 6.], array_arange![1., 5.], None, array![-8., -23., -44., -70., -100., -85., -70., -55., -40., -25., -10.,  5., 20., 35., 50., 58., 58., 49., 30.]),
case(array_arange![-8., 6.], array_arange![1., 5.], Some(ConvolveMode::Full), array![-8., -23., -44., -70., -100., -85., -70., -55., -40., -25., -10.,  5., 20., 35., 50., 58., 58., 49., 30.]),
case(array_arange![-8., 6.], array_arange![1., 5.], Some(ConvolveMode::Valid), array![-100., -85., -70., -55., -40., -25., -10.,  5., 20., 35., 50.]),
case(array_arange![-8., 6.], array_arange![1., 5.], Some(ConvolveMode::Same), array![-44., -70., -100., -85., -70., -55., -40., -25., -10.,  5., 20., 35., 50., 58., 58.]),
)] fn test_convolve(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, mode: Option<ConvolveMode>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.convolve(&other.unwrap(), mode))
}

#[rstest(
array, a_min, a_max, expected,
case(array![1, 2, 3, 4], None, None, array![1, 2, 3, 4]),
case(array![1, 2, 3, 4], Some(array!(2).unwrap()), Some(array!(3).unwrap()), array![2, 2, 3, 3]),
case(array![1, 2, 3, 4], Some(array!(1).unwrap()), Some(array!(1).unwrap()), array![1, 1, 1, 1]),
case(array![1, 2, 3, 4], Some(array!(2, 2).unwrap()), Some(array!(3, 3).unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
case(array![1, 2, 3, 4], Some(array!(2, 2, 2, 2).unwrap()), Some(array!(3, 3, 3, 3).unwrap()), array![2, 2, 3, 3]),
case(array![-2, -1, 1, 2], Some(array!(-1).unwrap()), Some(array!(1).unwrap()), array![-1, -1, 1, 1]),
)] fn test_clip(array: Result<Array<i32>, ArrayError>, a_min: Option<Array<i32>>, a_max: Option<Array<i32>>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.clip(a_min, a_max))
}

#[rstest(
array, expected,
case(array![1, 4, 9, 16], array![1, 2, 3, 4]),
case(array!([[1, 4], [9, 16]]), array!([[1, 2], [3, 4]])),
)] fn test_sqrt(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.sqrt())
}

#[rstest(
array, expected,
case(array![1, 8, 27, 64], array![1, 2, 3, 4]),
case(array!([[1, 8], [27, 64]]), array!([[1, 2], [3, 4]])),
)] fn test_cbrt(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.cbrt())
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], array![1, 4, 9, 16]),
case(array!([[1, 2], [3, 4]]), array!([[1, 4], [9, 16]])),
)] fn test_square(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.square())
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], array![1, 2, 3, 4]),
case(array![1, -2, 3, -4], array![1, 2, 3, 4]),
case(array!([[1, -2], [3, -4]]), array!([[1, 2], [3, 4]])),
)] fn test_absolute(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.absolute())
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], array![1, 2, 3, 4]),
case(array![1, -2, 3, -4], array![1, 2, 3, 4]),
case(array!([[1, -2], [3, -4]]), array!([[1, 2], [3, 4]])),
)] fn test_abs(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.abs())
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], array![1, 2, 3, 4]),
case(array![1, -2, 3, -4], array![1, 2, 3, 4]),
case(array!([[1, -2], [3, -4]]), array!([[1, 2], [3, 4]])),
)] fn test_fabs(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.fabs())
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], array![1, 1, 1, 1]),
case(array![1, -2, 3, -4], array![1, -1, 1, -1]),
case(array!([[1, -2], [3, -4]]), array!([[1, -1], [1, -1]])),
)] fn test_sign(array: Result<Array<i32>, ArrayError>, expected: Result<Array<isize>, ArrayError>) {
    assert_eq!(expected, array.sign())
}

#[rstest(
array, other, expected,
case(array![1.], array![0.], array![1.]),
case(array![-1.], array![0.], array![0.]),
case(array![-1.5, 0., 2.], array![0.5], array![0., 0.5, 1.]),
case(array![-1.5, 0., 2.], array![1.], array![0., 1., 1.]),
case(array![-1.5, 0., 0., 2.], array![1.], array![0., 1., 1., 1.]),
case(array![[-1.5, 0.], [0., 2.]], array![1.], array![[0., 1.], [1., 1.]]),
)] fn test_heaviside(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.heaviside(&other.unwrap()))
}

#[rstest(
array, expected,
case(array![1., 2., 3., 4.], array![1., 2., 3., 4.]),
case(array![1., f64::NAN, 1., f64::NAN], array![1., 0., 1., 0.]),
case(array![0., f64::INFINITY, 0., f64::INFINITY], array![0., f64::MAX, 0., f64::MAX]),
)] fn test_nan_to_num(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.nan_to_num())
}

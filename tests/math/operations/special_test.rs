use arr_rs::prelude::*;
use rstest::rstest;

#[rstest(
array, expected,
case(array![3.], array![4.880792585865023]),
case(array![0., 1., 2., 3.], array![0.9999999999999997, 1.2660658777520077, 2.2795853023360664, 4.880792585865023]),
case(array![-2., 0., 3.5], array![2.27958510662287, 0.9999999999999997, 7.378203432225479]),
)] fn test_i0(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.i0());
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![3.8981718325193755e-17, 1.0, 3.8981718325193755e-17]),
case(array![-4., -2., 2., 4.], array![-3.8981718325193755e-17, -3.8981718325193755e-17, -3.8981718325193755e-17, -3.8981718325193755e-17]),
case(array![-1.5, -0.5, 0.5, 1.5], array![-0.2122065907891938, std::f64::consts::FRAC_2_PI, std::f64::consts::FRAC_2_PI, -0.2122065907891938]),
)] fn test_sinc(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.sinc());
}

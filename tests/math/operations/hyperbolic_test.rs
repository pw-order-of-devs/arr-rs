use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![-1.1752011936438014, 0., 1.1752011936438014]),
case(array![0.5, -0.5, 0.25], array![0.5210953054937474, -0.5210953054937474, 0.2526123168081683]),
case(array![1.5, -1.5, 2.0], array![2.1292794550948173, -2.1292794550948173, 3.626860407847019]),
case(array![2.5, -2.5, 3.0], array![6.0502044810397875, -6.0502044810397875, 10.017874927409903]),
case(array![0.1, -0.1, 0.75], array![0.10016675001984403, -0.10016675001984403, 0.82231673193583]),
)] fn test_sinh(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.sinh()))
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![1.5430806348152437, 1., 1.5430806348152437]),
case(array![0.5, -0.5, 0.25], array![1.1276259652063807, 1.1276259652063807, 1.0314130998795732]),
case(array![1.5, -1.5, 2.0], array![2.352409615243247, 2.352409615243247, 3.7621956910836314]),
case(array![2.5, -2.5, 3.0], array![6.132289479663686, 6.132289479663686, 10.067661995777765]),
case(array![0.1, -0.1, 0.75], array![1.0050041680558035, 1.0050041680558035, 1.2946832846768448]),
)] fn test_cosh(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.cosh()))
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![-0.7615941559557649, 0., 0.7615941559557649]),
case(array![0.5, -0.5, 0.25], array![0.46211715726000974, -0.46211715726000974, 0.24491866240370913]),
case(array![1.5, -1.5, 2.0], array![0.9051482536448664, -0.9051482536448664, 0.9640275800758169]),
case(array![2.5, -2.5, 3.0], array![0.9866142981514303, -0.9866142981514303, 0.9950547536867305]),
case(array![0.1, -0.1, 0.75], array![0.09966799462495582, -0.09966799462495582, 0.6351489523872873]),
)] fn test_tanh(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.tanh()))
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![-0.881373587019543, 0., 0.881373587019543]),
case(array![0.5, -0.5, 0.25], array![0.4812118251, -0.4812118251, 0.2474664615]),
case(array![1.5, -1.5, 2.0], array![1.1947632172871093, -1.1947632172871093, 1.4436354751788103]),
case(array![2.5, -2.5, 3.0], array![1.6472311463710957, -1.6472311463710957, 1.8184464592320668]),
case(array![0.1, -0.1, 0.75], array![0.0998340789, -0.0998340789, std::f64::consts::LN_2]),
)] fn test_asinh(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.asinh()))
}

#[rstest(
array, expected,
case(array![1., 2., 3.], array![0., 1.3169578969248166, 1.762747174039086]),
case(array![0.5, -0.5, 0.25], array![f64::NAN, f64::NAN, f64::NAN]),
case(array![1.5, -1.5, 2.0], array![0.96242365, f64::NAN, 1.31695790]),
case(array![2.5, -2.5, 3.0], array![1.56679924, f64::NAN, 1.76274717]),
case(array![0.1, -0.1, 0.75], array![f64::NAN, f64::NAN, f64::NAN]),
)] fn test_acosh(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.acosh()))
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![-f64::INFINITY, 0., f64::INFINITY]),
case(array![0.5, -0.5, 0.25], array![0.5493061443340549, -0.5493061443340549, 0.25541281188299534]),
case(array![1.5, -1.5, 2.0], array![f64::NAN, f64::NAN, f64::NAN]),
case(array![2.5, -2.5, 3.0], array![f64::NAN, f64::NAN, f64::NAN]),
case(array![0.1, -0.1, 0.75], array![0.10033534773107558, -0.10033534773107558, 0.9729550745276566]),
)] fn test_atanh(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.atanh()))
}

use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array![1, 4, 9, 16], array![1, 2, 3, 4]),
case(array!([[1, 4], [9, 16]]), array!([[1, 2], [3, 4]])),
)] fn test_sqrt(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.sqrt())
}

#[rstest(
array, expected,
case(array![1., 4., 8., 16.], array![0., 1.3862943611198906, 2.0794415416798357, 2.772588722239781]),
case(array!([[1., 4.], [8., 16.]]), array!([[0., 1.3862943611198906], [2.0794415416798357, 2.772588722239781]])),
)] fn test_log(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.log())
}

#[rstest(
array, expected,
case(array![1., 4., 8., 16.], array![0., 2., 3., 4.]),
case(array!([[1., 4.], [8., 16.]]), array!([[0., 2.], [3., 4.]])),
)] fn test_log2(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.log2())
}

#[rstest(
array, expected,
case(array![1., 10., 100.], array![0., 1., 2.]),
case(array!([[1., 10.], [1., 10.]]), array!([[0., 1.], [0., 1.]])),
)] fn test_log10(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.log10())
}

#[rstest(
array, log, expected,
case(array![2., 4., 8., 20.], array![2., 2., 2., 10.], array![1., 2., 3., 1.301029995663981]),
case(array!([[2., 4.], [8., 20.]]), array!([[2., 2.], [2., 10.]]), array!([[1., 2.], [3., 1.301029995663981]])),
)] fn test_logn(array: Result<Array<f64>, ArrayError>, log: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.logn(&log.unwrap()))
}

#[rstest(
array, pow, expected,
case(array![1, 2, 3, 4], array![2], array![1, 4, 9, 16]),
case(array![1, 2, 3, 4], array![3], array![1, 8, 27, 64]),
case(array!([[1, 2], [3, 4]]), array!([[2, 2], [3, 3]]), array!([[1, 4], [27, 64]])),
)] fn test_power(array: Result<Array<i32>, ArrayError>, pow: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.power(&pow.unwrap()))
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], 24),
case(array!([[1, 2], [3, 4]]), 24),
)] fn test_product(array: Result<Array<i32>, ArrayError>, expected: i32) {
    assert_eq!(expected, array.product().unwrap())
}

#[rstest(
array, expected,
case(array![1, 2, 3, 4], 10),
case(array!([[1, 2], [3, 4]]), 10),
)] fn test_sum(array: Result<Array<i32>, ArrayError>, expected: i32) {
    assert_eq!(expected, array.sum().unwrap())
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), array!([1, 3, 6, 10])),
case(array!([[1, 2], [3, 4]]), array!([1, 3, 6, 10])),
)] fn test_cumsum(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.cumsum())
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![-0.8414709848078965, 0., 0.8414709848078965]),
case(array![0.5, -0.5, 0.25], array![0.479425538604203, -0.479425538604203, 0.24740395925452294]),
case(array![1.5, -1.5, 2.0], array![0.9974949866040544, -0.9974949866040544, 0.9092974268256817]),
)] fn test_sin(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.sin()))
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![0.5403023058681398, 1., 0.5403023058681398]),
case(array![0.5, -0.5, 0.25], array![0.8775825618903728, 0.8775825618903728, 0.9689124217106447]),
case(array![1.5, -1.5, 2.0], array![0.0707372016677029, 0.0707372016677029, -0.4161468365471424]),
)] fn test_cos(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.cos()))
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![-1.5574077246549023, 0., 1.5574077246549023]),
case(array![0.5, -0.5, 0.25], array![0.5463024898437905, -0.5463024898437905, 0.25534192122103627]),
case(array![1.5, -1.5, 2.0], array![14.101419947171719, -14.101419947171719, -2.185039863261519]),
case(array![2.5, -2.5, 3.0], array![-0.7470222972386603, 0.7470222972386603, -0.1425465430742778]),
case(array![0.1, -0.1, 0.75], array![0.10033467208545055, -0.10033467208545055, 0.9315964599440725]),
)] fn test_tan(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.tan()))
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![-std::f64::consts::FRAC_PI_2, 0., std::f64::consts::FRAC_PI_2]),
case(array![0.5, -0.5, 0.25], array![std::f64::consts::FRAC_PI_6, -std::f64::consts::FRAC_PI_6, 0.25268025514207865]),
case(array![1.5, -1.5, 2.0], array![f64::NAN, f64::NAN, f64::NAN]),
case(array![0.1, -0.1, 0.75], array![0.10016742116155979, -0.10016742116155979, 0.848062078981481]),
)] fn test_asin(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.asin()))
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![std::f64::consts::PI, std::f64::consts::FRAC_PI_2, 0.]),
case(array![0.5, -0.5, 0.25], array![std::f64::consts::FRAC_PI_3, 2.0943951023931957, 1.318116071652818]),
case(array![1.5, -1.5, 2.0], array![f64::NAN, f64::NAN, f64::NAN]),
case(array![0.1, -0.1, 0.75], array![1.4706289056333368, 1.6709637479564565, 0.7227342478134154]),
)] fn test_acos(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.acos()))
}

#[rstest(
array, expected,
case(array![-1., 0., 1.], array![-std::f64::consts::FRAC_PI_4, 0., std::f64::consts::FRAC_PI_4]),
case(array![0.5, -0.5, 0.25], array![0.4636476090008061, -0.4636476090008061, 0.24497866312686414]),
case(array![1.5, -1.5, 2.0], array![0.982793723247329, -0.982793723247329, 1.1071487177940904]),
case(array![2.5, -2.5, 3.0], array![1.1902899496825317, -1.1902899496825317, 1.2490457723982544]),
case(array![0.1, -0.1, 0.75], array![0.09966865249116204, -0.09966865249116204, 0.643501109]),
)] fn test_atan(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.atan()))
}

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

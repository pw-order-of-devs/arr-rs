use rstest::rstest;
use arr_rs::prelude::*;

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
array, other, expected,
case(array![-1., 0., 1.], array![0., 1., 0.], array![-std::f64::consts::FRAC_PI_2, 0., std::f64::consts::FRAC_PI_2]),
case(array![0.5, -0.5, 0.25], array![0., -1., 1.], array![std::f64::consts::FRAC_PI_2, -2.67794504, 0.24497866]),
case(array![1.5, -1.5, 2.0], array![0., -1., 1.], array![std::f64::consts::FRAC_PI_2, -2.15879893, 1.10714872]),
case(array![2.5, -2.5, 3.0], array![1., -1., 0.], array![1.19028995, -1.95130270, std::f64::consts::FRAC_PI_2]),
case(array![0.1, -0.1, 0.75], array![1., 0., -1.], array![0.09966865, -std::f64::consts::FRAC_PI_2, 2.49809154]),
)] fn test_atan2(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.atan2(&other.unwrap())))
}

#[rstest(
array, other, expected,
case(array_full!(vec![2, 2], 1.), array_full!(vec![2, 2], 2.), array_full!(vec![2, 2], 2.23606798)),
case(array_full!(vec![3, 3], 0.), array_full!(vec![3, 3], 5.), array_full!(vec![3, 3], 5.)),
case(array_full!(vec![3, 3], 3.), array_full!(vec![3, 3], 4.), array_full!(vec![3, 3], 5.)),
case(array_full!(vec![4, 4], 10.), array_full!(vec![4, 4], 5.), array_full!(vec![4, 4], 11.18033989)),
)] fn test_hypot(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.hypot(&other.unwrap())))
}

#[rstest(
array, expected,
case(array![0., std::f64::consts::PI, 2. * std::f64::consts::PI], array![0., 180., 360.]),
case(array![std::f64::consts::FRAC_PI_6, std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_2], array![30., 45., 60., 90.]),
)] fn test_degrees(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.degrees()))
}

#[rstest(
array, expected,
case(array![0., std::f64::consts::PI, 2. * std::f64::consts::PI], array![0., 180., 360.]),
case(array![std::f64::consts::PI / 12., std::f64::consts::FRAC_PI_8, std::f64::consts::FRAC_PI_6], array![15., 22.5, 30.]),
case(array![std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_2], array![45., 60., 90.]),
)] fn test_rad2deg(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.rad2deg()))
}

#[rstest(
array, expected,
case(array![0., 180., 360.], array![0., std::f64::consts::PI, 2. * std::f64::consts::PI]),
case(array![15., 22.5, 30.], array![std::f64::consts::PI / 12., std::f64::consts::FRAC_PI_8, std::f64::consts::FRAC_PI_6]),
case(array![45., 60., 90.], array![std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_2]),
)] fn test_radians(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.radians()))
}

#[rstest(
array, expected,
case(array![0., 180., 360.], array![0., std::f64::consts::PI, 2. * std::f64::consts::PI]),
case(array![15., 22.5, 30.], array![std::f64::consts::PI / 12., std::f64::consts::FRAC_PI_8, std::f64::consts::FRAC_PI_6]),
case(array![45., 60., 90.], array![std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_2]),
)] fn test_deg2rad(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.deg2rad()))
}

fn array_unwrap_3d() -> Result<Array<f64>, ArrayError> {
    array!([
 [[1.2, 2.4, 3.5, 4.8], [5.1, 6.3, 7.4, 8.7], [9.2, 10.5, 11.7, 12.9]],
 [[13.1, 14.4, 15.6, 16.8], [17.2, 18.5, 19.7, 20.9], [21.3, 22.6, 23.8, 24.1]]
    ])
}

#[rstest(
array, discont, axis, period, expected,
case(array_flat![0.5, 1.5, 3.2, 6.8, 5.9], None, None, None, array![0.5, 1.5, 3.2, 0.51681469, -0.38318531]),
case(array!([[0.5, 1.5], [3.2, 6.8], [5.9, 7.2]]), None, None, None, array!([[0.5, 1.5], [3.2, 0.51681469], [5.9, 7.2]])),
case(array!([[0.5, 1.5], [3.2, 6.8], [5.9, 7.2]]), Some(array!([[std::f64::consts::PI * 2.], [std::f64::consts::PI], [std::f64::consts::FRAC_PI_2]]).unwrap()), None, None, array!([[0.5, 1.5], [3.2, 0.51681469], [5.9, 7.2]])),
case(array!([[0.5, 1.5], [3.2, 6.8], [5.9, 7.2]]), None, Some(0), None, array!([[0.5, 1.5], [3.2, 0.51681469], [5.9, 0.91681469]])),
case(array!([[0.5, 1.5], [3.2, 6.8], [5.9, 7.2]]), None, Some(1), None, array!([[0.5, 1.5], [3.2, 0.51681469], [5.9, 7.2]])),
case(array!([[0.5, 1.5], [3.2, 6.8], [5.9, 7.2]]), None, Some(-1), None, array!([[0.5, 1.5], [3.2, 0.51681469], [5.9, 7.2]])),
case(array_unwrap_3d(), None, None, None, array_unwrap_3d()),
case(array_unwrap_3d(), Some(array!([[[1.], [2.], [1.]], [[2.], [3.], [1.]]]).unwrap()), None, Some(array!([[[2.], [3.], [2.]], [[1.], [2.], [2.]]]).unwrap()), array!([[[1.2, 0.4, -0.5, -1.2], [5.1, 6.3, 7.4, 8.7], [9.2, 8.5, 7.7, 6.9]], [[13.1, 14.4, 15.6, 16.8], [17.2, 18.5, 19.7, 20.9], [21.3, 20.6, 19.8, 20.1]]])),
case(array_unwrap_3d(), None, Some(0), None, array!([[[1.2, 2.4, 3.5, 4.8], [5.1, 6.3, 7.4, 8.7], [9.2, 10.5, 11.7, 12.9]], [[0.53362939, 1.83362939, 3.03362939, 4.23362939], [4.63362939, 5.93362939, 7.13362939, 8.33362939], [8.73362939, 10.03362939, 11.23362939, 11.53362939]]])),
case(array_unwrap_3d(), None, Some(1), None, array!([[[1.2, 2.4, 3.5, 4.8], [-1.18318531, 0.01681469, 1.11681469, 2.41681469], [-3.36637061, -2.06637061, -0.86637061, 0.33362939]], [[13.1, 14.4, 15.6, 16.8], [10.91681469, 12.21681469, 13.41681469, 14.61681469], [8.73362939, 10.03362939, 11.23362939, 11.53362939]]])),
case(array_unwrap_3d(), None, Some(2), None, array_unwrap_3d()),
)] fn test_unwrap(array: Result<Array<f64>, ArrayError>, discont: Option<Array<f64>>, axis: Option<isize>, period: Option<Array<f64>>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", array.unwrap_phase(discont, axis, period)))
}

use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array![1., 2., 3., 4.], array![false, false, false, false]),
case(array![-1., -2., -3., -4.], array![true, true, true, true]),
case(array![1., -2., 3., -4.], array![false, true, false, true]),
)] fn test_signbit(array: Result<Array<f64>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.signbit())
}

#[rstest(
array, other, expected,
case(array![1., 2., 3., 4.], array![-1., -2., -3., 4.], array![-1., -2., -3., 4.]),
case(array![-1., -2., -3., -4.], array![1., 2., -3., 4.], array![1., 2., -3., 4.]),
case(array![1., -2., 3., -4.], array![-1., 2., -3., 4.], array![-1., 2., -3., 4.]),
)] fn test_copysign(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.copysign(&other.unwrap()))
}

#[rstest(
array, expected,
case(array_arange![0., 8.], Ok((array_flat!(0., 0.5, 0.5, 0.75, 0.5, 0.625, 0.75, 0.875, 0.5).unwrap(), array_flat!(0, 1, 2, 2, 3, 3, 3, 3, 4).unwrap()))),
case(array_arange![-8., 0.], Ok((array_flat!(-0.5, -0.875, -0.75, -0.625, -0.5, -0.75, -0.5, -0.5, 0.).unwrap(), array_flat!(4, 3, 3, 3, 3, 2, 2, 1, 0 ).unwrap()))),
case(array_flat![1., -2., 3., -4.], Ok((array_flat!(0.5, -0.5, 0.75, -0.5).unwrap(), array_flat!(1, 2, 2, 3).unwrap()))),
)] fn test_frexp(array: Result<Array<f64>, ArrayError>, expected: Result<(Array<f64>, Array<i32>), ArrayError>) {
    assert_eq!(expected, array.frexp())
}

#[rstest(
array, other, expected,
case(array![0., 0.5, 0.5, 0.75, 0.5, 0.625, 0.75, 0.875, 0.5], array![0, 1, 2, 2, 3, 3, 3, 3, 4], array_arange![0., 8.]),
case(array![0., -0.5, -0.5, -0.75, -0.5, -0.625, -0.75, -0.875, -0.5], array![0, -1, -2, -2, -3, -3, -3, -3, -4], array![0., -0.25, -0.125, -0.1875, -0.0625, -0.078125, -0.09375, -0.109375, -0.03125]),
case(array![1., -2., 3., -4.], array![0, -1, 1, -1], array![1., -1., 6., -2.]),
)] fn test_ldexp(array: Result<Array<f64>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.ldexp(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array![1., 2., 3., 4.], array![2., 1., 4., 3.], array![1. + f64::EPSILON, 2. - f64::EPSILON, 3. + f64::EPSILON, 4. - f64::EPSILON]),
case(array![-1., -2., -3., -4.], array![-2., -1., -4., -3.], array![-1. - f64::EPSILON, -2. + f64::EPSILON, -3. + f64::EPSILON, -4. - f64::EPSILON]),
case(array![1., -2., 3., -4.], array![2., 1., 4., -3.], array![1. + f64::EPSILON, -2. + f64::EPSILON, 3. + f64::EPSILON, -4. - f64::EPSILON]),
)] fn test_nextafter(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.nextafter(&other.unwrap()))
}

#[rstest(
array, expected,
case(array![1., 2., 3., 4.], array_flat![f64::EPSILON, f64::EPSILON * 2., f64::EPSILON * 2., f64::EPSILON * 4.]),
case(array![-1., -2., -3., -4.], array_flat![f64::EPSILON / 2., f64::EPSILON, f64::EPSILON * 2., f64::EPSILON * 2.]),
case(array![1., -2., 3., -4.], array_flat![f64::EPSILON, f64::EPSILON, f64::EPSILON * 2., f64::EPSILON * 2.]),
)] fn test_spacing(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.spacing())
}

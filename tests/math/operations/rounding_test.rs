use arr_rs::prelude::*;
use rstest::rstest;

#[rstest(
array, decimals, expected,
case(array![2.01, 4.6, 8.0010, 22.234], array![0, 1, 3, -1], array![2.0, 4.6, 8.001, 20.0]),
case(array![3.8, -3.8, 2.3, -2.3], array![0], array![4.0, -4.0, 2.0, -2.0]),
case(array![2.01, 4.6, -1.6, -2.2], array![0], array![2.0, 5.0, -2.0, -2.0]),
)] fn test_round(array: Result<Array<f64>, ArrayError>, decimals: Result<Array<isize>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.round(&decimals.unwrap()));
}

#[rstest(
array, decimals, expected,
case(array![2.01, 4.6, 8.0010, 22.234], array![0, 1, 3, -1], array![2.0, 4.6, 8.001, 20.0]),
case(array![3.8, -3.8, 2.3, -2.3], array![0], array![4.0, -4.0, 2.0, -2.0]),
case(array![2.01, 4.6, -1.6, -2.2], array![0], array![2.0, 5.0, -2.0, -2.0]),
)] fn test_around(array: Result<Array<f64>, ArrayError>, decimals: Result<Array<isize>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.around(&decimals.unwrap()));
}

#[rstest(
array, expected,
case(array![2.01, 4.6, 8.0010, 22.234], array![2.0, 5.0, 8.0, 22.0]),
case(array![3.8, -3.8, 2.3, -2.3], array![4.0, -4.0, 2.0, -2.0]),
case(array![2.01, 4.6, -1.6, -2.2], array![2.0, 5.0, -2.0, -2.0]),
)] fn test_rint(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.rint());
}

#[rstest(
array, expected,
case(array![2.01, 4.6, 8.0010, 22.234], array![2.0, 4.0, 8.0, 22.0]),
case(array![3.8, -3.8, 2.3, -2.3], array![3.0, -3.0, 2.0, -2.0]),
case(array![2.01, 4.6, -1.6, -2.2], array![2.0, 4.0, -1.0, -2.0]),
)] fn test_fix(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.fix());
}

#[rstest(
array, expected,
case(array![2.01, 4.6, 8.0010, 22.234], array![2.0, 4.0, 8.0, 22.0]),
case(array![3.8, -3.8, 2.3, -2.3], array![3.0, -3.0, 2.0, -2.0]),
case(array![2.01, 4.6, -1.6, -2.2], array![2.0, 4.0, -1.0, -2.0]),
)] fn test_trunc(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.trunc());
}

#[rstest(
array, expected,
case(array![2.01, 4.6, -1.6, -2.2], array![2.0, 4.0, -2.0, -3.0]),
case(array![3.8, -3.8, 2.3, -2.3], array![3.0, -4.0, 2.0, -3.0]),
case(array![2.01, 4.6, 8.0010, 22.234], array![2.0, 4.0, 8.0, 22.0]),
)] fn test_floor(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.floor());
}

#[rstest(
array, expected,
case(array![2.01, 4.6, -1.6, -2.2], array![3.0, 5.0, -1.0, -2.0]),
case(array![3.8, -3.8, 2.3, -2.3], array![4.0, -3.0, 3.0, -2.0]),
case(array![2.01, 4.6, 8.0010, 22.234], array![3.0, 5.0, 9.0, 23.0]),
)] fn test_ceil(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.ceil());
}

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

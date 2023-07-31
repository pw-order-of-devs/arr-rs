use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array![1., 2., 3., 4.], array![std::f64::consts::E,  7.38905609893065, 20.085536923187668, 54.598150033144236]),
case(array!([[1., 2.], [3., 4.]]), array!([[std::f64::consts::E,  7.38905609893065], [20.085536923187668, 54.598150033144236]])),
)] fn test_exp(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.exp())
}

#[rstest(
array, expected,
case(array![1., 2., 3., 4.], array![2., 4., 8., 16.]),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 4.], [8., 16.]])),
)] fn test_exp2(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.exp2())
}

#[rstest(
array, expected,
case(array![1., 2., 3., 4.], array![1.718281828459045, 6.38905609893065, 19.085536923187668, 53.598150033144236]),
case(array!([[1., 2.], [3., 4.]]), array!([[1.718281828459045, 6.38905609893065], [19.085536923187668, 53.598150033144236]])),
)] fn test_exp_m1(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.exp_m1())
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
array, expected,
case(array![2., 4., 8., 20.], array![1.0986122886681098, 1.6094379124341003, 2.1972245773362196, 3.044522437723423]),
case(array![[2., 4.], [8., 20.]], array![[1.0986122886681098, 1.6094379124341003], [2.1972245773362196, 3.044522437723423]]),
)] fn test_log_1p(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.log_1p())
}

#[rstest(
array, log, expected,
case(array![2., 4., 8., 20.], array![2., 2., 2., 10.], array![1., 2., 3., 1.301029995663981]),
case(array!([[2., 4.], [8., 20.]]), array!([[2., 2.], [2., 10.]]), array!([[1., 2.], [3., 1.301029995663981]])),
)] fn test_logn(array: Result<Array<f64>, ArrayError>, log: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.logn(&log.unwrap()))
}

#[rstest(
array, value, expected,
case(array![2., 4., 8., 20.], array![2., 2., 2., 10.], array![2.6931471805599454, 4.126928011042972, 8.00247568513773, 20.000045398899218]),
case(array!([[2., 4.], [8., 20.]]), array!([[2., 2.], [2., 10.]]), array!([[2.6931471805599454, 4.126928011042972], [8.00247568513773, 20.000045398899218]])),
)] fn test_log_add_exp(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.log_add_exp(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![2., 4., 8., 20.], array![2., 2., 2., 10.], array![3., 4.321928094887363, 6.087462841250339, 8.965784284662087]),
case(array!([[2., 4.], [8., 20.]]), array!([[2., 2.], [2., 10.]]), array!([[3., 4.321928094887363], [6.087462841250339, 8.965784284662087]])),
)] fn test_log_add_exp2(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.log_add_exp2(&value.unwrap()))
}

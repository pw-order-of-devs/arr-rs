use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, value, expected,
case(array![f64, 1, 2, 3, 4], array![f64, 2], array![f64, 3, 4, 5, 6]),
case(array![f64, 1, 2, 3, 4], array![f64, 3], array![f64, 4, 5, 6, 7]),
case(array!(f64, [[1, 2], [3, 4]]), array!(f64, [[2, 2], [3, 3]]), array!(f64, [[3, 4], [6, 7]])),
)] fn test_add(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.add(&value.unwrap()))
}

#[rstest(
array, expected,
case(array![f64, 1., 2., 4., 10.], array![f64, 1., 0.5, 0.25, 0.1]),
case(array!(f64, [[1., 2.], [4., 10.]]), array!(f64, [[1., 0.5], [0.25, 0.1]])),
)] fn test_reciprocal(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.reciprocal())
}

#[rstest(
array, expected,
case(array![f64, 1., -1., 1., -1.], array![f64, 1., -1., 1., -1.]),
case(array!(f64, [[1., -1.], [1., -1.]]), array!(f64, [[1., -1.], [1., -1.]])),
)] fn test_positive(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.positive())
}

#[rstest(
array, expected,
case(array![f64, 1., -1., 1., -1.], array![f64, -1., 1., -1., 1.]),
case(array!(f64, [[1., -1.], [1., -1.]]), array!(f64, [[-1., 1.], [-1., 1.]])),
)] fn test_negative(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.negative())
}

#[rstest(
array, value, expected,
case(array![f64, 1, 2, 3, 4], array![f64, 2], array![f64, 2, 4, 6, 8]),
case(array![f64, 1, 2, 3, 4], array![f64, 3], array![f64, 3, 6, 9, 12]),
case(array!(f64, [[1, 2], [3, 4]]), array!(f64, [[2, 2], [3, 3]]), array!(f64, [[2, 4], [9, 12]])),
case(array!(f64, [[1, 2, 3], [4, 5, 6], [7, 8, 9]]), array!(f64, [[2, 2, 2], [3, 3, 3], [4, 4, 4]]), array!(f64, [[2, 4, 6], [12, 15, 18], [28, 32, 36]])),
case(array!(f64, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(f64, [[[2, 3], [3, 2]], [[2, 4], [2, 4]]]), array!(f64, [[[2, 6], [9, 8]], [[10, 24], [14, 32]]])),
case(array!(f64, [[[[1], [0]]], [[[0], [1]]]]), array!(f64, [[[[1, 1]], [[1, 1]]]]), array!(f64, [[[[1, 1], [0, 0]], [[1, 1], [0, 0]]], [[[0, 0], [1, 1]], [[0, 0], [1, 1]]]])),
)] fn test_multiply(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.multiply(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![f64, 1., 2., 3., 4.], array![f64, 2.], array![f64, 0.5, 1., 1.5, 2.]),
case(array![f64, 3., 6., 9., 12.], array![f64, 3.], array![f64, 1., 2., 3., 4.]),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 2.], [3., 3.]]), array!(f64, [[0.5, 1.], [1., 2.]])),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 0.], [3., 0.]]), Err(ArrayError::ParameterError { param: "value", message: "cannot contain `0`" })),
)] fn test_divide(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.divide(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![f64, 1., 2., 3., 4.], array![f64, 2.], array![f64, 0.5, 1., 1.5, 2.]),
case(array![f64, 3., 6., 9., 12.], array![f64, 3.], array![f64, 1., 2., 3., 4.]),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 2.], [3., 3.]]), array!(f64, [[0.5, 1.], [1., 2.]])),
)] fn test_true_divide(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.true_divide(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![f64, 1., 2., 3., 4.], array![f64, 2.], array![f64, 0., 1., 1., 2.]),
case(array![f64, 3., 6., 9., 12.], array![f64, 3.], array![f64, 1., 2., 3., 4.]),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 2.], [3., 3.]]), array!(f64, [[0., 1.], [1., 2.]])),
)] fn test_floor_divide(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.floor_divide(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![f64, 1, 2, 3, 4], array![f64, 2], array![f64, 1, 4, 9, 16]),
case(array![f64, 1, 2, 3, 4], array![f64, 3], array![f64, 1, 8, 27, 64]),
case(array!(f64, [[1, 2], [3, 4]]), array!(f64, [[2, 2], [3, 3]]), array!(f64, [[1, 4], [27, 64]])),
)] fn test_power(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.power(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![f64, 1, 2, 3, 4], array![f64, 2], array![f64, 1, 4, 9, 16]),
case(array![f64, 1, 2, 3, 4], array![f64, 3], array![f64, 1, 8, 27, 64]),
case(array!(f64, [[1, 2], [3, 4]]), array!(f64, [[2, 2], [3, 3]]), array!(f64, [[1, 4], [27, 64]])),
)] fn test_float_power(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.float_power(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![f64, 1., 2., 3., 4.], array![f64, 2.], array![f64, -1., 0., 1., 2.]),
case(array![f64, 3., 6., 9., 12.], array![f64, 3.], array![f64, 0., 3., 6., 9.]),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 2.], [3., 3.]]), array!(f64, [[-1., 0.], [0., 3.]])),
)] fn test_subtract(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.subtract(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![f64, 1., 2., 3., 4.], array![f64, 2.], array![f64, 1., 0., 1., 0.]),
case(array![f64, 3., 6., 9., 12.], array![f64, 3.], array![f64, 0., 0., 0., 0.]),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 2.], [3., 3.]]), array!(f64, [[1., 0.], [0., 0.]])),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 0.], [3., 0.]]), Err(ArrayError::ParameterError { param: "value", message: "cannot contain `0`" })),
)] fn test_mod(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.r#mod(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![f64, 1., 2., 3., 4.], array![f64, 2.], array![f64, 1., 0., 1., 0.]),
case(array![f64, 3., 6., 9., 12.], array![f64, 3.], array![f64, 0., 0., 0., 0.]),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 2.], [3., 3.]]), array!(f64, [[1., 0.], [0., 0.]])),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 0.], [3., 0.]]), Err(ArrayError::ParameterError { param: "value", message: "cannot contain `0`" })),
)] fn test_fmod(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.fmod(&value.unwrap()))
}

#[rstest(
array, expected,
case(array![f64, 1.5, 2., 3.5, 4.5], Ok((array![f64, 0.5, 0., 0.5, 0.5].unwrap(), array![f64, 1., 2., 3., 4.].unwrap()))),
case(array!(f64, [[1.5, 2.], [3.5, 4.5]]), Ok((array!(f64, [[0.5, 0.], [0.5, 0.5]]).unwrap(), array!(f64, [[1., 2.], [3., 4.]]).unwrap()))),
)] fn test_modf(array: Result<Array<f64>, ArrayError>, expected: Result<(Array<f64>, Array<f64>), ArrayError>) {
    assert_eq!(expected, array.modf())
}

#[rstest(
array, value, expected,
case(array![f64, 1., 2., 3., 4.], array![f64, 2.], array![f64, 1., 0., 1., 0.]),
case(array![f64, 3., 6., 9., 12.], array![f64, 3.], array![f64, 0., 0., 0., 0.]),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 2.], [3., 3.]]), array!(f64, [[1., 0.], [0., 0.]])),
case(array!(f64, [[1., 2.], [3., 6.]]), array!(f64, [[2., 0.], [3., 0.]]), Err(ArrayError::ParameterError { param: "value", message: "cannot contain `0`" })),
)] fn test_remainder(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.remainder(&value.unwrap()))
}

#[rstest(
array, expected,
case(array![f64, 1.5, 2., 3.5, 4.5], Ok((array![f64, 1., 2., 3., 4.].unwrap(), array![f64, 0.5, 0., 0.5, 0.5].unwrap()))),
case(array!(f64, [[1.5, 2.], [3.5, 4.5]]), Ok((array!(f64, [[1., 2.], [3., 4.]]).unwrap(), array!(f64, [[0.5, 0.], [0.5, 0.5]]).unwrap()))),
)] fn test_divmod(array: Result<Array<f64>, ArrayError>, expected: Result<(Array<f64>, Array<f64>), ArrayError>) {
    assert_eq!(expected, array.divmod())
}

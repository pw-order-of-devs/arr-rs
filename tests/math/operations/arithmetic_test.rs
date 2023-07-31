use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, value, expected,
case(array![1, 2, 3, 4], array![2], array![3, 4, 5, 6]),
case(array![1, 2, 3, 4], array![3], array![4, 5, 6, 7]),
case(array!([[1, 2], [3, 4]]), array!([[2, 2], [3, 3]]), array!([[3, 4], [6, 7]])),
)] fn test_add(array: Result<Array<i32>, ArrayError>, value: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.add(&value.unwrap()))
}

#[rstest(
array, expected,
case(array![1., 2., 4., 10.], array![1., 0.5, 0.25, 0.1]),
case(array!([[1., 2.], [4., 10.]]), array!([[1., 0.5], [0.25, 0.1]])),
)] fn test_reciprocal(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.reciprocal())
}

#[rstest(
array, expected,
case(array![1., -1., 1., -1.], array![1., -1., 1., -1.]),
case(array!([[1., -1.], [1., -1.]]), array!([[1., -1.], [1., -1.]])),
)] fn test_positive(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.positive())
}

#[rstest(
array, expected,
case(array![1., -1., 1., -1.], array![-1., 1., -1., 1.]),
case(array!([[1., -1.], [1., -1.]]), array!([[-1., 1.], [-1., 1.]])),
)] fn test_negative(array: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.negative())
}

#[rstest(
array, value, expected,
case(array![1, 2, 3, 4], array![2], array![2, 4, 6, 8]),
case(array![1, 2, 3, 4], array![3], array![3, 6, 9, 12]),
case(array!([[1, 2], [3, 4]]), array!([[2, 2], [3, 3]]), array!([[2, 4], [9, 12]])),
)] fn test_multiply(array: Result<Array<i32>, ArrayError>, value: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.multiply(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![1., 2., 3., 4.], array![2.], array![0.5, 1., 1.5, 2.]),
case(array![3., 6., 9., 12.], array![3.], array![1., 2., 3., 4.]),
case(array!([[1., 2.], [3., 6.]]), array!([[2., 2.], [3., 3.]]), array!([[0.5, 1.], [1., 2.]])),
)] fn test_divide(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.divide(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![1., 2., 3., 4.], array![2.], array![0.5, 1., 1.5, 2.]),
case(array![3., 6., 9., 12.], array![3.], array![1., 2., 3., 4.]),
case(array!([[1., 2.], [3., 6.]]), array!([[2., 2.], [3., 3.]]), array!([[0.5, 1.], [1., 2.]])),
)] fn test_true_divide(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.true_divide(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![1., 2., 3., 4.], array![2.], array![0., 1., 1., 2.]),
case(array![3., 6., 9., 12.], array![3.], array![1., 2., 3., 4.]),
case(array!([[1., 2.], [3., 6.]]), array!([[2., 2.], [3., 3.]]), array!([[0., 1.], [1., 2.]])),
)] fn test_floor_divide(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.floor_divide(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![1, 2, 3, 4], array![2], array![1, 4, 9, 16]),
case(array![1, 2, 3, 4], array![3], array![1, 8, 27, 64]),
case(array!([[1, 2], [3, 4]]), array!([[2, 2], [3, 3]]), array!([[1, 4], [27, 64]])),
)] fn test_power(array: Result<Array<i32>, ArrayError>, value: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.power(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![1, 2, 3, 4], array![2], array![1, 4, 9, 16]),
case(array![1, 2, 3, 4], array![3], array![1, 8, 27, 64]),
case(array!([[1, 2], [3, 4]]), array!([[2, 2], [3, 3]]), array!([[1, 4], [27, 64]])),
)] fn test_float_power(array: Result<Array<i32>, ArrayError>, value: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.float_power(&value.unwrap()))
}

#[rstest(
array, value, expected,
case(array![1., 2., 3., 4.], array![2.], array![-1., 0., 1., 2.]),
case(array![3., 6., 9., 12.], array![3.], array![0., 3., 6., 9.]),
case(array!([[1., 2.], [3., 6.]]), array!([[2., 2.], [3., 3.]]), array!([[-1., 0.], [0., 3.]])),
)] fn test_subtract(array: Result<Array<f64>, ArrayError>, value: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.subtract(&value.unwrap()))
}

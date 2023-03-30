use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([3., 4., 5., 6.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[3., 4.], [5., 6.]])),
#[should_panic(expected = "Arrays must have the same shape")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[3., 4.], [5., 6.]])),
)] fn test_add(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    assert_eq!(expected, arr1 + arr2);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([3., 4., 5., 6.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[3., 4.], [5., 6.]])),
)] fn test_add_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    assert_eq!(expected, arr + scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([-1., 0., 1., 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[-1., 0.], [1., 2.]])),
#[should_panic(expected = "Arrays must have the same shape")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[-1., 0.], [1., 2.]])),
)] fn test_sub(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    assert_eq!(expected, arr1 - arr2);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([-1., 0., 1., 2.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[-1., 0.], [1., 2.]])),
)] fn test_sub_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    assert_eq!(expected, arr - scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([2., 4., 6., 8.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[2., 4.], [6., 8.]])),
#[should_panic(expected = "Arrays must have the same shape")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[2., 4.], [6., 8.]])),
)] fn test_mul(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    assert_eq!(expected, arr1 * arr2);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([2., 4., 6., 8.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[2., 4.], [6., 8.]])),
)] fn test_mul_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    assert_eq!(expected, arr * scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([0.5, 1., 1.5, 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[0.5, 1.], [1.5, 2.]])),
#[should_panic(expected = "Arrays must have the same shape")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[0.5, 1.], [1.5, 2.]])),
)] fn test_div(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    assert_eq!(expected, arr1 / arr2);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([0.5, 1., 1.5, 2.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[0.5, 1.], [1.5, 2.]])),
)] fn test_div_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    assert_eq!(expected, arr / scalar);
}
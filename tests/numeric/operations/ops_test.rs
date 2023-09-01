use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [3., 4., 5., 6.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[3., 4.], [5., 6.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[3., 4.], [5., 6.]])),
)] fn test_add(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() + arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [3., 4., 5., 6.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[3., 4.], [5., 6.]])),
)] fn test_add_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() + scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [3., 4., 5., 6.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[3., 4.], [5., 6.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[3., 4.], [5., 6.]])),
)] fn test_add_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 += arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [3., 4., 5., 6.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[3., 4.], [5., 6.]])),
)] fn test_add_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr += scalar;
    assert_eq!(expected.unwrap(), arr);
}

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [-1., 0., 1., 2.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[-1., 0.], [1., 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[-1., 0.], [1., 2.]])),
)] fn test_sub(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() - arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [-1., 0., 1., 2.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[-1., 0.], [1., 2.]])),
)] fn test_sub_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() - scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [-1., 0., 1., 2.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[-1., 0.], [1., 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[-1., 0.], [1., 2.]])),
)] fn test_sub_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 -= arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [-1., 0., 1., 2.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[-1., 0.], [1., 2.]])),
)] fn test_sub_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr -= scalar;
    assert_eq!(expected.unwrap(), arr);
}

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [2., 4., 6., 8.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[2., 4.], [6., 8.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[2., 4.], [6., 8.]])),
)] fn test_mul(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() * arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [2., 4., 6., 8.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[2., 4.], [6., 8.]])),
)] fn test_mul_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() * scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [2., 4., 6., 8.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[2., 4.], [6., 8.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[2., 4.], [6., 8.]])),
)] fn test_mul_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 *= arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [2., 4., 6., 8.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[2., 4.], [6., 8.]])),
)] fn test_mul_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr *= scalar;
    assert_eq!(expected.unwrap(), arr);
}

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [0.5, 1., 1.5, 2.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[0.5, 1.], [1.5, 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[0.5, 1.], [1.5, 2.]])),
)] fn test_div(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() / arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [0.5, 1., 1.5, 2.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[0.5, 1.], [1.5, 2.]])),
)] fn test_div_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() / scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [0.5, 1., 1.5, 2.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[0.5, 1.], [1.5, 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[0.5, 1.], [1.5, 2.]])),
)] fn test_div_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 /= arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [0.5, 1., 1.5, 2.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[0.5, 1.], [1.5, 2.]])),
)] fn test_div_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr /= scalar;
    assert_eq!(expected.unwrap(), arr);
}

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [1., 0., 1., 0.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[1., 0.], [1., 0.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[0., 1.], [1., 2.]])),
)] fn test_rem(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() % arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [1., 0., 1., 0.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[1., 0.], [1., 0.]])),
)] fn test_rem_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() % scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [2., 2., 2., 2.]), array!(f64, [1., 0., 1., 0.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[2., 2.], [2., 2.]]), array!(f64, [[1., 0.], [1., 0.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `Ok([2, 2])`,\n right: `Ok([4])`")]
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [2., 2., 2., 2.]), array!(f64, [[1., 0.], [1., 0.]])),
)] fn test_rem_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 %= arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!(f64, [1., 2., 3., 4.]), 2., array!(f64, [1., 0., 1., 0.])),
case(array!(f64, [[1., 2.], [3., 4.]]), 2., array!(f64, [[1., 0.], [1., 0.]])),
)] fn test_rem_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr %= scalar;
    assert_eq!(expected.unwrap(), arr);
}

#[rstest(
arr, expected,
case(array!(f64, [1., 2., 3., 4.]), array!(f64, [-1., -2., -3., -4.])),
case(array!(f64, [[1., 2.], [3., 4.]]), array!(f64, [[-1., -2.], [-3., -4.]])),
)] fn test_neg(arr: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), -arr.unwrap());
}

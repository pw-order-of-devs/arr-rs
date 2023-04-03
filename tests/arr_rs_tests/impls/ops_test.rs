use rstest::rstest;
use arr_rs::prelude::*;

// ==== Compare

#[rstest(
array1, array2, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), true),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![4]), false),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), true),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[4]`,\n right: `[2, 2]`")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
)] fn test_eq(array1: Array<i32>, array2: Array<i32>, expected: bool) {
    assert_eq!(expected, array1 == array2);
}

#[rstest(
array1, array2, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), false),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![4]), true),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), false),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), true),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[4]`,\n right: `[2, 2]`")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
)] fn test_ne(array1: Array<i32>, array2: Array<i32>, expected: bool) {
    assert_eq!(expected, array1 != array2);
}

#[rstest(
array1, array2, expected,
case(Array::new(vec![2, 2, 2, 2], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), std::cmp::Ordering::Greater),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), std::cmp::Ordering::Equal),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![4]), std::cmp::Ordering::Less),
case(Array::new(vec![2, 2, 2, 2], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), std::cmp::Ordering::Greater),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), std::cmp::Ordering::Equal),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), std::cmp::Ordering::Less),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[4]`,\n right: `[2, 2]`")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), std::cmp::Ordering::Greater),
)] fn test_partial_cmp(array1: Array<i32>, array2: Array<i32>, expected: std::cmp::Ordering) {
    assert_eq!(expected, array1.partial_cmp(&array2).unwrap());
}

#[rstest(
array1, array2, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), false),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![4]), true),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), false),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), true),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[4]`,\n right: `[2, 2]`")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
)] fn test_lt(array1: Array<i32>, array2: Array<i32>, expected: bool) {
    assert_eq!(expected, array1.lt(&array2));
}

#[rstest(
array1, array2, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), true),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![4]), true),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), true),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), true),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[4]`,\n right: `[2, 2]`")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
)] fn test_le(array1: Array<i32>, array2: Array<i32>, expected: bool) {
    assert_eq!(expected, array1.le(&array2));
}

#[rstest(
array1, array2, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), false),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![4]), false),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), false),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[4]`,\n right: `[2, 2]`")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
)] fn test_gt(array1: Array<i32>, array2: Array<i32>, expected: bool) {
    assert_eq!(expected, array1.gt(&array2));
}

#[rstest(
array1, array2, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), true),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![4]), false),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), true),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[4]`,\n right: `[2, 2]`")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
)] fn test_ge(array1: Array<i32>, array2: Array<i32>, expected: bool) {
    assert_eq!(expected, array1.ge(&array2));
}

// ==== Add

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([3., 4., 5., 6.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[3., 4.], [5., 6.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
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
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([3., 4., 5., 6.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[3., 4.], [5., 6.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[3., 4.], [5., 6.]])),
)] fn test_add_assign(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    let mut arr1 = arr1;
    arr1 += arr2;
    assert_eq!(expected, arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([3., 4., 5., 6.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[3., 4.], [5., 6.]])),
)] fn test_add_assign_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    let mut arr = arr;
    arr += scalar;
    assert_eq!(expected, arr);
}

// ==== Sub

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([-1., 0., 1., 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[-1., 0.], [1., 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
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
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([-1., 0., 1., 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[-1., 0.], [1., 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[-1., 0.], [1., 2.]])),
)] fn test_sub_assign(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    let mut arr1 = arr1;
    arr1 -= arr2;
    assert_eq!(expected, arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([-1., 0., 1., 2.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[-1., 0.], [1., 2.]])),
)] fn test_sub_assign_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    let mut arr = arr;
    arr -= scalar;
    assert_eq!(expected, arr);
}

// ==== Mul

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([2., 4., 6., 8.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[2., 4.], [6., 8.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
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
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([2., 4., 6., 8.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[2., 4.], [6., 8.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[2., 4.], [6., 8.]])),
)] fn test_mul_assign(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    let mut arr1 = arr1;
    arr1 *= arr2;
    assert_eq!(expected, arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([2., 4., 6., 8.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[2., 4.], [6., 8.]])),
)] fn test_mul_assign_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    let mut arr = arr;
    arr *= scalar;
    assert_eq!(expected, arr);
}

// ==== Div

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([0.5, 1., 1.5, 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[0.5, 1.], [1.5, 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
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

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([0.5, 1., 1.5, 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[0.5, 1.], [1.5, 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[0.5, 1.], [1.5, 2.]])),
)] fn test_div_assign(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    let mut arr1 = arr1;
    arr1 /= arr2;
    assert_eq!(expected, arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([0.5, 1., 1.5, 2.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[0.5, 1.], [1.5, 2.]])),
)] fn test_div_assign_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    let mut arr = arr;
    arr /= scalar;
    assert_eq!(expected, arr);
}

// ==== Rem

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([1., 0., 1., 0.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[1., 0.], [1., 0.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[0., 1.], [1., 2.]])),
)] fn test_rem(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    assert_eq!(expected, arr1 % arr2);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([1., 0., 1., 0.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[1., 0.], [1., 0.]])),
)] fn test_rem_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    assert_eq!(expected, arr % scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([1., 0., 1., 0.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[1., 0.], [1., 0.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[1., 0.], [1., 0.]])),
)] fn test_rem_assign(arr1: Array<f64>, arr2: Array<f64>, expected: Array<f64>) {
    let mut arr1 = arr1;
    arr1 %= arr2;
    assert_eq!(expected, arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([1., 0., 1., 0.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[1., 0.], [1., 0.]])),
)] fn test_rem_assign_scalar(arr: Array<f64>, scalar: f64, expected: Array<f64>) {
    let mut arr = arr;
    arr %= scalar;
    assert_eq!(expected, arr);
}

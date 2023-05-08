use rstest::rstest;
use arr_rs::prelude::*;

// ==== Indexing

#[rstest(
array, index, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), 0, 1),
case(Array::new(vec![1, 2, 3, 4], vec![4]), 2, 3),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), 1, 2),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), 3, 4),
#[should_panic(expected = "index out of bounds: the len is 4 but the index is 8")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), 8, 0),
)] fn test_index(array: Result<Array<i32>, ArrayError>, index: usize, expected: i32) {
    assert_eq!(expected, array.unwrap()[index]);
}

#[rstest(
array, index, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[0], 1),
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[3], 4),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[0, 0], 1),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[1, 1], 4),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), &[1, 1], 5),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), &[2, 0], 5),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[0, 0, 1], 2),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[1, 1, 1], 8),
#[should_panic(expected = "parameter error: `coords`: length must match array dimension")]
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[2, 3, 4], 0),
#[should_panic(expected = "parameter error: `coords`: value must match array shape")]
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[2, 3, 4], 0),
)] fn test_index_coord(array: Result<Array<i32>, ArrayError>, index: &[usize], expected: i32) {
    assert_eq!(expected, array.unwrap()[index]);
}

#[rstest(
array, index, expected,
case(&mut Array::new(vec![1, 2, 3, 4], vec![4]), 0, 1),
case(&mut Array::new(vec![1, 2, 3, 4], vec![4]), 2, 3),
case(&mut Array::new(vec![1, 2, 3, 4], vec![2, 2]), 1, 2),
case(&mut Array::new(vec![1, 2, 3, 4], vec![2, 2]), 3, 4),
#[should_panic(expected = "index out of bounds: the len is 4 but the index is 8")]
case(&mut Array::new(vec![1, 2, 3, 4], vec![4]), 8, 0),
)] fn test_index_mut(array: &mut Result<Array<i32>, ArrayError>, index: usize, expected: i32) {
    assert_eq!(expected, array.as_ref().unwrap().clone()[index]);
}

#[rstest(
array, index, expected,
case(&mut Array::new(vec![1, 2, 3, 4], vec![4]), &[0], 1),
case(&mut Array::new(vec![1, 2, 3, 4], vec![4]), &[3], 4),
case(&mut Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[0, 0], 1),
case(&mut Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[1, 1], 4),
case(&mut Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), &[1, 1], 5),
case(&mut Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), &[2, 0], 5),
case(&mut Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[0, 0, 1], 2),
case(&mut Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[1, 1, 1], 8),
#[should_panic(expected = "parameter error: `coords`: length must match array dimension")]
case(&mut Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[2, 3, 4], 0),
#[should_panic(expected = "parameter error: `coords`: value must match array shape")]
case(&mut Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[2, 3, 4], 0),
)] fn test_index_mut_coord(array: &mut Result<Array<i32>, ArrayError>, index: &[usize], expected: i32) {
    assert_eq!(expected, array.as_ref().unwrap().clone()[index]);
}

// ==== Compare

#[rstest(
array1, array2, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), true),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![4]), false),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), true),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[4]`,\n right: `[2, 2]`")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
)] fn test_eq(array1: Result<Array<i32>, ArrayError>, array2: Result<Array<i32>, ArrayError>, expected: bool) {
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
)] fn test_ne(array1: Result<Array<i32>, ArrayError>, array2: Result<Array<i32>, ArrayError>, expected: bool) {
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
)] fn test_partial_cmp(array1: Result<Array<i32>, ArrayError>, array2: Result<Array<i32>, ArrayError>, expected: std::cmp::Ordering) {
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
)] fn test_lt(array1: Result<Array<i32>, ArrayError>, array2: Result<Array<i32>, ArrayError>, expected: bool) {
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
)] fn test_le(array1: Result<Array<i32>, ArrayError>, array2: Result<Array<i32>, ArrayError>, expected: bool) {
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
)] fn test_gt(array1: Result<Array<i32>, ArrayError>, array2: Result<Array<i32>, ArrayError>, expected: bool) {
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
)] fn test_ge(array1: Result<Array<i32>, ArrayError>, array2: Result<Array<i32>, ArrayError>, expected: bool) {
    assert_eq!(expected, array1.ge(&array2));
}

// ==== Add

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([3., 4., 5., 6.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[3., 4.], [5., 6.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[3., 4.], [5., 6.]])),
)] fn test_add(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() + arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([3., 4., 5., 6.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[3., 4.], [5., 6.]])),
)] fn test_add_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() + scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([3., 4., 5., 6.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[3., 4.], [5., 6.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[3., 4.], [5., 6.]])),
)] fn test_add_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 += arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([3., 4., 5., 6.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[3., 4.], [5., 6.]])),
)] fn test_add_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr += scalar;
    assert_eq!(expected.unwrap(), arr);
}

// ==== Sub

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([-1., 0., 1., 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[-1., 0.], [1., 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[-1., 0.], [1., 2.]])),
)] fn test_sub(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() - arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([-1., 0., 1., 2.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[-1., 0.], [1., 2.]])),
)] fn test_sub_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() - scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([-1., 0., 1., 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[-1., 0.], [1., 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[-1., 0.], [1., 2.]])),
)] fn test_sub_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 -= arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([-1., 0., 1., 2.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[-1., 0.], [1., 2.]])),
)] fn test_sub_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr -= scalar;
    assert_eq!(expected.unwrap(), arr);
}

// ==== Mul

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([2., 4., 6., 8.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[2., 4.], [6., 8.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[2., 4.], [6., 8.]])),
)] fn test_mul(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() * arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([2., 4., 6., 8.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[2., 4.], [6., 8.]])),
)] fn test_mul_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() * scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([2., 4., 6., 8.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[2., 4.], [6., 8.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[2., 4.], [6., 8.]])),
)] fn test_mul_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 *= arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([2., 4., 6., 8.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[2., 4.], [6., 8.]])),
)] fn test_mul_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr *= scalar;
    assert_eq!(expected.unwrap(), arr);
}

// ==== Div

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([0.5, 1., 1.5, 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[0.5, 1.], [1.5, 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[0.5, 1.], [1.5, 2.]])),
)] fn test_div(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() / arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([0.5, 1., 1.5, 2.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[0.5, 1.], [1.5, 2.]])),
)] fn test_div_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() / scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([0.5, 1., 1.5, 2.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[0.5, 1.], [1.5, 2.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[0.5, 1.], [1.5, 2.]])),
)] fn test_div_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 /= arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([0.5, 1., 1.5, 2.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[0.5, 1.], [1.5, 2.]])),
)] fn test_div_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr /= scalar;
    assert_eq!(expected.unwrap(), arr);
}

// ==== Rem

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([1., 0., 1., 0.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[1., 0.], [1., 0.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[0., 1.], [1., 2.]])),
)] fn test_rem(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() % arr2.unwrap());
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([1., 0., 1., 0.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[1., 0.], [1., 0.]])),
)] fn test_rem_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, arr.unwrap() % scalar);
}

#[rstest(
arr1, arr2, expected,
case(array!([1., 2., 3., 4.]), array!([2., 2., 2., 2.]), array!([1., 0., 1., 0.])),
case(array!([[1., 2.], [3., 4.]]), array!([[2., 2.], [2., 2.]]), array!([[1., 0.], [1., 0.]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[1., 2.], [3., 4.]]), array!([2., 2., 2., 2.]), array!([[1., 0.], [1., 0.]])),
)] fn test_rem_assign(arr1: Result<Array<f64>, ArrayError>, arr2: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    let mut arr1 = arr1.unwrap();
    arr1 %= arr2.unwrap();
    assert_eq!(expected.unwrap(), arr1);
}

#[rstest(
arr, scalar, expected,
case(array!([1., 2., 3., 4.]), 2., array!([1., 0., 1., 0.])),
case(array!([[1., 2.], [3., 4.]]), 2., array!([[1., 0.], [1., 0.]])),
)] fn test_rem_assign_scalar(arr: Result<Array<f64>, ArrayError>, scalar: f64, expected: Result<Array<f64>, ArrayError>) {
    let mut arr = arr.unwrap();
    arr %= scalar;
    assert_eq!(expected.unwrap(), arr);
}

// ==== Neg

#[rstest(
arr, expected,
case(array!([1., 2., 3., 4.]), array!([-1., -2., -3., -4.])),
case(array!([[1., 2.], [3., 4.]]), array!([[-1., -2.], [-3., -4.]])),
)] fn test_neg(arr: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected.unwrap(), -arr.unwrap());
}

// ==== Not

#[rstest(
arr, expected,
case(array!([true, false, true, false]), array!([false, true, false, true])),
case(array!([[true, false], [true, false]]), array!([[false, true], [false, true]])),
)] fn test_not(arr: Result<Array<bool>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), !arr.unwrap());
}

// ==== BitAnd

#[rstest(
arr1, arr2, expected,
case(array!([true, false, true, false]), array!([true, true, false, false]), array!([true, false, false, false])),
case(array!([[true, false], [true, false]]), array!([[true, true], [false, false]]), array!([[true, false], [false, false]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[true, false], [true, false]]), array!([true, false, true, false]), array!([[true, false], [true, false]])),
)] fn test_bitand(arr1: Result<Array<bool>, ArrayError>, arr2: Result<Array<bool>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() & arr2.unwrap());
}

#[rstest(
arr, value, expected,
case(array!([true, false, true, false]), true, array!([true, false, true, false])),
case(array!([[true, false], [true, false]]), true, array!([[true, false], [true, false]])),
)] fn test_bitand_value(arr: Result<Array<bool>, ArrayError>, value: bool, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr.unwrap() & value);
}

// ==== BitOr

#[rstest(
arr1, arr2, expected,
case(array!([true, false, true, false]), array!([true, true, false, false]), array!([true, true, true, false])),
case(array!([[true, false], [true, false]]), array!([[true, true], [false, false]]), array!([[true, true], [true, false]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[true, false], [true, false]]), array!([true, false, true, false]), array!([[true, false], [true, false]])),
)] fn test_bitor(arr1: Result<Array<bool>, ArrayError>, arr2: Result<Array<bool>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() | arr2.unwrap());
}

#[rstest(
arr, value, expected,
case(array!([true, false, true, false]), true, array!([true, true, true, true])),
case(array!([true, false, true, false]), false, array!([true, false, true, false])),
case(array!([[true, false], [true, false]]), true, array!([[true, true], [true, true]])),
case(array!([[true, false], [true, false]]), false, array!([[true, false], [true, false]])),
)] fn test_bitor_value(arr: Result<Array<bool>, ArrayError>, value: bool, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr.unwrap() | value);
}

// ==== BitXor

#[rstest(
arr1, arr2, expected,
case(array!([true, false, true, false]), array!([true, true, false, false]), array!([false, true, true, false])),
case(array!([[true, false], [true, false]]), array!([[true, true], [false, false]]), array!([[false, true], [true, false]])),
#[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[2, 2]`,\n right: `[4]`")]
case(array!([[true, false], [true, false]]), array!([true, false, true, false]), array!([[true, false], [true, false]])),
)] fn test_bitxor(arr1: Result<Array<bool>, ArrayError>, arr2: Result<Array<bool>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() ^ arr2.unwrap());
}

#[rstest(
arr, value, expected,
case(array!([true, false, true, false]), true, array!([false, true, false, true])),
case(array!([true, false, true, false]), false, array!([true, false, true, false])),
case(array!([[true, false], [true, false]]), true, array!([[false, true], [false, true]])),
case(array!([[true, false], [true, false]]), false, array!([[true, false], [true, false]])),
)] fn test_bitxor_value(arr: Result<Array<bool>, ArrayError>, value: bool, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr.unwrap() ^ value);
}

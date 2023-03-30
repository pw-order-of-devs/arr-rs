use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array1, array2, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![1, 2, 3, 4], vec![4]), true),
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![4]), false),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![1, 2, 3, 4], vec![2, 2]), true),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
#[should_panic(expected = "Cannot compare arrays of different shape: [4] and [2, 2]")]
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
#[should_panic(expected = "Cannot compare arrays of different shape: [4] and [2, 2]")]
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
#[should_panic(expected = "Cannot compare arrays of different shape: [4] and [2, 2]")]
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
#[should_panic(expected = "Cannot compare arrays of different shape: [4] and [2, 2]")]
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
#[should_panic(expected = "Cannot compare arrays of different shape: [4] and [2, 2]")]
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
#[should_panic(expected = "Cannot compare arrays of different shape: [4] and [2, 2]")]
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
#[should_panic(expected = "Cannot compare arrays of different shape: [4] and [2, 2]")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
)] fn test_ge(array1: Array<i32>, array2: Array<i32>, expected: bool) {
    assert_eq!(expected, array1.ge(&array2));
}
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
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([4])\n right: Ok([2, 2])")]
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
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([4])\n right: Ok([2, 2])")]
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
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([4])\n right: Ok([2, 2])")]
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
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([4])\n right: Ok([2, 2])")]
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
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([4])\n right: Ok([2, 2])")]
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
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([4])\n right: Ok([2, 2])")]
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
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([4])\n right: Ok([2, 2])")]
case(Array::new(vec![1, 2, 3, 4], vec![4]), Array::new(vec![2, 2, 2, 2], vec![2, 2]), false),
)] fn test_ge(array1: Result<Array<i32>, ArrayError>, array2: Result<Array<i32>, ArrayError>, expected: bool) {
    assert_eq!(expected, array1.ge(&array2));
}

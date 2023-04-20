use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, coords, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[0], 0),
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[3], 3),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[0, 0], 0),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[1, 1], 3),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), &[1, 1], 4),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), &[2, 0], 4),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[0, 0, 1], 1),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[1, 1, 1], 7),
#[should_panic(expected = "coords length must match array dimension")]
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[2, 3, 4], 0),
#[should_panic(expected = "coord value must match array shape")]
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[2, 3, 4], 0),
)] fn test_index_at(array: Array<i32>, coords: &[usize], expected: usize) {
    assert_eq!(expected, array.index_at(coords))
}

#[rstest(
array, idx, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), 0, &[0]),
case(Array::new(vec![1, 2, 3, 4], vec![4]), 3, &[3]),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), 0, &[0, 0]),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), 3, &[1, 1]),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), 4, &[1, 1]),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), 4, &[2, 0]),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 1, &[0, 0, 1]),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 7, &[1, 1, 1]),
)] fn test_index_to_coord(array: Array<i32>, idx: usize, expected: &[usize]) {
    assert_eq!(expected, array.index_to_coord(idx))
}

#[rstest(
array, coords, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[0], 1),
case(Array::new(vec![1, 2, 3, 4], vec![4]), &[3], 4),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[0, 0], 1),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[1, 1], 4),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![2, 3]), &[1, 1], 5),
case(Array::new(vec![1, 2, 3, 4, 5, 6], vec![3, 2]), &[2, 0], 5),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[0, 0, 1], 2),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[1, 1, 1], 8),
#[should_panic(expected = "coords length must match array dimension")]
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), &[2, 3, 4], 0),
#[should_panic(expected = "coord value must match array shape")]
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), &[2, 3, 4], 0),
)] fn test_at(array: Array<i32>, coords: &[usize], expected: i32) {
    assert_eq!(expected, array.at(coords))
}

#[rstest(
array, range, expected,
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![8]), 0 .. 4, array!([1, 2, 3, 4])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![8]), 0 .. 6, array!([1, 2, 3, 4, 5, 6])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 2]), 0 .. 1, array!([1, 2])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 2]), 0 .. 2, array!([[1, 2], [3, 4]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 2]), 1 .. 2, array!([3, 4])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![4, 2]), 2 .. 4, array!([[5, 6], [7, 8]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]), 0 .. 1, array!([1, 2, 3, 4])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]), 0 .. 2, array!([[1, 2, 3, 4], [5, 6, 7, 8]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 0 .. 1, array!([[1, 2], [3, 4]])),
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 2 .. 3, array!([[5, 6], [7, 8]])),
#[should_panic(expected = "Slice range out of bounds")]
case(Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 2, 2]), 3 .. 10, array!([0])),
)] fn test_slice(array: Array<i32>, range: std::ops::Range<usize>, expected: Array<i32>) {
    assert_eq!(expected, array.slice(range))
}

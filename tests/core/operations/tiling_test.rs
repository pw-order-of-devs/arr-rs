use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, repeats, axis, expected,
case(array_flat!(i32, 3), vec![4], None, array_flat!(i32, 3, 3, 3, 3)),
case(array_flat!(i32, 1, 2), vec![2], None, array_flat!(i32, 1, 1, 2, 2)),
case(array_flat!(i32, 1, 2), vec![1, 2], None, array_flat!(i32, 1, 2, 2)),
case(array!(i32, [[1, 2], [3, 4]]), vec![1, 2], Some(0), array!(i32, [[1, 2], [3, 4], [3, 4]])),
case(array!(i32, [[1, 2], [3, 4]]), vec![1, 2], Some(1), array!(i32, [[1, 2, 2], [3, 4, 4]])),
case(array!(i32, [[1, 2, 3], [3, 4, 5]]), vec![1, 2], Some(0), array!(i32, [[1, 2, 3], [3, 4, 5], [3, 4, 5]])),
case(array!(i32, [[1, 2, 3], [3, 4, 5]]), vec![1, 2, 1], Some(1), array!(i32, [[1, 2, 2, 3], [3, 4, 4, 5]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![1, 2], Some(0), array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[5, 6], [7, 8]]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![1, 2], Some(1), array!(i32, [[[1, 2], [3, 4], [3, 4]], [[5, 6], [7, 8], [7, 8]]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), vec![1, 2], Some(2), array!(i32, [[[1, 2, 2], [3, 4, 4]], [[5, 6, 6], [7, 8, 8]]])),
case(array_arange!(i32, 1, 24).reshape(&[2, 3, 4]), vec![1, 2], Some(0), array!(i32, [[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]], [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]], [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]]])),
case(array_arange!(i32, 1, 24).reshape(&[2, 3, 4]), vec![1, 2, 1], Some(1), array!(i32, [[[1, 2, 3, 4], [5, 6, 7, 8], [5, 6, 7, 8], [9, 10, 11, 12]], [[13, 14, 15, 16], [17, 18, 19, 20], [17, 18, 19, 20], [21, 22, 23, 24]]])),
case(array_arange!(i32, 1, 24).reshape(&[2, 3, 4]), vec![1, 2, 1, 1], Some(2), array!(i32, [[[1, 2, 2, 3, 4], [5, 6, 6, 7, 8], [9, 10, 10, 11, 12]], [[13, 14, 14, 15, 16], [17, 18, 18, 19, 20], [21, 22, 22, 23, 24]]])),
)] fn test_repeat(array: Result<Array<i32>, ArrayError>, repeats: Vec<usize>, axis: Option<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.repeat(&repeats, axis))
}

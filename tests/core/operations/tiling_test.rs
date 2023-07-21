use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, repeats, axis, expected,
case(array_flat!(3), array_flat!(4), None, array_flat!(3, 3, 3, 3)),
case(array_flat!(1, 2), array_flat!(2), None, array_flat!(1, 1, 2, 2)),
case(array_flat!(1, 2), array_flat!(1, 2), None, array_flat!(1, 2, 2)),
case(array!([[1, 2], [3, 4]]), array_flat!(1, 2), Some(0), array!([[1, 2], [3, 4], [3, 4]])),
case(array!([[1, 2], [3, 4]]), array_flat!(1, 2), Some(1), array!([[1, 2, 2], [3, 4, 4]])),
case(array!([[1, 2, 3], [3, 4, 5]]), array_flat!(1, 2), Some(0), array!([[1, 2, 3], [3, 4, 5], [3, 4, 5]])),
case(array!([[1, 2, 3], [3, 4, 5]]), array_flat!(1, 2, 1), Some(1), array!([[1, 2, 2, 3], [3, 4, 4, 5]])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array_flat!(1, 2), Some(0), array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[5, 6], [7, 8]]])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array_flat!(1, 2), Some(1), array!([[[1, 2], [3, 4], [3, 4]], [[5, 6], [7, 8], [7, 8]]])),
case(array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array_flat!(1, 2), Some(2), array!([[[1, 2, 2], [3, 4, 4]], [[5, 6, 6], [7, 8, 8]]])),
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), array_flat!(1, 2), Some(0), array!([[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]], [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]], [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]]])),
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), array_flat!(1, 2, 1), Some(1), array!([[[1, 2, 3, 4], [5, 6, 7, 8], [5, 6, 7, 8], [9, 10, 11, 12]], [[13, 14, 15, 16], [17, 18, 19, 20], [17, 18, 19, 20], [21, 22, 23, 24]]])),
case(array_arange!(1, 24).reshape(vec![2, 3, 4]), array_flat!(1, 2, 1, 1), Some(2), array!([[[1, 2, 2, 3, 4], [5, 6, 6, 7, 8], [9, 10, 10, 11, 12]], [[13, 14, 14, 15, 16], [17, 18, 18, 19, 20], [21, 22, 22, 23, 24]]])),
)] fn test_repeat(array: Result<Array<i32>, ArrayError>, repeats: Result<Array<usize>, ArrayError>, axis: Option<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.repeat(&repeats.unwrap(), axis))
}

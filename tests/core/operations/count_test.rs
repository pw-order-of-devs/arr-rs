use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, axis, keepdims, expected,
case(array_eye!(4), None, None, array!(4)),
case(array!([[0, 1, 7, 0], [3, 0, 1, 19]]), None, None, array!(5)),
case(array!([[0, 1, 7, 0], [3, 0, 1, 19]]), None, Some(true), array!([[5]])),
case(array!([[0, 1, 7, 0], [3, 0, 1, 19]]), Some(0), None, array!(1, 1, 2, 1)),
case(array!([[0, 1, 7, 0], [3, 0, 1, 19]]), Some(0), Some(true), array!([[1, 1, 2, 1]])),
case(array!([[0, 1, 7, 0], [3, 0, 1, 19]]), Some(1), None, array!(2, 3)),
case(array!([[0, 1, 7, 0], [3, 0, 1, 19]]), Some(1), Some(true), array!([[2], [3]])),
case(array!([[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), None, None, array!(8)),
case(array!([[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), None, Some(true), array!([[[8]]])),
case(array!([[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(0), None, array!([[2, 1, 1], [2, 0, 2]])),
case(array!([[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(0), Some(true), array!([[[2, 1, 1], [2, 0, 2]]])),
case(array!([[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(1), None, array!([[2, 1, 2], [2, 0, 1]])),
case(array!([[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(1), Some(true), array!([[[2, 1, 2]], [[2, 0, 1]]])),
case(array!([[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(2), None, array!([[3, 2], [1, 2]])),
case(array!([[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(2), Some(true), array!([[[3], [2]], [[1], [2]]])),
)] fn test_count_nonzero(arr: Result<Array<i32>, ArrayError>, axis: Option<isize>, keepdims: Option<bool>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, arr.count_nonzero(axis, keepdims))
}

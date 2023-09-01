use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, axis, keepdims, expected,
case(array_eye!(i32, 4), None, None, array!(usize, 4)),
case(array!(i32, [[0, 1, 7, 0], [3, 0, 1, 19]]), None, None, array!(usize, 5)),
case(array!(i32, [[0, 1, 7, 0], [3, 0, 1, 19]]), None, Some(true), array!(usize, [[5]])),
case(array!(i32, [[0, 1, 7, 0], [3, 0, 1, 19]]), Some(0), None, array!(usize, 1, 1, 2, 1)),
case(array!(i32, [[0, 1, 7, 0], [3, 0, 1, 19]]), Some(0), Some(true), array!(usize, [[1, 1, 2, 1]])),
case(array!(i32, [[0, 1, 7, 0], [3, 0, 1, 19]]), Some(1), None, array!(usize, 2, 3)),
case(array!(i32, [[0, 1, 7, 0], [3, 0, 1, 19]]), Some(1), Some(true), array!(usize, [[2], [3]])),
case(array!(i32, [[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), None, None, array!(usize, 8)),
case(array!(i32, [[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), None, Some(true), array!(usize, [[[8]]])),
case(array!(i32, [[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(0), None, array!(usize, [[2, 1, 1], [2, 0, 2]])),
case(array!(i32, [[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(0), Some(true), array!(usize, [[[2, 1, 1], [2, 0, 2]]])),
case(array!(i32, [[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(1), None, array!(usize, [[2, 1, 2], [2, 0, 1]])),
case(array!(i32, [[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(1), Some(true), array!(usize, [[[2, 1, 2]], [[2, 0, 1]]])),
case(array!(i32, [[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(2), None, array!(usize, [[3, 2], [1, 2]])),
case(array!(i32, [[[1, 2, 3], [3, 0, 3]], [[2, 0, 0], [5, 0, 6]]]), Some(2), Some(true), array!(usize, [[[3], [2]], [[1], [2]]])),
)] fn test_count_nonzero(arr: Result<Array<i32>, ArrayError>, axis: Option<isize>, keepdims: Option<bool>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, arr.count_nonzero(axis, keepdims))
}

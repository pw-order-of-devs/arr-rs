use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, axis, kind, expected,
case(Array::empty(), None, None, Array::empty()),
case(array!(i32, 3, 2, 4, 1), None, None, array!(i32, 1, 2, 3, 4)),
case(array!(i32, 3, 2, 4, 1), None, Some(SortKind::Quicksort), array!(i32, 1, 2, 3, 4)),
case(array!(i32, 3, 2, 4, 1), None, Some(SortKind::Mergesort), array!(i32, 1, 2, 3, 4)),
case(array!(i32, 3, 2, 4, 1), None, Some(SortKind::Heapsort), array!(i32, 1, 2, 3, 4)),
case(array!(i32, 3, 2, 4, 1), None, Some(SortKind::Stable), array!(i32, 1, 2, 3, 4)),
case(array!(i32, [[3, 2], [4, 1]]), None, None, array!(i32, [1, 2, 3, 4])),
case(array!(i32, [[3, 2], [4, 1]]), Some(0), None, array!(i32, [[3, 1], [4, 2]])),
case(array!(i32, [[3, 2], [4, 1]]), Some(1), None, array!(i32, [[2, 3], [1, 4]])),
case(array!(i32, [[3, 2], [4, 1]]), Some(-1), None, array!(i32, [[2, 3], [1, 4]])),
)] fn test_sort(array: Result<Array<i32>, ArrayError>, axis: Option<isize>, kind: Option<SortKind>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.sort(axis, kind))
}

#[rstest(
array, axis, kind, expected,
case(Array::empty(), None, None, Array::empty()),
case(array!(i32, 3, 2, 4, 1), None, None, array!(usize, 2, 1, 3, 0)),
case(array!(i32, 3, 2, 4, 1), None, Some(SortKind::Quicksort), array!(usize, 2, 1, 3, 0)),
case(array!(i32, 3, 2, 4, 1), None, Some(SortKind::Mergesort), array!(usize, 2, 1, 3, 0)),
case(array!(i32, 3, 2, 4, 1), None, Some(SortKind::Heapsort), array!(usize, 2, 1, 3, 0)),
case(array!(i32, 3, 2, 4, 1), None, Some(SortKind::Stable), array!(usize, 2, 1, 3, 0)),
case(array!(i32, [[3, 2], [4, 1]]), None, None, array!(usize, [2, 1, 3, 0])),
case(array!(i32, [[3, 2], [4, 1]]), Some(0), None, array!(usize, [[0, 1], [1, 0]])),
case(array!(i32, [[3, 2], [4, 1]]), Some(1), None, array!(usize, [[1, 0], [1, 0]])),
case(array!(i32, [[3, 2], [4, 1]]), Some(-1), None, array!(usize, [[1, 0], [1, 0]])),
)] fn test_argsort(array: Result<Array<i32>, ArrayError>, axis: Option<isize>, kind: Option<SortKind>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, array.argsort(axis, kind))
}

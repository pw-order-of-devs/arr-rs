use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, axis, kind, expected,
case(Array::empty(), None, None, Array::empty()),
case(array!(3, 2, 4, 1), None, None, array!(1, 2, 3, 4)),
case(array!(3, 2, 4, 1), None, Some(SortKind::Quicksort), array!(1, 2, 3, 4)),
case(array!(3, 2, 4, 1), None, Some(SortKind::Mergesort), array!(1, 2, 3, 4)),
case(array!(3, 2, 4, 1), None, Some(SortKind::Heapsort), array!(1, 2, 3, 4)),
case(array!(3, 2, 4, 1), None, Some(SortKind::Stable), array!(1, 2, 3, 4)),
case(array!([[3, 2], [4, 1]]), None, None, array!([1, 2, 3, 4])),
case(array!([[3, 2], [4, 1]]), Some(0), None, array!([[3, 1], [4, 2]])),
case(array!([[3, 2], [4, 1]]), Some(1), None, array!([[2, 3], [1, 4]])),
case(array!([[3, 2], [4, 1]]), Some(-1), None, array!([[2, 3], [1, 4]])),
)] fn test_sort(array: Result<Array<i32>, ArrayError>, axis: Option<isize>, kind: Option<SortKind>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.sort(axis, kind))
}

#[rstest(
array, axis, kind, expected,
case(Array::empty(), None, None, Array::empty()),
case(array!(3, 2, 4, 1), None, None, array!(2, 1, 3, 0)),
case(array!(3, 2, 4, 1), None, Some(SortKind::Quicksort), array!(2, 1, 3, 0)),
case(array!(3, 2, 4, 1), None, Some(SortKind::Mergesort), array!(2, 1, 3, 0)),
case(array!(3, 2, 4, 1), None, Some(SortKind::Heapsort), array!(2, 1, 3, 0)),
case(array!(3, 2, 4, 1), None, Some(SortKind::Stable), array!(2, 1, 3, 0)),
case(array!([[3, 2], [4, 1]]), None, None, array!([2, 1, 3, 0])),
case(array!([[3, 2], [4, 1]]), Some(0), None, array!([[0, 1], [1, 0]])),
case(array!([[3, 2], [4, 1]]), Some(1), None, array!([[1, 0], [1, 0]])),
case(array!([[3, 2], [4, 1]]), Some(-1), None, array!([[1, 0], [1, 0]])),
)] fn test_argsort(array: Result<Array<i32>, ArrayError>, axis: Option<isize>, kind: Option<SortKind>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, array.argsort(axis, kind))
}

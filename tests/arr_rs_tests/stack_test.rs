use rstest::rstest;
use arr_rs::prelude::*;
use arr_rs::traits::manipulate::stack::ArrayStack;

#[rstest(
arrs, axis, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], None, array!([1, 2, 3, 4, 5, 6])),
case(vec![array!([[1, 2], [3, 4]]), array!([5, 6])], None, array!([1, 2, 3, 4, 5, 6])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6]])], Some(0), array!([[1, 2], [3, 4], [5, 6]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6]]), array!([[7, 8]])], Some(0), array!([[1, 2], [3, 4], [5, 6], [7, 8]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5], [6]])], Some(1), array!([[1, 2, 5], [3, 4, 6]])),
case(vec![array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]]])], Some(0), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
case(vec![array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]])], Some(0), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]], [[1, 2], [3, 4]], [[5, 6], [7, 8]]])),
case(vec![array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]])], Some(1), array!([[[1, 2], [3, 4], [1, 2], [3, 4]], [[1, 2], [3, 4], [1, 2], [3, 4]]])),
case(vec![array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]])], Some(2), array!([[[1, 2, 1, 2], [3, 4, 3, 4]], [[1, 2, 1, 2], [3, 4, 3, 4]]])),
#[should_panic(expected = "incompatible shapes for concatenate")]
case(vec![array!([[1, 2], [3, 4]]), array!([5, 6])], Some(0), array!([[1, 2], [3, 4], [5, 6]])),
#[should_panic(expected = "incompatible shapes for concatenate")]
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6]])], Some(1), array!([[1, 2], [3, 4], [5, 6]])),
)] fn test_concatenate(arrs: Vec<Array<i32>>, axis: Option<usize>, expected: Array<i32>) {
    assert_eq!(expected, Array::concatenate(arrs, axis))
}

#[rstest(
arrs, axis, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], None, array!([[1, 2, 3], [4, 5, 6]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6], [7, 8]])], None, array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])),
case(vec![array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]])], Some(0), array!([[[[1, 2], [3, 4]]], [[[5, 6], [7, 8]]]])),
case(vec![array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]]), array!([[[9, 10], [11, 12]]])], Some(0), array!([[[[1, 2], [3, 4]]], [[[5, 6], [7, 8]]], [[[9, 10], [11, 12]]]])),
case(vec![array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]])], Some(1), array!([[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]])),
case(vec![array!([[[1, 2], [3, 4]]]), array!([[[5, 6], [7, 8]]])], Some(2), array!([[[[1, 2], [5, 6]], [[3, 4], [7, 8]]]])),
#[should_panic(expected = "all input arrays must have the same shape")]
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6]])], None, array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])),
)] fn test_stack(arrs: Vec<Array<i32>>, axis: Option<usize>, expected: Array<i32>) {
    assert_eq!(expected, Array::stack(arrs, axis))
}

#[rstest(
arrs, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], array!([[1, 2, 3], [4, 5, 6]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6], [7, 8], [9, 10]])], array!([[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]])),
case(vec![array!([[1]]), array!([[2]]), array!([[3]])], array!([[1], [2], [3]])),
#[should_panic(expected = "incompatible shapes for concatenate")]
case(vec![array!([[1, 2, 3], [1, 2, 3]]), Array::empty()], Array::empty()),
#[should_panic(expected = "incompatible shapes for concatenate")]
case(vec![array!([[1, 2, 3], [1, 2, 3]]), array!([[5, 6]])], Array::empty()),
)] fn test_vstack(arrs: Vec<Array<i32>>, expected: Array<i32>) {
    assert_eq!(expected, Array::vstack(arrs))
}

#[rstest(
arrs, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], array!([1, 2, 3, 4, 5, 6])),
case(vec![array!([1, 2, 3]), array!([4, 5, 6]), array!([7, 8])], array!([1, 2, 3, 4, 5, 6, 7, 8])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6], [7, 8]])], array!([[1, 2, 5, 6], [3, 4, 7, 8]])),
case(vec![array!([[1]]), array!([[2]]), array!([[3]])], array!([[1, 2, 3]])),
#[should_panic(expected = "incompatible shapes for concatenate")]
case(vec![array!([[1, 2], [1, 2]]), array!([5, 6])], Array::empty()),
#[should_panic(expected = "incompatible shapes for concatenate")]
case(vec![array!([[1, 2], [1, 2]]), array!([[5], [6]])], Array::empty()),
)] fn test_hstack(arrs: Vec<Array<i32>>, expected: Array<i32>) {
    assert_eq!(expected, Array::hstack(arrs))
}

#[rstest(
arrs, expected,
case(vec![array!([1, 2, 3]), array!([4, 5, 6])], array!([[[1, 4], [2, 5], [3, 6]]])),
case(vec![array!([[1], [2], [3]]), array!([[4], [5], [6]])], array!([[[1, 4]], [[2, 5]], [[3, 6]]])),
case(vec![array!([[1, 2], [3, 4]]), array!([[5, 6], [7, 8]])], array!([[[1, 5], [2, 6]], [[3, 7], [4, 8]]])),
#[should_panic(expected = "incompatible shapes for concatenate")]
case(vec![array!([1, 2, 3]), array!([4, 5, 6]), array!([7, 8])], Array::empty()),
)] fn test_dstack(arrs: Vec<Array<i32>>, expected: Array<i32>) {
    assert_eq!(expected, Array::dstack(arrs))
}

use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, expected,
case(Array::single(2), Array::single(3), Array::single(6)),
case(Array::single(2), array_flat!(i32, 1, 2, 3), array_flat!(i32, 2, 4, 6)),
case(array_flat!(i32, 1, 2, 3), Array::single(2), array_flat!(i32, 2, 4, 6)),
case(Array::single(2), array!(i32, [[1, 2], [3, 4]]), array!(i32, [[2, 4], [6, 8]])),
case(array!(i32, [[1, 2], [3, 4]]), Array::single(2), array!(i32, [[2, 4], [6, 8]])),
case(array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 1, 2), array_flat!(i32, 7, 10)),
case(array_flat!(i32, 1, 2), array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 5, 11)),
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [[5, 6], [7, 8]]), array!(i32, [[19, 22], [43, 50]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, [[1, 2], [3, 4]]), array!(i32, [[[7, 10], [15, 22]], [[23, 34], [31, 46]]])),
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, [[[7, 10], [19, 22]], [[15, 22], [43, 50]]])),
case(array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]), array!(i32, [[[[7, 10], [19, 22]], [[15, 22], [43, 50]]], [[[23, 34], [67, 78]], [[31, 46], [91, 106]]]])),
case(array_flat!(i32, 1, 2, 3), array!(i32, [[1, 2, 3], [4, 5, 6]]), Err(ArrayError::ParameterError { param: "`len`", message: "of inputs for sum_product must be equal" })),
case(array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 1, 2, 3), Err(ArrayError::ParameterError { param: "`len`", message: "of inputs for sum_product must be equal" })),
case(array!(i32, [[1, 2], [3, 4]]), array_flat!(i32, 1, 2, 3, 4), Err(ArrayError::ParameterError { param: "`len`", message: "of inputs for sum_product must be equal" })),
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [[5, 6, 3], [7, 8, 3]]), Err(ArrayError::ShapesMustMatch { shape_1: vec![2, 2], shape_2: vec![2, 3] })),
case(array_arange!(i32, 1, 24).reshape(&[2, 3, 4]), array_arange!(i32, 1, 24).reshape(&[2, 3, 4]), Err(ArrayError::ParameterError { param: "`shapes`", message: "are not aligned" })),
)] fn test_linalg_dot(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.dot(&other.unwrap()))
}

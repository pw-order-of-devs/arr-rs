use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, ord, axis, keepdims, expected,
case(array_arange!(f64, -4., 4.), None, None, None, array_single!(f64, 7.745966692414834)),
case(array_arange!(f64, -4., 4.).reshape(&[3, 3]), None, None, None, array_single!(f64, 7.745966692414834)),
case(array_arange!(f64, -4., 4.).reshape(&[3, 3]), Some(NormOrd::Fro), None, None, array_single!(f64, 7.745966692414834)),
case(array_arange!(f64, -4., 4.), Some(NormOrd::Inf), None, None, array_single!(f64, 4.)),
case(array_arange!(f64, -4., 4.), Some(NormOrd::NegInf), None, None, array_single!(f64, 0.)),
case(array_arange!(f64, -4., 4.), Some(NormOrd::Int(1)), None, None, array_single!(f64, 20.)),
case(array_arange!(f64, -4., 4.), Some(NormOrd::Int(-1)), None, None, array_single!(f64, 0.)),
case(array_arange!(f64, -4., 4.), Some(NormOrd::Int(2)), None, None, array_single!(f64, 7.745966692414834)),
case(array_arange!(f64, -4., 4.), Some(NormOrd::Int(-2)), None, None, array_single!(f64, 0.)),
case(array_arange!(f64, -4., 4.), Some(NormOrd::Int(3)), None, None, array_single!(f64, 5.848035476425731)),
case(array_arange!(f64, -4., 4.), Some(NormOrd::Int(-3)), None, None, array_single!(f64, 0.)),
case(array!(f64, [[1., 2., 3.], [-1., 1., 4.]]), None, Some(vec![0]), None, array_flat!(f64, std::f64::consts::SQRT_2, 2.23606797749979, 5.)),
case(array!(f64, [[1., 2., 3.], [-1., 1., 4.]]), None, Some(vec![1]), None, array_flat!(f64, 3.7416573867739413, 4.242640687119285)),
case(array!(f64, [[1., 2., 3.], [-1., 1., 4.]]), Some(NormOrd::Int(1)), Some(vec![1]), None, array_flat!(f64, 6., 6.)),
case(array_arange!(f64, -4., 4.).reshape(&[3, 3]), Some(NormOrd::Inf), None, None, array_single!(f64, 9.)),
case(array_arange!(f64, -4., 4.).reshape(&[3, 3]), Some(NormOrd::NegInf), None, None, array_single!(f64, 2.)),
case(array_arange!(f64, -4., 4.).reshape(&[3, 3]), Some(NormOrd::Int(1)), None, None, array_single!(f64, 7.)),
case(array_arange!(f64, -4., 4.).reshape(&[3, 3]), Some(NormOrd::Int(-1)), None, None, array_single!(f64, 6.)),
// case(array_arange!(f64, -4., 4.).reshape(&[3, 3]), Some(NormOrd::Int(2)), None, None, array_single!(f64, 7.745966692414834)),
// case(array_arange!(f64, -4., 4.).reshape(&[3, 3]), Some(NormOrd::Int(-2)), None, None, array_single!(f64, 0.)),
// case(array_arange!(f64, 1., 8.).reshape(&[2, 2, 2]), None, Some(vec![1, 2]), None, array_flat!(f64, 3.74165739, 11.22497216)),
case(array_arange!(f64, -4., 4.), Some(NormOrd::Fro), None, None, Err(ArrayError::ParameterError { param: "`ord`", message: "invalid norm order for vectors." })),
case(array_arange!(f64, -4., 4.), None, Some(vec![2, 2]), None, Err(ArrayError::ParameterError { param: "`axis`", message: "duplicate axes given." })),
case(array_arange!(f64, -4., 4.), None, Some(vec![1, 2, 3]), None, Err(ArrayError::ParameterError { param: "`axis`", message: "improper number of dimensions to norm." })),
)] fn test_linalg_norm(
    array: Result<Array<f64>, ArrayError>,
    ord: Option<NormOrd>,
    axis: Option<Vec<isize>>,
    keepdims: Option<bool>,
    expected: Result<Array<f64>, ArrayError>,
) {
    assert_eq!(expected, array.norm(ord, axis, keepdims))
}

#[rstest(
array, expected,
case(array!(i32, [[1, 2], [3, 4]]), array_single!(i32, -2)),
case(array!(i32, [[3, 8], [4, 6]]), array_single!(i32, -14)),
case(array!(i32, [[6, 2], [1, 4]]), array_single!(i32, 22)),
case(array!(i32, [[1, 2, 3], [3, 2, 1], [2, 1, 3]]), array_single!(i32, -12)),
case(array!(i32, [[5, 6, 4], [6, 3, 5], [8, 2, 8]]), array_single!(i32, -26)),
case(array!(i32, [[1, 2, 6, 6], [4, 7, 3, 2], [0, 0, 0, 0], [1, 2, 2, 9]]), array_single!(i32, 0)),
case(array!(i32, [[4, 3, 2, 2], [0, 1, -3, 3], [0, -1, 3, 3], [0, 3, 1, 1]]), array_single!(i32, -240)),
case(array!(i32, [[[1, 2], [3, 4]], [[4, 3], [2, 1]]]), array_flat!(i32, -2, -2)),
case(array!(i32, [[[1, 2, 3], [3, 2, 1], [2, 1, 3]], [[5, 6, 4], [6, 3, 5], [8, 2, 8]]]), array_flat!(i32, -12, -26)),
)] fn test_linalg_det(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.det())
}

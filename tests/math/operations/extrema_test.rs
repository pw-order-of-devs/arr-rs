use arr_rs::prelude::*;
use rstest::rstest;

#[rstest(
array, other, expected,
case(array![f64, 1., 4., f64::NAN, 4.], array![f64, 2., 2., 2., 10.], array![f64, 2., 4., f64::NAN, 10.]),
case(array![f64, [1., 4.], [f64::NAN, 4.]], array![f64, [2., 2.], [2., 10.]], array![f64, [2., 4.], [f64::NAN, 10.]]),
case(array![f64, 1., 2., 3.], array![f64, 4., 5., 6., 7.], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_maximum(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.maximum(&other.unwrap()) }))
}

#[rstest(
array, axis, expected,
case(array![f64, 1., 2., 3., 4.], None, array![f64, 4.]),
case(array!(f64, [[1., 2.], [3., 4.]]), None, array![f64, 4.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(0), array![f64, 3., 4.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(1), array![f64, 2., 4.]),
case(array!(f64, [[1., 2.], [3., f64::NAN]]), None, array![f64, f64::NAN]),
)] fn test_max(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.max(axis) }))
}

#[rstest(
array, axis, expected,
case(array![f64, 1., 2., 3., 4.], None, array![f64, 4.]),
case(array!(f64, [[1., 2.], [3., 4.]]), None, array![f64, 4.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(0), array![f64, 3., 4.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(1), array![f64, 2., 4.]),
case(array!(f64, [[1., 2.], [3., f64::NAN]]), None, array![f64, f64::NAN]),
)] fn test_amax(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.amax(axis) }))
}

#[rstest(
array, other, expected,
case(array![f64, 1., 4., f64::NAN, 4.], array![f64, 2., 2., 2., 10.], array![f64, 2., 4., 2., 10.]),
case(array![f64, [1., 4.], [f64::NAN, 4.]], array![f64, [2., 2.], [2., 10.]], array![f64, [2., 4.], [2., 10.]]),
case(array![f64, 1., 2., 3.], array![f64, 4., 5., 6., 7.], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_fmax(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.fmax(&other.unwrap()) }))
}

#[rstest(
array, axis, expected,
case(array![f64, 1., 2., 3., 4.], None, array![f64, 4.]),
case(array!(f64, [[1., 2.], [3., 4.]]), None, array![f64, 4.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(0), array![f64, 3., 4.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(1), array![f64, 2., 4.]),
case(array!(f64, [[1., 2.], [f64::NAN, f64::NAN]]), Some(1), array![f64, 2., f64::NAN]),
)] fn test_nanmax(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.nanmax(axis) }))
}

#[rstest(
array, other, expected,
case(array![f64, 1., 4., f64::NAN, 4.], array![f64, 2., 2., 2., 10.], array![f64, 1., 2., f64::NAN, 4.]),
case(array![f64, [1., 4.], [f64::NAN, 4.]], array![f64, [2., 2.], [2., 10.]], array![f64, [1., 2.], [f64::NAN, 4.]]),
case(array![f64, 1., 2., 3.], array![f64, 4., 5., 6., 7.], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_minimum(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.minimum(&other.unwrap()) }))
}

#[rstest(
array, axis, expected,
case(array![f64, 1., 2., 3., 4.], None, array![f64, 1.]),
case(array!(f64, [[1., 2.], [3., 4.]]), None, array![f64, 1.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(0), array![f64, 1., 2.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(1), array![f64, 1., 3.]),
case(array!(f64, [[1., 2.], [3., f64::NAN]]), None, array![f64, f64::NAN]),
)] fn test_min(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.min(axis) }))
}

#[rstest(
array, axis, expected,
case(array![f64, 1., 2., 3., 4.], None, array![f64, 1.]),
case(array!(f64, [[1., 2.], [3., 4.]]), None, array![f64, 1.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(0), array![f64, 1., 2.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(1), array![f64, 1., 3.]),
case(array!(f64, [[1., 2.], [3., f64::NAN]]), None, array![f64, f64::NAN]),
)] fn test_amin(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.amin(axis) }))
}

#[rstest(
array, other, expected,
case(array![f64, 1., 4., f64::NAN, 4.], array![f64, 2., 2., 2., 10.], array![f64, 1., 2., 2., 4.]),
case(array![f64, [1., 4.], [f64::NAN, 4.]], array![f64, [2., 2.], [2., 10.]], array![f64, [1., 2.], [2., 4.]]),
case(array![f64, 1., 2., 3.], array![f64, 4., 5., 6., 7.], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_fmin(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.fmin(&other.unwrap()) }))
}

#[rstest(
array, axis, expected,
case(array![f64, 1., 2., 3., 4.], None, array![f64, 1.]),
case(array!(f64, [[1., 2.], [3., 4.]]), None, array![f64, 1.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(0), array![f64, 1., 2.]),
case(array!(f64, [[1., 2.], [3., 4.]]), Some(1), array![f64, 1., 3.]),
case(array!(f64, [[1., 2.], [f64::NAN, f64::NAN]]), Some(1), array![f64, 1., f64::NAN]),
)] fn test_nanmin(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(format!("{:#}", PrintableResult { result: expected }), format!("{:#}", PrintableResult { result: array.nanmin(axis) }))
}

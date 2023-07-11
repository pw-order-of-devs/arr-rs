use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
shape, expected,
case(vec![4], 4),
case(vec![4, 4], 16),
case(vec![4, 4, 4], 64),
)] fn test_rand(shape: Vec<usize>, expected: usize) {
    assert_eq!(expected, Array::<f64>::rand(shape).len().unwrap())
}

#[rstest(
n, m, k, expected,
case(2, Some(3), Some(0), array!([[1., 0., 0.], [0., 1., 0.]])),
case(3, Some(2), Some(1), array!([[0., 1.], [0., 0.], [0., 0.]])),
case(4, Some(3), Some(0), array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [0., 0., 0.]])),
case(4, Some(3), Some(1), array!([[0., 1., 0.], [0., 0., 1.], [0., 0., 0.], [0., 0., 0.]])),
)] fn test_eye(n: usize, m: Option<usize>, k: Option<usize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::eye(n, m, k))
}

#[rstest(
n, expected,
case(2, array!([[1., 0.], [0., 1.]])),
case(3, array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])),
case(4, array!([[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]])),
)] fn test_identity(n: usize, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::identity(n))
}

#[rstest(
shape, expected,
case(vec![4], array!([0, 0, 0, 0])),
case(vec![2, 2], array!([[0, 0], [0, 0]])),
)] fn test_zeros(shape: Vec<usize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::zeros(shape))
}

#[rstest(
other, expected,
case(array!(1, 2, 3, 4), array!([0., 0., 0., 0.])),
case(array!([[1, 2], [1, 2]]), array!([[0., 0.], [0., 0.]])),
)] fn test_zeros_like(other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::zeros_like(&other.unwrap()))
}

#[rstest(
shape, expected,
case(vec![4], array!([1., 1., 1., 1.])),
case(vec![2, 2], array!([[1., 1.], [1., 1.]])),
)] fn test_ones(shape: Vec<usize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::ones(shape))
}

#[rstest(
other, expected,
case(array!(1, 2, 3, 4), array!([1., 1., 1., 1.])),
case(array!([[1, 2], [1, 2]]), array!([[1., 1.], [1., 1.]])),
)] fn test_ones_like(other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::ones_like(&other.unwrap()))
}

#[rstest(
shape, fill_value, expected,
case(vec![4], 2., array!([2., 2., 2., 2.])),
case(vec![2, 2], 2., array!([[2., 2.], [2., 2.]])),
)] fn test_full(shape: Vec<usize>, fill_value: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::full(shape, fill_value))
}

#[rstest(
other, fill_value, expected,
case(array!(1, 2, 3, 4), 2., array!([2., 2., 2., 2.])),
case(array!([[1, 2], [1, 2]]), 2., array!([[2., 2.], [2., 2.]])),
)] fn test_full_like(other: Result<Array<f64>, ArrayError>, fill_value: f64, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::<f64>::full_like(&other.unwrap(), fill_value))
}

#[rstest(
start, stop, step, expected,
case(0, 4, None, array!([0, 1, 2, 3, 4])),
case(0, 4, Some(1), array!([0, 1, 2, 3, 4])),
case(0, 7, Some(2), array!([0, 2, 4, 6])),
)] fn test_arange(start: i32, stop: i32, step: Option<i32>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, Array::arange(start, stop, step))
}

#[rstest(
start, stop, num, endpoint, expected,
case(0., 5., Some(5), None, array!([0., 1.25, 2.5, 3.75, 5.])),
case(0., 5., Some(5), Some(true), array!([0., 1.25, 2.5, 3.75, 5.])),
case(0., 5., Some(5), Some(false), array!([0., 1., 2., 3., 4.])),
case(0., 10., Some(6), Some(true), array!([0., 2., 4., 6., 8., 10.])),
case(-1., 1., Some(5), None, array!([-1., -0.5, 0.0, 0.5, 1.])),
)] fn test_linspace(start: f64, stop: f64, num: Option<usize>, endpoint: Option<bool>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::linspace(start, stop, num, endpoint))
}

#[rstest(
start, stop, num, endpoint, expected,
case(&array!([0., 2.]), &array!([2., 4.]), Some(5), None, array!([[0., 2.], [0.5, 2.5], [1., 3.], [1.5, 3.5], [2., 4.]])),
case(&array!([0.]), &array!([2., 4.]), Some(5), None, array!([[0., 0.], [0.5, 1.], [1., 2.], [1.5, 3.], [2., 4.]])),
case(&array!([0.]), &array!([1., 1., 1.]), Some(5), Some(true), array!([[0., 0., 0.], [0.25, 0.25, 0.25], [0.5, 0.5, 0.5], [0.75, 0.75, 0.75], [1., 1., 1.]])),
case(&array!([0., 0., 0.]), &array!([1.]), Some(5), Some(true), array!([[0., 0., 0.], [0.25, 0.25, 0.25], [0.5, 0.5, 0.5], [0.75, 0.75, 0.75], [1., 1., 1.]])),
case(&array!([0., 0., 0.]), &array!([1., 1., 1.]), Some(5), Some(true), array!([[0., 0., 0.], [0.25, 0.25, 0.25], [0.5, 0.5, 0.5], [0.75, 0.75, 0.75], [1., 1., 1.]])),
case(&array!([-1., -1., -1.]), &array!([1., 1., 1.]), Some(5), None, array!([[-1., -1., -1.], [-0.5, -0.5, -0.5], [0., 0., 0.], [0.5, 0.5, 0.5], [1., 1., 1.]])),
case(&array!([[-1., -1.], [-1., -1.]]), &array!([[1., 1.], [1., 1.]]), Some(5), None, array!([[[-1., -1.], [-1., -1.]], [[-0.5, -0.5], [-0.5, -0.5]], [[0., 0.], [0., 0.]], [[0.5, 0.5], [0.5, 0.5]], [[1., 1.], [1., 1.]]])),
)] fn test_linspace_a(start: &Result<Array<f64>, ArrayError>, stop: &Result<Array<f64>, ArrayError>, num: Option<usize>, endpoint: Option<bool>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, Array::linspace_a(start.as_ref().unwrap(), stop.as_ref().unwrap(), num, endpoint))
}

#[rstest(
start, stop, num, endpoint, expected,
case(0., 5., Some(5), None, array!([1., 17.78279410038923, 316.22776601683796, 5623.413251903491, 100000.])),
case(0., 5., Some(5), Some(true), array!([1., 17.78279410038923, 316.22776601683796, 5623.413251903491, 100000.])),
case(0., 5., Some(5), Some(false), array!([1., 10.000000000000002, 100.00000000000004, 1000.0000000000006, 10000.000000000007])),
case(0., 10., Some(6), Some(true), array!([1., 100.00000000000003, 10000.000000000005, 1000000.0000000008, 100000000.00000012, 10000000000.])),
case(-1., 1., Some(5), None, array!([0.1, 0.316227766016838, 1.0000000000000002, 3.16227766016838, 10.])),
)] fn test_logspace(start: f64, stop: f64, num: Option<usize>, endpoint: Option<bool>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(&expected, &Array::logspace(start, stop, num, endpoint, None))
}

#[rstest(
start, stop, num, endpoint, expected,
case(&array!([0., 2.]), &array!([2., 4.]), Some(5), None, array!([[1., 100.], [3.1622776601683795, 316.22776601683796], [10.000000000000002, 1000.0000000000002], [31.6227766016838, 3162.27766016838], [100., 10000.]])),
case(&array!([0.]), &array!([2., 4.]), Some(5), None, array!([[1., 1.], [3.1622776601683795, 10.], [10.000000000000002, 100.], [31.6227766016838, 1000.], [100., 10000.]])),
case(&array!([0.]), &array!([1., 1., 1.]), Some(5), Some(true), array!([[1., 1., 1.], [1.7782794100389228, 1.7782794100389228, 1.7782794100389228], [3.162277660168379, 3.162277660168379, 3.162277660168379], [5.62341325190349, 5.62341325190349, 5.62341325190349], [10., 10., 10.]])),
case(&array!([0., 0., 0.]), &array!([1.]), Some(5), Some(true), array!([[1., 1., 1.], [1.7782794100389228, 1.7782794100389228, 1.7782794100389228], [3.162277660168379, 3.162277660168379, 3.162277660168379], [5.62341325190349, 5.62341325190349, 5.62341325190349], [10., 10., 10.]])),
case(&array!([0., 0., 0.]), &array!([1., 1., 1.]), Some(5), Some(true), array!([[1., 1., 1.], [1.7782794100389228, 1.7782794100389228, 1.7782794100389228], [3.162277660168379, 3.162277660168379, 3.162277660168379], [5.62341325190349, 5.62341325190349, 5.62341325190349], [10., 10., 10.]])),
case(&array!([-1., -1., -1.]), &array!([1., 1., 1.]), Some(5), None, array!([[0.1, 0.1, 0.1], [0.316227766016838, 0.316227766016838, 0.316227766016838], [1.0000000000000002, 1.0000000000000002, 1.0000000000000002], [ 3.16227766016838, 3.16227766016838, 3.16227766016838], [10., 10., 10.]])),
)] fn test_logspace_a(start: &Result<Array<f64>, ArrayError>, stop: &Result<Array<f64>, ArrayError>, num: Option<usize>, endpoint: Option<bool>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(&expected, &Array::logspace_a(start.as_ref().unwrap(), stop.as_ref().unwrap(), num, endpoint, None))
}

#[rstest(
start, stop, num, endpoint, expected,
case(1., 5., Some(5), None, array!([1., 1.4953487812212205, 2.2360679774997894, 3.34370152488211, 5.])),
case(1., 5., Some(5), Some(true), array!([1., 1.4953487812212205, 2.2360679774997894, 3.34370152488211, 5.])),
case(1., 5., Some(5), Some(false), array!([1., 1.379729661461215, 1.9036539387158786, 2.6265278044037674, 3.6238983183884783])),
case(1., 10., Some(6), Some(true), array!([1., 1.5848931924611136, 2.5118864315095806, 3.981071705534973, 6.309573444801934, 10.])),
case(0., 5., Some(5), None, Err(ArrayError::ParameterError { param: "start", message: "geometric sequence cannot include zero" })),
)] fn test_geomspace(start: f64, stop: f64, num: Option<usize>, endpoint: Option<bool>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(&expected, &Array::geomspace(start, stop, num, endpoint))
}

#[rstest(
start, stop, num, endpoint, expected,
case(&array!([1., 2.]), &array!([5., 6.]), Some(5), None, array!([[1., 2.], [1.4953487812212205, 2.6321480259049848], [2.2360679774997894, 3.464101615137754], [3.34370152488211, 4.559014113909554], [5., 6.]])),
case(&array!([1., 2.]), &array!([5., 6.]), Some(5), Some(true), array!([[1., 2.], [1.4953487812212205, 2.6321480259049848], [2.2360679774997894, 3.464101615137754], [3.34370152488211, 4.559014113909554], [5., 6.]])),
case(&array!([1., 2.]), &array!([5., 6.]), Some(5), Some(false), array!([[1., 2.], [1.379729661461215, 2.491461879231035], [1.9036539387158786, 3.1036911478307196], [2.6265278044037674 , 3.8663640898635263], [3.6238983183884783, 4.816449370561386]])),
case(&array!([0., 2.]), &array!([0., 6.]), Some(5), Some(false), Err(ArrayError::ParameterError { param: "start", message: "geometric sequence cannot include zero" })),
)] fn test_geomspace_a(start: &Result<Array<f64>, ArrayError>, stop: &Result<Array<f64>, ArrayError>, num: Option<usize>, endpoint: Option<bool>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(&expected, &Array::geomspace_a(start.as_ref().unwrap(), stop.as_ref().unwrap(), num, endpoint))
}

#[rstest(
n, m, k, expected,
case(2, Some(2), None, array!([[1, 0], [1, 1]])),
case(3, Some(3), None, array!([[1, 0, 0], [1, 1, 0], [1, 1, 1]])),
case(3, Some(3), Some(0), array!([[1, 0, 0], [1, 1, 0], [1, 1, 1]])),
case(3, Some(3), Some(1), array!([[1, 1, 0], [1, 1, 1], [1, 1, 1]])),
case(3, Some(3), Some(-1), array!([[0, 0, 0], [1, 0, 0], [1, 1, 0]])),
case(2, Some(4), Some(0), array!([[1, 0, 0, 0], [1, 1, 0, 0]])),
case(2, Some(4), Some(1), array!([[1, 1, 0, 0], [1, 1, 1, 0]])),
case(2, Some(4), Some(-1), array!([[0, 0, 0, 0], [1, 0, 0, 0]])),
)] fn test_tri(n: usize, m: Option<usize>, k: Option<isize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, Array::tri(n, m, k))
}

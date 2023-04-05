use rstest::rstest;
use arr_rs::prelude::*;

use crate::arr_rs_tests::common::test_runner;

#[rstest(
elements, shape, expected,
case(vec![1, 2, 3, 4], vec![4], array!([1, 2, 3, 4])),
case(vec![1, 2, 3, 4], vec![2, 2], array!([[1, 2], [3, 4]])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2], array!([[[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
#[should_panic(expected = "Shape must match values length")]
case(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![10], array!([1])),
)] fn test_new(elements: Vec<i32>, shape: Vec<usize>, expected: Array<i32>) {
    assert_eq!(expected, Array::new(elements, shape))
}

#[rstest(
elements, expected,
case(vec![1, 2, 3, 4], array!([1, 2, 3, 4])),
case(vec![1, 2, 3, 4, 1, 2, 3, 4], array!([1, 2, 3, 4, 1, 2, 3, 4])),
)] fn test_flat(elements: Vec<i32>, expected: Array<i32>) {
    assert_eq!(expected, Array::flat(elements))
}

#[rstest(
shape, expected,
case(vec![4], 4),
case(vec![4, 4], 16),
case(vec![4, 4, 4], 64),
)] fn test_rand(shape: Vec<usize>, expected: usize) {
    assert_eq!(expected, Array::<f64>::rand(shape).len())
}

#[rstest(
array, expected,
case(Array::empty(), Array::new(vec![], vec![0])),
)] fn test_empty(array: Array<f64>, expected: Array<f64>) {
    assert_eq!(expected, array)
}

#[rstest(
n, m, k, expected,
case(2, Some(3), Some(0), array!([[1., 0., 0.], [0., 1., 0.]])),
case(3, Some(2), Some(1), array!([[0., 1.], [0., 0.], [0., 0.]])),
case(4, Some(3), Some(0), array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [0., 0., 0.]])),
case(4, Some(3), Some(1), array!([[0., 1., 0.], [0., 0., 1.], [0., 0., 0.], [0., 0., 0.]])),
)] fn test_eye(n: usize, m: Option<usize>, k: Option<usize>, expected: Array<f64>) {
     assert_eq!(expected, Array::<f64>::eye(n, m, k))
}

#[rstest(
n, expected,
case(2, array!([[1., 0.], [0., 1.]])),
case(3, array!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])),
case(4, array!([[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]])),
)] fn test_identity(n: usize, expected: Array<f64>) {
     assert_eq!(expected, Array::<f64>::identity(n))
}

#[rstest(
shape, expected,
case(vec![4], array!([0, 0, 0, 0])),
case(vec![2, 2], array!([[0, 0], [0, 0]])),
)] fn test_zeros(shape: Vec<usize>, expected: Array<f64>) {
    assert_eq!(expected, Array::<f64>::zeros(shape))
}

#[rstest(
other, expected,
case(array!(1, 2, 3, 4), array!([0., 0., 0., 0.])),
case(array!([[1, 2], [1, 2]]), array!([[0., 0.], [0., 0.]])),
)] fn test_zeros_like(other: Array<f64>, expected: Array<f64>) {
    assert_eq!(expected, Array::<f64>::zeros_like(&other))
}

#[rstest(
shape, expected,
case(vec![4], array!([1., 1., 1., 1.])),
case(vec![2, 2], array!([[1., 1.], [1., 1.]])),
)] fn test_ones(shape: Vec<usize>, expected: Array<f64>) {
    assert_eq!(expected, Array::<f64>::ones(shape))
}

#[rstest(
other, expected,
case(array!(1, 2, 3, 4), array!([1., 1., 1., 1.])),
case(array!([[1, 2], [1, 2]]), array!([[1., 1.], [1., 1.]])),
)] fn test_ones_like(other: Array<f64>, expected: Array<f64>) {
    assert_eq!(expected, Array::<f64>::ones_like(&other))
}

#[rstest(
shape, fill_value, expected,
case(vec![4], 2., array!([2., 2., 2., 2.])),
case(vec![2, 2], 2., array!([[2., 2.], [2., 2.]])),
)] fn test_full(shape: Vec<usize>, fill_value: f64, expected: Array<f64>) {
    assert_eq!(expected, Array::<f64>::full(shape, fill_value))
}

#[rstest(
other, fill_value, expected,
case(array!(1, 2, 3, 4), 2., array!([2., 2., 2., 2.])),
case(array!([[1, 2], [1, 2]]), 2., array!([[2., 2.], [2., 2.]])),
)] fn test_full_like(other: Array<f64>, fill_value: f64, expected: Array<f64>) {
    assert_eq!(expected, Array::<f64>::full_like(&other, fill_value))
}

#[rstest(
start, stop, step, expected,
case(0, 5, None, array!([0, 1, 2, 3, 4])),
case(0, 5, Some(1), array!([0, 1, 2, 3, 4])),
case(0, 8, Some(2), array!([0, 2, 4, 6])),
)] fn test_arange(start: i32, stop: i32, step: Option<i32>, expected: Array<i32>) {
    assert_eq!(expected, Array::arange(start, stop, step))
}

#[rstest(
start, stop, num, endpoint, expected,
case(0., 5., Some(5), None, array!([0., 1.25, 2.5, 3.75, 5.])),
case(0., 5., Some(5), Some(true), array!([0., 1.25, 2.5, 3.75, 5.])),
case(0., 5., Some(5), Some(false), array!([0., 1., 2., 3., 4.])),
case(0., 10., Some(6), Some(true), array!([0., 2., 4., 6., 8., 10.])),
case(-1., 1., Some(5), None, array!([-1., -0.5, 0.0, 0.5, 1.])),
)] fn test_linspace(start: f64, stop: f64, num: Option<usize>, endpoint: Option<bool>, expected: Array<f64>) {
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
)] fn test_linspace_a(start: &Array<f64>, stop: &Array<f64>, num: Option<usize>, endpoint: Option<bool>, expected: Array<f64>) {
    assert_eq!(expected, Array::linspace_a(start, stop, num, endpoint))
}

#[rstest(
start, stop, num, endpoint, expected,
case(0., 5., Some(5), None, array!([1., 17.7828, 316.2278, 5623.4133, 100000.])),
case(0., 5., Some(5), Some(true), array!([1., 17.7828, 316.2278, 5623.4133, 100000.])),
case(0., 5., Some(5), Some(false), array!([1., 10., 100., 1000., 10000.])),
case(0., 10., Some(6), Some(true), array!([1., 100., 10000., 1000000., 100000000., 10000000000.])),
case(-1., 1., Some(5), None, array!([0.1, 0.3162, 1., 3.1623, 10.])),
)] fn test_logspace(start: f64, stop: f64, num: Option<usize>, endpoint: Option<bool>, expected: Array<f64>) {
    test_runner(&expected, &Array::logspace(start, stop, num, endpoint, None))
}

#[rstest(
start, stop, num, endpoint, expected,
case(&array!([0., 2.]), &array!([2., 4.]), Some(5), None, array!([[1., 100.], [3.1623, 316.2278], [10., 1000.], [31.6228, 3162.2777], [100., 10000.]])),
case(&array!([0.]), &array!([2., 4.]), Some(5), None, array!([[1., 1.], [3.1623, 10.], [10., 100.], [31.6228, 1000.], [100., 10000.]])),
case(&array!([0.]), &array!([1., 1., 1.]), Some(5), Some(true), array!([[1., 1., 1.], [1.7783, 1.7783, 1.7783], [3.1623, 3.1623, 3.1623], [5.6234, 5.6234, 5.6234], [10., 10., 10.]])),
case(&array!([0., 0., 0.]), &array!([1.]), Some(5), Some(true), array!([[1., 1., 1.], [1.7783, 1.7783, 1.7783], [3.1623, 3.1623, 3.1623], [5.6234, 5.6234, 5.6234], [10., 10., 10.]])),
case(&array!([0., 0., 0.]), &array!([1., 1., 1.]), Some(5), Some(true), array!([[1., 1., 1.], [1.7783, 1.7783, 1.7783], [3.1623, 3.1623, 3.1623], [5.6234, 5.6234, 5.6234], [10., 10., 10.]])),
case(&array!([-1., -1., -1.]), &array!([1., 1., 1.]), Some(5), None, array!([[0.1, 0.1, 0.1], [0.3162, 0.3162, 0.3162], [1., 1., 1.], [ 3.1623, 3.1623, 3.1623], [10., 10., 10.]])),
)] fn test_logspace_a(start: &Array<f64>, stop: &Array<f64>, num: Option<usize>, endpoint: Option<bool>, expected: Array<f64>) {
    test_runner(&expected, &Array::logspace_a(start, stop, num, endpoint, None))
}

#[rstest(
start, stop, num, endpoint, expected,
case(1., 5., Some(5), None, array!([1., 1.49534878, 2.23606798, 3.34370152, 5.])),
case(1., 5., Some(5), Some(true), array!([1., 1.49534878, 2.23606798, 3.34370152, 5.])),
case(1., 5., Some(5), Some(false), array!([1., 1.37972966, 1.90365394, 2.6265278, 3.62389832])),
case(1., 10., Some(6), Some(true), array!([1., 1.58489319, 2.51188643, 3.98107171, 6.30957344, 10.])),
#[should_panic(expected = "Geometric sequence cannot include zero")]
case(0., 5., Some(5), None, array!([1., 1., 2., 3., 5.])),
)] fn test_geomspace(start: f64, stop: f64, num: Option<usize>, endpoint: Option<bool>, expected: Array<f64>) {
    test_runner(&expected, &Array::geomspace(start, stop, num, endpoint))
}

#[rstest(
start, stop, num, endpoint, expected,
case(&array!([1., 2.]), &array!([5., 6.]), Some(5), None, array!([[1., 2.], [1.4953, 2.6321], [2.2361, 3.4641], [3.3437, 4.5590], [5., 6.]])),
case(&array!([1., 2.]), &array!([5., 6.]), Some(5), Some(true), array!([[1., 2.], [1.4953, 2.6321], [2.2361, 3.4641], [3.3437, 4.5590], [5., 6.]])),
case(&array!([1., 2.]), &array!([5., 6.]), Some(5), Some(false), array!([[1., 2.], [1.3797, 2.4915], [1.9037, 3.1037], [2.6265 , 3.8664], [3.6239, 4.8164]])),
#[should_panic(expected = "Geometric sequence cannot include zero")]
case(&array!([0., 2.]), &array!([0., 6.]), Some(5), Some(false), array!([[1., 2.], [1., 2.], [1., 3.], [2. , 3.], [3., 4.]])),
)] fn test_geomspace_a(start: &Array<f64>, stop: &Array<f64>, num: Option<usize>, endpoint: Option<bool>, expected: Array<f64>) {
    test_runner(&expected, &Array::geomspace_a(start, stop, num, endpoint))
}

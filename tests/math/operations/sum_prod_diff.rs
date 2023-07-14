use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, axis, expected,
case(array![1, 2, 3, 4], None, array_flat!(24)),
case(array!([[1, 2], [3, 4]]), None, array_flat!(24)),
case(array!([[1, 2], [3, 4]]), Some(0), array_flat!(3, 8)),
case(array!([[1, 2], [3, 4]]), Some(1), array_flat!(2, 12)),
case(array_arange!(0, 23).reshape(vec![2, 3, 4]), None, array_flat!(0)),
case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(0), array!([[0, 13, 28, 45], [64, 85, 108, 133], [160, 189, 220, 253]])),
case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(1), array!([[0, 45, 120, 231], [3840, 4641, 5544, 6555]])),
case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(2), array!([[0, 840, 7920], [32760, 93024, 212520]])),
)] fn test_prod(array: Result<Array<i32>, ArrayError>, axis: Option<isize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.prod(axis))
}

#[rstest(
array, axis, expected,
case(array![1, 2, 3, 4], None, array_flat!(10)),
case(array!([[1, 2], [3, 4]]), None, array_flat!(10)),
case(array!([[1, 2], [3, 4]]), Some(0), array_flat!(4, 6)),
case(array!([[1, 2], [3, 4]]), Some(1), array_flat!(3, 7)),
case(array_arange!(0, 23).reshape(vec![2, 3, 4]), None, array_flat!(276)),
case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(0), array!([[12, 14, 16, 18], [20, 22, 24, 26], [28, 30, 32, 34]])),
case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(1), array!([[12, 15, 18, 21], [48, 51, 54, 57]])),
case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(2), array!([[6, 22, 38], [54, 70, 86]])),
case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(2), array!([[6, 22, 38], [54, 70, 86]])),
)] fn test_sum(array: Result<Array<i32>, ArrayError>, axis: Option<isize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.sum(axis))
}

#[rstest(
array, axis, expected,
case(array![1., 2., 3., 4.], None, array_flat!(24.)),
case(array!([[1., 2.], [3., 4.]]), None, array_flat!(24.)),
case(array!([[1., f64::NAN], [3., f64::NAN]]), None, array_flat!(3.)),
case(array!([[1., 2.], [3., 4.]]), Some(0), array_flat!(3., 8.)),
case(array!([[1., 2.], [3., 4.]]), Some(1), array_flat!(2., 12.)),
case(array!([[1., f64::NAN], [3., f64::NAN]]), Some(1), array_flat!(1., 3.)),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), None, array_flat!(0.)),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(0), array!([[0., 13., 28., 45.], [64., 85., 108., 133.], [160., 189., 220., 253.]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(1), array!([[0., 45., 120., 231.], [3840., 4641., 5544., 6555.]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(2), array!([[0., 840., 7920.], [32760., 93024., 212520.]])),
)] fn test_nanprod(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.nanprod(axis))
}

#[rstest(
array, axis, expected,
case(array![1., 2., 3., 4.], None, array_flat!(10.)),
case(array!([[1., 2.], [3., 4.]]), None, array_flat!(10.)),
case(array!([[1., f64::NAN], [3., f64::NAN]]), None, array_flat!(4.)),
case(array!([[1., 2.], [3., 4.]]), Some(0), array_flat!(4., 6.)),
case(array!([[1., 2.], [3., 4.]]), Some(1), array_flat!(3., 7.)),
case(array!([[1., f64::NAN], [3., f64::NAN]]), Some(1), array_flat!(1., 3.)),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), None, array_flat!(276.)),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(0), array!([[12., 14., 16., 18.], [20., 22., 24., 26.], [28., 30., 32., 34.]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(1), array!([[12., 15., 18., 21.], [48., 51., 54., 57.]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(2), array!([[6., 22., 38.], [54., 70., 86.]])),
)] fn test_nansum(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.nansum(axis))
}

#[rstest(
array, axis, expected,
case(array![1., 2., 3., 4.], None, array_flat!(1., 2., 6., 24.)),
case(array!([[1., 2.], [3., 4.]]), None, array_flat!(1., 2., 6., 24.)),
case(array!([[1., 2.], [3., 4.]]), Some(0), array!([[1., 2.], [3., 8.]])),
case(array!([[1., 2.], [3., 4.]]), Some(1), array!([[1., 2.], [3., 12.]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), None, array_full!(vec![24], 0.)),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(0), array!([[[0., 1., 2., 3.], [4., 5., 6., 7.], [8., 9., 10., 11.]], [[0., 13., 28., 45.], [64., 85., 108., 133.], [160., 189., 220., 253.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(1), array!([[[0., 1., 2., 3.], [0., 5., 12., 21.], [0., 45., 120., 231.]], [[12., 13., 14., 15.], [192., 221., 252., 285.], [3840., 4641., 5544., 6555.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(2), array!([[[0., 0., 0., 0.], [4., 20., 120., 840.], [8., 72., 720., 7920.]], [[12., 156., 2184., 32760.], [16., 272., 4896., 93024.], [20., 420., 9240., 212520.]]])),
)] fn test_cumprod(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.cumprod(axis))
}

#[rstest(
array, axis, expected,
case(array![1., 2., 3., 4.], None, array_flat!(1., 3., 6., 10.)),
case(array!([[1., 2.], [3., 4.]]), None, array_flat!(1., 3., 6., 10.)),
case(array!([[1., 2.], [3., 4.]]), Some(0), array!([[1., 2.], [4., 6.]])),
case(array!([[1., 2.], [3., 4.]]), Some(1), array!([[1., 3.], [3., 7.]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), None, array_flat!(0., 1., 3., 6., 10., 15., 21., 28., 36., 45., 55., 66., 78., 91., 105., 120., 136., 153., 171., 190., 210., 231., 253., 276.)),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(0), array!([[[0., 1., 2., 3.], [4., 5., 6., 7.], [8., 9., 10., 11.]], [[12., 14., 16., 18.], [20., 22., 24., 26.], [28., 30., 32., 34.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(1), array!([[[0., 1., 2., 3.], [4., 6., 8., 10.], [12., 15., 18., 21.]], [[12., 13., 14., 15.], [28., 30., 32., 34.], [48., 51., 54., 57.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(2), array!([[[0., 1., 3., 6.], [4., 9., 15., 22.], [8., 17., 27., 38.]], [[12., 25., 39., 54.], [16., 33., 51., 70.], [20., 41., 63., 86.]]])),
)] fn test_cumsum(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.cumsum(axis))
}

#[rstest(
array, axis, expected,
case(array![1., 2., 3., 4.], None, array_flat!(1., 2., 6., 24.)),
case(array!([[1., 2.], [3., 4.]]), None, array_flat!(1., 2., 6., 24.)),
case(array!([[1., f64::NAN], [3., f64::NAN]]), None, array_flat!(1., 1., 3., 3.)),
case(array!([[1., 2.], [3., 4.]]), Some(0), array!([[1., 2.], [3., 8.]])),
case(array!([[1., 2.], [3., 4.]]), Some(1), array!([[1., 2.], [3., 12.]])),
case(array!([[1., f64::NAN], [3., f64::NAN]]), Some(1), array!([[1., 1.], [3., 3.]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), None, array_full!(vec![24], 0.)),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(0), array!([[[0., 1., 2., 3.], [4., 5., 6., 7.], [8., 9., 10., 11.]], [[0., 13., 28., 45.], [64., 85., 108., 133.], [160., 189., 220., 253.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(1), array!([[[0., 1., 2., 3.], [0., 5., 12., 21.], [0., 45., 120., 231.]], [[12., 13., 14., 15.], [192., 221., 252., 285.], [3840., 4641., 5544., 6555.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(2), array!([[[0., 0., 0., 0.], [4., 20., 120., 840.], [8., 72., 720., 7920.]], [[12., 156., 2184., 32760.], [16., 272., 4896., 93024.], [20., 420., 9240., 212520.]]])),
)] fn test_nancumprod(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.nancumprod(axis))
}

#[rstest(
array, axis, expected,
case(array![1., 2., 3., 4.], None, array_flat!(1., 3., 6., 10.)),
case(array!([[1., 2.], [3., 4.]]), None, array_flat!(1., 3., 6., 10.)),
case(array!([[1., f64::NAN], [3., f64::NAN]]), None, array_flat!(1., 1., 4., 4.)),
case(array!([[1., 2.], [3., 4.]]), Some(0), array!([[1., 2.], [4., 6.]])),
case(array!([[1., 2.], [3., 4.]]), Some(1), array!([[1., 3.], [3., 7.]])),
case(array!([[1., f64::NAN], [3., f64::NAN]]), Some(1), array!([[1., 1.], [3., 3.]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), None, array_flat!(0., 1., 3., 6., 10., 15., 21., 28., 36., 45., 55., 66., 78., 91., 105., 120., 136., 153., 171., 190., 210., 231., 253., 276.)),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(0), array!([[[0., 1., 2., 3.], [4., 5., 6., 7.], [8., 9., 10., 11.]], [[12., 14., 16., 18.], [20., 22., 24., 26.], [28., 30., 32., 34.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(1), array!([[[0., 1., 2., 3.], [4., 6., 8., 10.], [12., 15., 18., 21.]], [[12., 13., 14., 15.], [28., 30., 32., 34.], [48., 51., 54., 57.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), Some(2), array!([[[0., 1., 3., 6.], [4., 9., 15., 22.], [8., 17., 27., 38.]], [[12., 25., 39., 54.], [16., 33., 51., 70.], [20., 41., 63., 86.]]])),
)] fn test_nancumsum(array: Result<Array<f64>, ArrayError>, axis: Option<isize>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.nancumsum(axis))
}

#[rstest(
array, n, axis, prepend, append, expected,
case(array![1., 2., 4., 4., 7.], 0, None, None, None, Array::empty()),
case(array![1., 2., 4., 4., 7.], 1, None, None, None, array_flat!(1., 2., 0., 3.)),
case(array![1., 2., 4., 4., 7.], 2, None, None, None, array_flat!(1., -2., 3.)),
case(array!([[1., 2., 4.], [4., 7., 0.]]), 1, Some(0), Some(array!([[1., 1., 1.]]).unwrap()), None, array!([[0., 1., 3.], [3., 5., -4.]])),
case(array!([[1., 2., 4.], [4., 7., 0.]]), 2, Some(0), None, Some(array!([[1., 1., 1.]]).unwrap()), array!([[-6., -11., 5.]])),
case(array!([[1., 2., 4.], [4., 7., 0.]]), 2, Some(0), None, Some(array!([[1., 1., 1.], [1., 1., 1.]]).unwrap()), array!([[-6., -11., 5.], [3., 6., -1.]])),
case(array!([[1., 2., 4.], [4., 7., 0.]]), 1, Some(1), Some(array!([[1.], [1.]]).unwrap()), None, array!([[0., 1., 2.], [3., 3., -7.]])),
case(array!([[1., 2., 4.], [4., 7., 0.]]), 2, Some(1), None, Some(array!([[1.], [1.]]).unwrap()), array!([[1., -5.], [-10., 8.]])),
case(array!([[1., 2., 4.], [4., 7., 0.]]), 2, Some(1), None, Some(array!([[1., 1., 1.], [1., 1., 1.]]).unwrap()), array!([[1., -5., 3., 0.], [-10., 8., -1., 0.]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), 1, Some(0), Some(array_arange!(0., 11.).reshape(vec![1, 3, 4]).unwrap()), None, array!([[[0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.]], [[12., 12., 12., 12.], [12., 12., 12., 12.], [12., 12., 12., 12.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), 2, Some(0), Some(array_arange!(0., 11.).reshape(vec![1, 3, 4]).unwrap()), None, array!([[[12., 12., 12., 12.], [12., 12., 12., 12.], [12., 12., 12., 12.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), 1, Some(1), Some(array_arange!(0., 7.).reshape(vec![2, 1, 4]).unwrap()), None, array!([[[0., 0., 0., 0.], [4., 4., 4., 4.], [4., 4., 4., 4.]], [[8., 8., 8., 8.], [4., 4., 4., 4.], [4., 4., 4., 4.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), 2, Some(1), Some(array_arange!(0., 7.).reshape(vec![2, 1, 4]).unwrap()), None, array!([[[4., 4., 4., 4.], [0., 0., 0., 0.]], [[-4., -4., -4., -4.], [0., 0., 0., 0.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), 1, Some(2), Some(array_arange!(0., 5.).reshape(vec![2, 3, 1]).unwrap()), None, array!([[[0., 1., 1., 1.], [3., 1., 1., 1.], [6., 1., 1., 1.]], [[9., 1., 1., 1.], [12., 1., 1., 1.], [15., 1., 1., 1.]]])),
case(array_arange!(0., 23.).reshape(vec![2, 3, 4]), 2, Some(2), Some(array_arange!(0., 5.).reshape(vec![2, 3, 1]).unwrap()), None, array!([[[1., 0., 0.], [-2., 0., 0.], [-5., 0., 0.]], [[-8., 0., 0.], [-11., 0., 0.], [-14., 0., 0.]]])),
)] fn test_diff(array: Result<Array<f64>, ArrayError>, n: usize, axis: Option<isize>, prepend: Option<Array<f64>>, append: Option<Array<f64>>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.diff(n, axis, prepend, append))
}

#[rstest(
array, to_end, to_begin, expected,
case(array![1., 2., 4., 4., 7.], None, None, array_flat!(1., 2., 0., 3.)),
case(array!([[1., 2., 4.], [4., 7., 0.]]), None, None, array_flat!(1., 2., 0., 3., -7.)),
case(array![1., 2., 4., 4., 7.], None, Some(Array::single(-99.).unwrap()), array_flat!(-99., 1., 2., 0., 3.)),
case(array![1., 2., 4., 4., 7.], Some(Array::flat(vec![98., 99.]).unwrap()), None, array_flat!(1., 2., 0., 3., 98., 99.)),
case(array![1., 2., 4., 4., 7.], Some(Array::flat(vec![98., 99.]).unwrap()), Some(Array::single(-99.).unwrap()), array_flat!(-99., 1., 2., 0., 3., 98., 99.)),
)] fn test_ediff1d(array: Result<Array<f64>, ArrayError>, to_end: Option<Array<f64>>, to_begin: Option<Array<f64>>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.ediff1d(to_end, to_begin))
}

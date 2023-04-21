use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, k, expected,
case(array![1, 2, 3, 4], None, array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]])),
case(array![1, 2, 3, 4], Some(1), array!([[0, 1, 0, 0, 0], [0, 0, 2, 0, 0], [0, 0, 0, 3, 0], [0, 0, 0, 0, 4], [0, 0, 0, 0, 0]])),
case(array![1, 2, 3, 4], Some(-1), array!([[0, 0, 0, 0, 0], [1, 0, 0, 0, 0], [0, 2, 0, 0, 0], [0, 0, 3, 0, 0], [0, 0, 0, 4, 0]])),
case(array_arange!(0, 15).reshape(vec![4, 4]), None, array!([0, 5, 10, 15])),
case(array_arange!(0, 15).reshape(vec![4, 4]), Some(1), array!([1, 6, 11])),
case(array_arange!(0, 15).reshape(vec![4, 4]), Some(-1), array!([4, 9, 14])),
case(array_arange!(0, 15).reshape(vec![4, 4]), Some(3), array!([3])),
case(array_arange!(0, 15).reshape(vec![4, 4]), Some(-3), array!([12])),
#[should_panic(expected = "`diag` is only defined for 1d and 2d input")]
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), None, array![0, 0]),
)] fn test_diag(array: Array<i32>, k: Option<isize>, expected: Array<i32>) {
    assert_eq!(expected, Array::diag(&array, k))
}

#[rstest(
array, k, expected,
case(array![1, 2, 3, 4], None, array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]])),
case(array![1, 2, 3, 4], Some(1), array!([[0, 1, 0, 0, 0], [0, 0, 2, 0, 0], [0, 0, 0, 3, 0], [0, 0, 0, 0, 4], [0, 0, 0, 0, 0]])),
case(array![1, 2, 3, 4], Some(-1), array!([[0, 0, 0, 0, 0], [1, 0, 0, 0, 0], [0, 2, 0, 0, 0], [0, 0, 3, 0, 0], [0, 0, 0, 4, 0]])),
case(array![[1, 2], [3, 4]], None, array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]])),
case(array![[1, 2], [3, 4]], Some(1), array!([[0, 1, 0, 0, 0], [0, 0, 2, 0, 0], [0, 0, 0, 3, 0], [0, 0, 0, 0, 4], [0, 0, 0, 0, 0]])),
case(array![[1, 2], [3, 4]], Some(-1), array!([[0, 0, 0, 0, 0], [1, 0, 0, 0, 0], [0, 2, 0, 0, 0], [0, 0, 3, 0, 0], [0, 0, 0, 4, 0]])),
)] fn test_diagflat(array: Array<i32>, k: Option<isize>, expected: Array<i32>) {
    assert_eq!(expected, Array::diagflat(&array, k))
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
)] fn test_tri(n: usize, m: Option<usize>, k: Option<isize>, expected: Array<i32>) {
    assert_eq!(expected, Array::tri(n, m, k))
}

#[rstest(
array, k, expected,
case(array_arange!(1, 8).reshape(vec![2, 4]), Some(0), array!([[1, 0, 0, 0], [5, 6, 0, 0]])),
case(array_arange!(1, 8).reshape(vec![2, 4]), Some(1), array!([[1, 2, 0, 0], [5, 6, 7, 0]])),
case(array_arange!(1, 8).reshape(vec![2, 4]), Some(-1), array!([[0, 0, 0, 0], [5, 0, 0, 0]])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), Some(0), array!([[[1, 0], [3, 4]], [[5, 0], [7, 8]]])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), Some(1), array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), Some(-1), array!([[[0, 0], [3, 0]], [[0, 0], [7, 0]]])),
)] fn test_tril(array: Array<i32>, k: Option<isize>, expected: Array<i32>) {
    assert_eq!(expected, array.tril(k))
}

#[rstest(
array, k, expected,
case(array_arange!(1, 8).reshape(vec![2, 4]), Some(0), array!([[1, 2, 3, 4], [0, 6, 7, 8]])),
case(array_arange!(1, 8).reshape(vec![2, 4]), Some(1), array!([[0, 2, 3, 4], [0, 0, 7, 8]])),
case(array_arange!(1, 8).reshape(vec![2, 4]), Some(-1), array!([[1, 2, 3, 4], [5, 6, 7, 8]])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), Some(0), array!([[[1, 2], [0, 4]], [[5, 6], [0, 8]]])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), Some(1), array!([[[0, 2], [0, 0]], [[0, 6], [0, 0]]])),
case(array_arange!(1, 8).reshape(vec![2, 2, 2]), Some(-1), array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]])),
)] fn test_triu(array: Array<i32>, k: Option<isize>, expected: Array<i32>) {
    assert_eq!(expected, array.triu(k))
}

#[rstest(
array, n, increment, expected,
case(array!([1, 2, 3, 4]), Some(3), None, array!([[1, 1, 1], [4, 2, 1], [9, 3, 1], [16, 4, 1]])),
case(array!([1, 2, 3, 4]), None, None, array!([[1, 1, 1, 1], [8, 4, 2, 1], [27, 9, 3, 1], [64, 16, 4, 1]])),
case(array!([1, 2, 3, 4]), None, Some(false), array!([[1, 1, 1, 1], [8, 4, 2, 1], [27, 9, 3, 1], [64, 16, 4, 1]])),
case(array!([1, 2, 3, 4]), None, Some(true), array!([[1, 1, 1, 1], [1, 2, 4, 8], [1, 3, 9, 27], [1, 4, 16, 64]])),
case(array!([1, 2, 3, 4]), Some(5), Some(false), array!([[1, 1, 1, 1, 1], [16, 8, 4, 2, 1], [81, 27, 9, 3, 1], [256, 64, 16, 4, 1]])),
case(array!([1, 2, 3, 4]), Some(5), Some(true), array!([[1, 1, 1, 1, 1], [1, 2, 4, 8, 16], [1, 3, 9, 27, 81], [1, 4, 16, 64, 256]])),
)] fn test_vander(array: Array<i32>, n: Option<usize>, increment: Option<bool>, expected: Array<i32>) {
    assert_eq!(expected, array.vander(n, increment))
}

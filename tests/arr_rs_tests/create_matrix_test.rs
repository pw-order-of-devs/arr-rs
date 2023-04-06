use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, k, expected,
case(array![1, 2, 3, 4], None, array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]])),
case(array![1, 2, 3, 4], Some(1), array!([[0, 1, 0, 0, 0], [0, 0, 2, 0, 0], [0, 0, 0, 3, 0], [0, 0, 0, 0, 4], [0, 0, 0, 0, 0]])),
case(array![1, 2, 3, 4], Some(-1), array!([[0, 0, 0, 0, 0], [1, 0, 0, 0, 0], [0, 2, 0, 0, 0], [0, 0, 3, 0, 0], [0, 0, 0, 4, 0]])),
case(array_arange!(0, 16).reshape(vec![4, 4]), None, array!([0, 5, 10, 15])),
case(array_arange!(0, 16).reshape(vec![4, 4]), Some(1), array!([1, 6, 11])),
case(array_arange!(0, 16).reshape(vec![4, 4]), Some(-1), array!([4, 9, 14])),
case(array_arange!(0, 16).reshape(vec![4, 4]), Some(3), array!([3])),
case(array_arange!(0, 16).reshape(vec![4, 4]), Some(-3), array!([12])),
#[should_panic(expected = "`diag` is only defined for 1d and 2d input")]
case(array_arange!(0, 8).reshape(vec![2, 2, 2]), None, array![0, 0]),
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

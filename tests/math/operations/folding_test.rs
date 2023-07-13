use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array![1, 2, 3, 4], 24),
case(array!([[1, 2], [3, 4]]), 24),
// case(array![1, 2, 3, 4], None, array_flat!(24)),
// case(array!([[1, 2], [3, 4]]), None, array_flat!(24)),
// case(array!([[1, 2], [3, 4]]), Some(0), array_flat!(3, 8)),
// case(array!([[1, 2], [3, 4]]), Some(1), array_flat!(2, 12)),
// case(array_arange!(0, 23).reshape(vec![2, 3, 4]), None, array_flat!(0)),
// case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(0), array!([[0, 13, 28, 45], [64, 85, 108, 133], [160, 189, 220, 253]])),
// case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(1), array!([[0, 45, 120, 231], [3840, 4641, 5544, 6555]])),
// case(array_arange!(0, 23).reshape(vec![2, 3, 4]), Some(2), array!([[0, 840, 7920], [32760, 93024, 212520]])),
)] fn test_prod(array: Result<Array<i32>, ArrayError>, expected: i32) {
    assert_eq!(expected, array.prod().unwrap())
}

#[rstest(
array, expected,
case(array!([1, 2, 3, 4]), array!([1, 3, 6, 10])),
case(array!([[1, 2], [3, 4]]), array!([[1, 3], [6, 10]])),
)] fn test_cumsum(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.cumsum())
}

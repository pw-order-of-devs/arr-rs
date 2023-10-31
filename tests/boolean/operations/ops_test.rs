use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array!(bool, [true, false, true, false]), array!(bool, [false, true, false, true])),
case(array!(bool, [[true, false], [true, false]]), array!(bool, [[false, true], [false, true]])),
)] fn test_not(arr: Result<Array<bool>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), !arr.unwrap());
}

#[rstest(
arr1, arr2, expected,
case(array!(bool, [true, false, true, false]), array!(bool, [true, true, false, false]), array!(bool, [true, false, false, false])),
case(array!(bool, [[true, false], [true, false]]), array!(bool, [[true, true], [false, false]]), array!(bool, [[true, false], [false, false]])),
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([2, 2])\n right: Ok([4])")]
case(array!(bool, [[true, false], [true, false]]), array!(bool, [true, false, true, false]), array!(bool, [[true, false], [true, false]])),
)] fn test_bitand(arr1: Result<Array<bool>, ArrayError>, arr2: Result<Array<bool>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() & arr2.unwrap());
}

#[rstest(
arr, value, expected,
case(array!(bool, [true, false, true, false]), true, array!(bool, [true, false, true, false])),
case(array!(bool, [[true, false], [true, false]]), true, array!(bool, [[true, false], [true, false]])),
)] fn test_bitand_value(arr: Result<Array<bool>, ArrayError>, value: bool, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr.unwrap() & value);
}

#[rstest(
arr1, arr2, expected,
case(array!(bool, [true, false, true, false]), array!(bool, [true, true, false, false]), array!(bool, [true, true, true, false])),
case(array!(bool, [[true, false], [true, false]]), array!(bool, [[true, true], [false, false]]), array!(bool, [[true, true], [true, false]])),
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([2, 2])\n right: Ok([4])")]
case(array!(bool, [[true, false], [true, false]]), array!(bool, [true, false, true, false]), array!(bool, [[true, false], [true, false]])),
)] fn test_bitor(arr1: Result<Array<bool>, ArrayError>, arr2: Result<Array<bool>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() | arr2.unwrap());
}

#[rstest(
arr, value, expected,
case(array!(bool, [true, false, true, false]), true, array!(bool, [true, true, true, true])),
case(array!(bool, [true, false, true, false]), false, array!(bool, [true, false, true, false])),
case(array!(bool, [[true, false], [true, false]]), true, array!(bool, [[true, true], [true, true]])),
case(array!(bool, [[true, false], [true, false]]), false, array!(bool, [[true, false], [true, false]])),
)] fn test_bitor_value(arr: Result<Array<bool>, ArrayError>, value: bool, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr.unwrap() | value);
}

#[rstest(
arr1, arr2, expected,
case(array!(bool, [true, false, true, false]), array!(bool, [true, true, false, false]), array!(bool, [false, true, true, false])),
case(array!(bool, [[true, false], [true, false]]), array!(bool, [[true, true], [false, false]]), array!(bool, [[false, true], [true, false]])),
#[should_panic(expected = "assertion `left == right` failed\n  left: Ok([2, 2])\n right: Ok([4])")]
case(array!(bool, [[true, false], [true, false]]), array!(bool, [true, false, true, false]), array!(bool, [[true, false], [true, false]])),
)] fn test_bitxor(arr1: Result<Array<bool>, ArrayError>, arr2: Result<Array<bool>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr1.unwrap() ^ arr2.unwrap());
}

#[rstest(
arr, value, expected,
case(array!(bool, [true, false, true, false]), true, array!(bool, [false, true, false, true])),
case(array!(bool, [true, false, true, false]), false, array!(bool, [true, false, true, false])),
case(array!(bool, [[true, false], [true, false]]), true, array!(bool, [[false, true], [false, true]])),
case(array!(bool, [[true, false], [true, false]]), false, array!(bool, [[true, false], [true, false]])),
)] fn test_bitxor_value(arr: Result<Array<bool>, ArrayError>, value: bool, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected.unwrap(), arr.unwrap() ^ value);
}

use arr_rs::prelude::*;
use rstest::rstest;

#[rstest(
array, other, expected,
case(array![i32, 10], array![i32, 15], array![i32, 30]),
case(array![i32, 20], array![i32, 30], array![i32, 60]),
case(array![i32, -10], array![i32, 20], array![i32, 20]),
case(array![i32, 10, 20], array![i32, 15, 30], array![i32, 30, 60]),
)] fn test_lcm(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.lcm(&other.unwrap()));
}

#[rstest(
array, other, expected,
case(array![i32, 10], array![i32, 15], array![i32, 5]),
case(array![i32, 20], array![i32, 30], array![i32, 10]),
case(array![i32, -10], array![i32, 20], array![i32, 10]),
case(array![i32, 10, 20], array![i32, 15, 30], array![i32, 5, 10]),
)] fn test_gcd(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.gcd(&other.unwrap()));
}

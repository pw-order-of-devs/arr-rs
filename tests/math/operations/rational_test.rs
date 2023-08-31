use arr_rs::prelude::*;
use rstest::rstest;

#[rstest(
array, other, expected,
case(array![10], array![15], array![30]),
case(array![20], array![30], array![60]),
case(array![-10], array![20], array![20]),
case(array![10, 20], array![15, 30], array![30, 60]),
)] fn test_lcm(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.lcm(&other.unwrap()));
}

#[rstest(
array, other, expected,
case(array![10], array![15], array![5]),
case(array![20], array![30], array![10]),
case(array![-10], array![20], array![10]),
case(array![10, 20], array![15, 30], array![5, 10]),
)] fn test_gcd(array: Result<Array<i32>, ArrayError>, other: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.gcd(&other.unwrap()));
}

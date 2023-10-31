use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array_single!(i32, 8), vec![1]),
case(array_single!(f64, 8.), vec![1]),
)] fn test_rand_array_macro<N: Numeric>(arr: Result<Array<N>, ArrayError>, expected: Vec<usize>) {
    assert_eq!(expected, arr.get_shape().unwrap());
}

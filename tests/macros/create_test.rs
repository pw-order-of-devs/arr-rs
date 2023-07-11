use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr, expected,
case(array!(4.), vec![1]),
case(array!([4.]), vec![1]),
case(array!([4., 6.]), vec![2]),
case(array!([[4., 6.], [4., 6.]]), vec![2, 2]),
case(array!([[[4., 6.], [4., 6.]], [[4., 6.], [4., 6.]]]), vec![2, 2, 2]),
case(array!([[[4., 6., 3.], [4., 6., 3.]], [[4., 6., 3.], [4., 6., 3.]]]), vec![2, 2, 3]),
)] fn test_new_array_macro(arr: Result<Array<f64>, ArrayError>, expected: Vec<usize>) {
    assert_eq!(expected, arr.get_shape().unwrap());
}

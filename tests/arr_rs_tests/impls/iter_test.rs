use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), array!([3, 6, 9, 12])),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), array!([[3, 6], [9, 12]])),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![8]), array!([3, 6, 9, 12, 3, 6, 9, 12])),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), array!([[3, 6, 9, 12], [3, 6, 9, 12]])),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), array!([[[3, 6], [9, 12]], [[3, 6], [9, 12]]])),
)] fn test_iter_array(array: Array<i32>, expected: Array<i32>) {
    let iterated: Array<i32> = array.clone().into_iter()
        .map(|i| i * 3)
        .collect::<Array<i32>>()
        .reshape(array.get_shape());
    assert_eq!(expected, iterated)
}

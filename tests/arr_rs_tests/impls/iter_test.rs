use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(Array::new(vec![1, 2, 3, 4], vec![4]), array!([3, 6, 9, 12])),
case(Array::new(vec![1, 2, 3, 4], vec![2, 2]), array!([[3, 6], [9, 12]])),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![8]), array!([3, 6, 9, 12, 3, 6, 9, 12])),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 4]), array!([[3, 6, 9, 12], [3, 6, 9, 12]])),
case(Array::new(vec![1, 2, 3, 4, 1, 2, 3, 4], vec![2, 2, 2]), array!([[[3, 6], [9, 12]], [[3, 6], [9, 12]]])),
)] fn test_iter_array(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    let shape = array.clone().unwrap().get_shape().unwrap();
    let iterated = array.unwrap().into_iter()
        .map(|i| i * 3)
        .collect::<Array<i32>>()
        .reshape(shape);
    assert_eq!(expected, iterated)
}

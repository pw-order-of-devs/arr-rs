use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr1, arr2, expected, expected_shape,
case(array!([[1], [2], [3]]), array!([[4, 5, 6]]), vec![(1, 4), (1, 5), (1, 6), (2, 4), (2, 5), (2, 6), (3, 4), (3, 5), (3, 6)], vec![3, 3]),
case(array!([1, 2]), array!([[2, 4], [5, 6]]), vec![(1, 2), (2, 4), (1, 5), (2, 6)], vec![2, 2]),
#[should_panic(expected = "incompatible shapes for broadcasting")]
case(array!([[1, 2, 3], [1, 2, 3]]), array!([[1, 2, 3, 4], [1, 2, 3, 4]]), vec![(1, 1)], vec![1]),
)] fn test_broadcast(arr1: Array<i32>, arr2: Array<i32>, expected: Vec<(i32, i32)>, expected_shape: Vec<usize>) {
    let expected = expected.into_iter().map(Tuple2::from_tuple).collect::<Vec<Tuple2<i32>>>();
    let expected = Array::new(expected, expected_shape);
    assert_eq!(expected, arr1.broadcast(&arr2))
}

#[rstest(
arr, shape, expected,
case(array!([[1], [2], [3]]), vec![3, 3], array!([[1, 1, 1], [2, 2, 2], [3, 3, 3]])),
case(array!([1, 2]), vec![2, 2], array!([[1, 2], [1, 2]])),
#[should_panic(expected = "incompatible shapes for broadcasting")]
case(array!([[1, 2, 3], [1, 2, 3]]), vec![2, 4], array![1]),
)] fn test_broadcast_to(arr: Array<i32>, shape: Vec<usize>, expected: Array<i32>) {
    assert_eq!(expected, arr.broadcast_to(shape))
}

#[rstest(
arrays, expected,
case(vec![array!([[1], [2], [3]]), array!([4, 5, 6])], vec![array!([[1, 1, 1], [2, 2, 2], [3, 3, 3]]), array!([[4, 5, 6], [4, 5, 6], [4, 5, 6]])]),
case(vec![array!([[1], [2], [3]]), array!([4, 5, 6]), array!([[1, 2, 3], [4, 5, 6], [7, 8, 9]])], vec![array!([[1, 1, 1], [2, 2, 2], [3, 3, 3]]), array!([[4, 5, 6], [4, 5, 6], [4, 5, 6]]), array!([[1, 2, 3], [4, 5, 6], [7, 8, 9]])]),
#[should_panic(expected = "incompatible shapes for broadcasting")]
case(vec![array!([[1], [2], [3]]), array!([4, 5, 6]), array!([[1, 2, 3], [4, 5, 6]])], vec![]),
)] fn test_broadcast_arrays(arrays: Vec<Array<i32>>, expected: Vec<Array<i32>>) {
    assert_eq!(expected, Array::broadcast_arrays(arrays))
}

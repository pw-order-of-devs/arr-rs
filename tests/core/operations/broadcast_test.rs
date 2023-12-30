use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
arr1, arr2, expected, expected_shape, result,
case(array!(i32, [[1, 2], [3, 4]]), array!(i32, [10, 20]), vec![(1, 10), (2, 20), (3, 10), (4, 20)], vec![2, 2], Ok(())),
case(array!(i32, [[1], [2], [3]]), array!(i32, [[4], [5], [6]]), vec![(1, 4), (2, 5), (3, 6)], vec![3, 1], Ok(())),
case(array!(i32, [1, 2, 3]), array!(i32, [[4], [5], [6]]), vec![(1, 4), (2, 4), (3, 4), (1, 5), (2, 5), (3, 5), (1, 6), (2, 6), (3, 6)], vec![3, 3], Ok(())),
case(array!(i32, [[1], [2], [3]]), array!(i32, [[4, 5, 6]]), vec![(1, 4), (1, 5), (1, 6), (2, 4), (2, 5), (2, 6), (3, 4), (3, 5), (3, 6)], vec![3, 3], Ok(())),
case(array!(i32, [1, 2]), array!(i32, [[2, 4], [5, 6]]), vec![(1, 2), (2, 4), (1, 5), (2, 6)], vec![2, 2], Ok(())),
case(array!(i32, [[[1, 2], [3, 4]]]), array!(i32, [[[2, 3], [3, 2]], [[2, 4], [2, 4]]]), vec![(1, 2), (2, 3), (3, 3), (4, 2), (1, 2), (2, 4), (3, 2), (4, 4)], vec![2, 2, 2], Ok(())),
case(array!(i32, [[[[1], [0]]], [[[0], [1]]]]), array!(i32, [[[[1, 1]], [[1, 1]]]]), vec![(1, 1), (1, 1), (0, 1), (0, 1), (1, 1), (1, 1), (0, 1), (0, 1), (0, 1), (0, 1), (1, 1), (1, 1), (0, 1), (0, 1), (1, 1), (1, 1)], vec![2, 2, 2, 2], Ok(())),
case(array!(i32, [[1, 2, 3], [1, 2, 3]]), array!(i32, [[1, 2, 3, 4], [1, 2, 3, 4]]), vec![(1, 1)], vec![1], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_broadcast(arr1: Result<Array<i32>, ArrayError>, arr2: Result<Array<i32>, ArrayError>, expected: Vec<(i32, i32)>, expected_shape: Vec<usize>, result: Result<(), ArrayError>) {
    let expected = expected.into_iter().map(Tuple2::from_tuple).collect::<Vec<Tuple2<i32, i32>>>();
    let expected = Array::new(expected, expected_shape);
    match result {
        Ok(_) => assert_eq!(expected, arr1.broadcast(&arr2.unwrap())),
        Err(err) => assert_eq!(Err(err), arr1.broadcast(&arr2.unwrap())),
    }
}

#[rstest(
arr, shape, expected,
case(array!(i32, [[1], [2], [3]]), vec![3, 3], array!(i32, [[1, 1, 1], [2, 2, 2], [3, 3, 3]])),
case(array!(i32, [1, 2]), vec![2, 2], array!(i32, [[1, 2], [1, 2]])),
case(array!(i32, [1, 2]), vec![1, 2, 2], array!(i32, [[[1, 2], [1, 2]]])),
case(array!(i32, [1, 2]), vec![1, 1, 1, 2], array!(i32, [[[[1, 2]]]])),
case(array!(i32, [1, 2]), vec![1, 1, 2, 1], array!(i32, [[[[1], [2]]]])),
case(array!(i32, [[[1, 2], [3, 4]]]), vec![2, 2, 2], array!(i32, [[[1, 2], [3, 4]], [[1, 2], [3, 4]]])),
case(array!(i32, [[[[1], [0]]], [[[0], [1]]]]), vec![2, 2, 2, 2], array!(i32, [[[[1, 1], [0, 0]], [[1, 1], [0, 0]]], [[[0, 0], [1, 1]], [[0, 0], [1, 1]]]])),
case(array!(i32, [[1, 2, 3], [1, 2, 3]]), vec![2, 4], Err(ArrayError::BroadcastShapeMismatch)),
case(array!(i32, [[[1, 2], [3, 4]], [[1, 2], [3, 4]]]), vec![2, 3], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_broadcast_to(arr: Result<Array<i32>, ArrayError>, shape: Vec<usize>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, arr.broadcast_to(shape))
}

#[rstest(
arrays, expected,
case(vec![array!(i32, [[1], [2], [3]]), array!(i32, [4, 5, 6])], Ok(vec![array!(i32, [[1, 1, 1], [2, 2, 2], [3, 3, 3]]), array!(i32, [[4, 5, 6], [4, 5, 6], [4, 5, 6]])])),
case(vec![array!(i32, [[1], [2], [3]]), array!(i32, [4, 5, 6]), array!(i32, [[1, 2, 3], [4, 5, 6], [7, 8, 9]])], Ok(vec![array!(i32, [[1, 1, 1], [2, 2, 2], [3, 3, 3]]), array!(i32, [[4, 5, 6], [4, 5, 6], [4, 5, 6]]), array!(i32, [[1, 2, 3], [4, 5, 6], [7, 8, 9]])])),
case(vec![array!(i32, [1, 2, 3]), array!(i32, [4]), array!(i32, [5, 6, 7])], Ok(vec![array!(i32, [1, 2, 3]), array!(i32, [4, 4, 4]), array!(i32, [5, 6, 7])])),
case(vec![array!(i32, [[1, 2, 3], [4, 5, 6]]), array!(i32, [1]), array!(i32, [[1], [2]])], Ok(vec![array!(i32, [[1, 2, 3], [4, 5, 6]]), array!(i32, [[1, 1, 1], [1, 1, 1]]), array!(i32, [[1, 1, 1], [2, 2, 2]])])),
case(vec![array!(i32, [[1], [2], [3]]), array!(i32, [4, 5, 6]), array!(i32, [[1, 2, 3], [4, 5, 6]])], Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_broadcast_arrays(arrays: Vec<Result<Array<i32>, ArrayError>>, expected: Result<Vec<Result<Array<i32>, ArrayError>>, ArrayError>) {
    let arrays = arrays.iter().map(|a| a.as_ref().unwrap().clone()).collect();
    match expected {
        Ok(expected) => {
            let expected = expected.iter().map(|a| a.as_ref().unwrap().clone()).collect::<Vec<Array<i32>>>();
            assert_eq!(Ok(expected), Array::broadcast_arrays(arrays))
        },
        Err(err) => assert_eq!(Err(err), Array::broadcast_arrays(arrays)),
    }
}

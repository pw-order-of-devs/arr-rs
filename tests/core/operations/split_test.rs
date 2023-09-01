use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, parts, axis, expected,
case(Array::empty(), 1, None, Ok(vec![Array::empty()])),
case(array!(i32, 1), 1, None, Ok(vec![array!(i32, 1)])),
case(array_arange!(i32, 0, 7), 1, None, Ok(vec![array_arange!(i32, 0, 7)])),
case(array_arange!(i32, 0, 7), 3, None, Ok(vec![array![i32, 0, 1, 2], array![i32, 3, 4, 5], array![i32, 6, 7]])),
case(array_arange!(i32, 0, 8), 4, None, Ok(vec![array![i32, 0, 1, 2], array![i32, 3, 4], array![i32, 5, 6], array![i32, 7, 8]])),
case(array_arange!(i32, 0, 7).reshape(&[4, 2]), 2, Some(0), Ok(vec![array!(i32, [[0, 1], [2, 3]]), array!(i32, [[4, 5], [6, 7]])])),
case(array_arange!(i32, 0, 8).reshape(&[3, 3]), 3, Some(1), Ok(vec![array!(i32, [[0], [3], [6]]), array!(i32, [[1], [4], [7]]), array!(i32, [[2], [5], [8]])])),
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 2, None, Ok(vec![array!(i32, [[[0, 1], [2, 3]]]), array!(i32, [[[4, 5], [6, 7]]])])),
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 2, Some(2), Ok(vec![array!(i32, [[[0], [2]], [[4], [6]]]), array!(i32, [[[1], [3]], [[5], [7]]])])),
case(array_arange!(i32, 0, 7), 0, None, Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })),
case(array_arange!(i32, 0, 7), 3, Some(2), Err(ArrayError::AxisOutOfBounds)),
)] fn test_array_split(array: Result<Array<i32>, ArrayError>, parts: usize, axis: Option<usize>, expected: Result<Vec<Result<Array<i32>, ArrayError>>, ArrayError>) {
    match expected {
        Ok(expected) => {
            let expected = expected.iter().map(|a| a.as_ref().unwrap().clone()).collect::<Vec<Array<i32>>>();
            assert_eq!(Ok(expected), array.unwrap().array_split(parts, axis))
        },
        Err(err) => assert_eq!(Err(err), array.unwrap().array_split(parts, axis)),
    }
}

#[rstest(
array, parts, axis, expected,
case(Array::empty(), 1, None, Ok(vec![Array::empty()])),
case(array!(i32, 1), 1, None, Ok(vec![array!(i32, 1)])),
case(array!(i32, [[2], [3], [5]]), 3, Some(0), Ok(vec![array!(i32, [[2]]), array!(i32, [[3]]), array!(i32, [[5]])])),
case(array_arange!(i32, 0, 7), 1, None, Ok(vec![array_arange!(i32, 0, 7)])),
case(array_arange!(i32, 0, 8), 3, None, Ok(vec![array![i32, 0, 1, 2], array![i32, 3, 4, 5], array![i32, 6, 7, 8]])),
case(array_arange!(i32, 0, 7).reshape(&[4, 2]), 2, Some(0), Ok(vec![array!(i32, [[0, 1], [2, 3]]), array!(i32, [[4, 5], [6, 7]])])),
case(array_arange!(i32, 0, 8).reshape(&[3, 3]), 3, Some(1), Ok(vec![array!(i32, [[0], [3], [6]]), array!(i32, [[1], [4], [7]]), array!(i32, [[2], [5], [8]])])),
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 2, None, Ok(vec![array!(i32, [[[0, 1], [2, 3]]]), array!(i32, [[[4, 5], [6, 7]]])])),
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 2, Some(2), Ok(vec![array!(i32, [[[0], [2]], [[4], [6]]]), array!(i32, [[[1], [3]], [[5], [7]]])])),
case(array_arange!(i32, 0, 7), 3, None, Err(ArrayError::ParameterError { param: "parts", message: "array split does not result in an equal division" })),
case(array_arange!(i32, 0, 7), 0, None, Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0" })),
case(array_arange!(i32, 0, 7), 3, Some(2), Err(ArrayError::AxisOutOfBounds)),
)] fn test_split(array: Result<Array<i32>, ArrayError>, parts: usize, axis: Option<usize>, expected: Result<Vec<Result<Array<i32>, ArrayError>>, ArrayError>) {
    match expected {
        Ok(expected) => {
            let expected = expected.iter().map(|a| a.as_ref().unwrap().clone()).collect::<Vec<Array<i32>>>();
            assert_eq!(Ok(expected), array.split(parts, axis))
        },
        Err(err) => assert_eq!(Err(err), array.split(parts, axis)),
    }
}

#[rstest(
array, axis, expected,
case(Array::empty(), 0, Ok(vec![Array::empty()])),
case(array!(i32, 1), 0, Ok(vec![array!(i32, 1)])),
case(array!(i32, [[2], [3], [5]]), 0, Ok(vec![array!(i32, [[2]]), array!(i32, [[3]]), array!(i32, [[5]])])),
case(array_arange!(i32, 0, 7), 0, Ok(vec![array_arange!(i32, 0, 7)])),
case(array_arange!(i32, 0, 7).reshape(&[4, 2]), 0, Ok(vec![array!(i32, [[0, 1]]), array!(i32, [[2, 3]]), array!(i32, [[4, 5]]), array!(i32, [[6, 7]])])),
case(array_arange!(i32, 0, 8).reshape(&[3, 3]), 1, Ok(vec![array!(i32, [[0], [3], [6]]), array!(i32, [[1], [4], [7]]), array!(i32, [[2], [5], [8]])])),
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 0, Ok(vec![array!(i32, [[[0, 1], [2, 3]]]), array!(i32, [[[4, 5], [6, 7]]])])),
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 2, Ok(vec![array!(i32, [[[0], [2]], [[4], [6]]]), array!(i32, [[[1], [3]], [[5], [7]]])])),
case(array_arange!(i32, 0, 7), 2, Err(ArrayError::AxisOutOfBounds)),
)] fn test_split_axis(array: Result<Array<i32>, ArrayError>, axis: usize, expected: Result<Vec<Result<Array<i32>, ArrayError>>, ArrayError>) {
    match expected {
        Ok(expected) => {
            let expected = expected.iter().map(|a| a.as_ref().unwrap().clone()).collect::<Vec<Array<i32>>>();
            assert_eq!(Ok(expected), array.split_axis(axis))
        },
        Err(err) => assert_eq!(Err(err), array.split_axis(axis)),
    }
}

#[rstest(
array, parts, expected,
case(array_arange!(i32, 0, 5), 3, vec![array!(i32, [0, 1]), array!(i32, [2, 3]), array!(i32, [4, 5])]),
case(array_arange!(i32, 0, 5).reshape(&[3, 2]), 2, vec![array!(i32, [[0], [2], [4]]), array!(i32, [[1], [3], [5]])]),
case(array_arange!(i32, 0, 5).reshape(&[2, 3]), 3, vec![array!(i32, [[0], [3]]), array!(i32, [[1], [4]]), array!(i32, [[2], [5]])]),
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 2, vec![array!(i32, [[[0, 1]], [[4, 5]]]), array!(i32, [[[2, 3]], [[6, 7]]])]),
#[should_panic(expected = "array split does not result in an equal division")]
case(array_arange!(i32, 0, 6), 3, vec![]),
)] fn test_hsplit(array: Result<Array<i32>, ArrayError>, parts: usize, expected: Vec<Result<Array<i32>, ArrayError>>) {
    let expected = expected.iter().map(|a| a.as_ref().unwrap().clone()).collect::<Vec<Array<i32>>>();
    assert_eq!(Ok(expected), array.hsplit(parts))
}

#[rstest(
array, parts, expected,
case(array_arange!(i32, 0, 5).reshape(&[3, 2]), 3, vec![array!(i32, [[0, 1]]), array!(i32, [[2, 3]]), array!(i32, [[4, 5]])]),
case(array_arange!(i32, 0, 5).reshape(&[2, 3]), 2, vec![array!(i32, [[0, 1, 2]]), array!(i32, [[3, 4, 5]])]),
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 2, vec![array!(i32, [[[0, 1], [2, 3]]]), array!(i32, [[[4, 5], [6, 7]]])]),
#[should_panic(expected = "array split does not result in an equal division")]
case(array_arange!(i32, 0, 5).reshape(&[2, 3]), 4, vec![]),
)] fn test_vsplit(array: Result<Array<i32>, ArrayError>, parts: usize, expected: Vec<Result<Array<i32>, ArrayError>>) {
    let expected = expected.iter().map(|a| a.as_ref().unwrap().clone()).collect::<Vec<Array<i32>>>();
    assert_eq!(Ok(expected), array.vsplit(parts))
}

#[rstest(
array, parts, expected,
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 2, vec![array!(i32, [[[0], [2]], [[4], [6]]]), array!(i32, [[[1], [3]], [[5], [7]]])]),
case(array_arange!(i32, 0, 11).reshape(&[3, 2, 2]), 2, vec![array!(i32, [[[0], [2]], [[4], [6]], [[8], [10]]]), array!(i32, [[[1], [3]], [[5], [7]], [[9], [11]]])]),
case(array_arange!(i32, 0, 11).reshape(&[2, 3, 2]), 2, vec![array!(i32, [[[0], [2], [4]], [[6], [8], [10]]]), array!(i32, [[[1], [3], [5]], [[7], [9], [11]]])]),
case(array_arange!(i32, 0, 11).reshape(&[2, 2, 3]), 3, vec![array!(i32, [[[0], [3]], [[6], [9]]]), array!(i32, [[[1], [4]], [[7], [10]]]), array!(i32, [[[2], [5]], [[8], [11]]])]),
#[should_panic(expected = "array split does not result in an equal division")]
case(array_arange!(i32, 0, 7).reshape(&[2, 2, 2]), 4, vec![array!(i32, [[[0], [2]], [[4], [6]]]), array!(i32, [[[1], [3]], [[5], [7]]])]),
)] fn test_dsplit(array: Result<Array<i32>, ArrayError>, parts: usize, expected: Vec<Result<Array<i32>, ArrayError>>) {
    let expected = expected.iter().map(|a| a.as_ref().unwrap().clone()).collect::<Vec<Array<i32>>>();
    assert_eq!(Ok(expected), array.dsplit(parts))
}

use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, axes, expected,
case(array_arange!(0, 3).reshape(vec![2, 2]), None, array!([[3, 2], [1, 0]])),
case(array_arange!(0, 3).reshape(vec![2, 2]), Some(vec![1]), array!([[1, 0], [3, 2]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), None, array!([[[7, 6], [5, 4]], [[3, 2], [1, 0]]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), Some(vec![0]), array!([[[4, 5], [6, 7]], [[0, 1], [2, 3]]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), Some(vec![1]), array!([[[2, 3], [0, 1]], [[6, 7], [4, 5]]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), Some(vec![2]), array!([[[1, 0], [3, 2]], [[5, 4], [7, 6]]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), Some(vec![1, 2]), array!([[[3, 2], [1, 0]], [[7, 6], [5, 4]]])),
case(array_arange!(0, 15).reshape(vec![2, 2, 2, 2]), Some(vec![1]), array!([[[[4, 5], [6, 7]], [[0, 1], [2, 3]]], [[[12, 13], [14, 15]], [[8, 9], [10, 11]]]])),
case(array_arange!(0, 15).reshape(vec![2, 2, 2, 2]), Some(vec![2]), array!([[[[2, 3], [0, 1]], [[6, 7], [4, 5]]], [[[10, 11], [8, 9]], [[14, 15], [12, 13]]]])),
case(array_arange!(0, 15).reshape(vec![2, 2, 2, 2]), Some(vec![1, 2]), array!([[[[6, 7], [4, 5]], [[2, 3], [0, 1]]], [[[14, 15], [12, 13]], [[10, 11], [8, 9]]]])),
)] fn test_flip(array: Result<Array<i32>, ArrayError>, axes: Option<Vec<isize>>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.flip(axes))
}

#[rstest(
array, expected,
case(array_arange!(0, 3).reshape(vec![2, 2]), array!([[2, 3], [0, 1]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), array!([[[4, 5], [6, 7]], [[0, 1], [2, 3]]])),
case(array_arange!(0, 15).reshape(vec![2, 2, 2, 2]), array!([[[[8, 9], [10, 11]], [[12, 13], [14, 15]]], [[[0, 1], [2, 3]], [[4, 5], [6, 7]]]])),
)] fn test_flipud(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.flipud())
}

#[rstest(
array, expected,
case(array_arange!(0, 3).reshape(vec![2, 2]), array!([[1, 0], [3, 2]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), array!([[[2, 3], [0, 1]], [[6, 7], [4, 5]]])),
case(array_arange!(0, 15).reshape(vec![2, 2, 2, 2]), array!([[[[4, 5], [6, 7]], [[0, 1], [2, 3]]], [[[12, 13], [14, 15]], [[8, 9], [10, 11]]]])),
)] fn test_fliplr(array: Result<Array<i32>, ArrayError>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.fliplr())
}

#[rstest(
array, shifts, axes, expected,
case(array_arange!(0, 9), vec![2], None, array!([8, 9, 0, 1, 2, 3, 4, 5, 6, 7])),
case(array_arange!(0, 9), vec![-2], None, array!([2, 3, 4, 5, 6, 7, 8, 9, 0, 1])),
case(array_arange!(0, 9).reshape(vec![2, 5]), vec![1], None, array!([[9, 0, 1, 2, 3], [4, 5, 6, 7, 8]])),
case(array_arange!(0, 9).reshape(vec![2, 5]), vec![-1], None, array!([[1, 2, 3, 4, 5], [6, 7, 8, 9, 0]])),
case(array_arange!(0, 9).reshape(vec![2, 5]), vec![1], Some(vec![0]), array!([[5, 6, 7, 8, 9], [0, 1, 2, 3, 4]])),
case(array_arange!(0, 9).reshape(vec![2, 5]), vec![-1], Some(vec![0]), array!([[5, 6, 7, 8, 9], [0, 1, 2, 3, 4]])),
case(array_arange!(0, 9).reshape(vec![2, 5]), vec![1], Some(vec![1]), array!([[4, 0, 1, 2, 3], [9, 5, 6, 7, 8]])),
case(array_arange!(0, 9).reshape(vec![2, 5]), vec![-1], Some(vec![1]), array!([[1, 2, 3, 4, 0], [6, 7, 8, 9, 5]])),
case(array_arange!(0, 9).reshape(vec![2, 5]), vec![1, 1], Some(vec![1, 0]), array!([[9, 5, 6, 7, 8], [4, 0, 1, 2, 3]])),
case(array_arange!(0, 9).reshape(vec![2, 5]), vec![2, 1], Some(vec![1, 0]), array!([[8, 9, 5, 6, 7], [3, 4, 0, 1, 2]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), vec![1], None, array!([[[7, 0], [1, 2]], [[3, 4], [5, 6]]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), vec![1], Some(vec![0]), array!([[[4, 5], [6, 7]], [[0, 1], [2, 3]]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), vec![1], Some(vec![1]), array!([[[2, 3], [0, 1]], [[6, 7], [4, 5]]])),
case(array_arange!(0, 7).reshape(vec![2, 2, 2]), vec![1], Some(vec![2]), array!([[[1, 0], [3, 2]], [[5, 4], [7, 6]]])),
)] fn test_roll(array: Result<Array<i32>, ArrayError>, shifts: Vec<isize>, axes: Option<Vec<isize>>, expected: Result<Array<i32>, ArrayError>) {
    assert_eq!(expected, array.roll(shifts, axes))
}

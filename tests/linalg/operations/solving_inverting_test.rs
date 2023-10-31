use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, expected,
case(array!(f64, [[2, 1], [1, 3]]), array_flat!(f64, 5, 8), array_flat!(f64, 1.4, 2.2)),
case(array!(f64, [[2, 1], [1, 3]]), array!(f64, [[5, 8], [3, 6]]), array!(f64, [[2.4, 3.6], [0.2, 0.8]])),
case(array!(f64, [[3, -2, 1], [1, 2, 3], [2, -3, 2]]), array_flat!(f64, 4, 6, 1), array_flat!(f64, 2.0833333333333335, 1.3333333333333333, 0.4166666666666669)),
case(array!(f64, [[5, 2, -1, 0, 3], [1, 8, 3, -2, 4], [2, -3, 7, 1, -5], [0, 2, 1, 6, -4], [3, 4, -5, 2, 10]]), array_flat!(f64, 10, 15, -2, 7, 11), array_flat!(f64, 1.3284015085581664, 1.9044096315636787, -0.023498694516971223, 0.43037423846823336, -0.15810850014505357)),
case(array!(f64, [[1, 1, 1], [0, 0, 1], [1, 1, 2]]), array_flat!(f64, 3, 1, 6), Err(ArrayError::SingularMatrix)),
case(array!(f64, [[2, 1, 3], [1, 3, 2]]), array_flat!(f64, 3, 1), Err(ArrayError::MustBeEqual { value1: "2".to_string(), value2: "3".to_string() })),
case(array!(f64, [[1, 2], [3, 4], [5, 6]]), array_flat!(f64, 3, 1, 6), Err(ArrayError::MustBeEqual { value1: "3".to_string(), value2: "2".to_string() })),
case(array!(f64, [[1, 2], [2, 4]]), array_flat!(f64, 3, 6), Err(ArrayError::SingularMatrix)),
)] fn test_linalg_solve(array: Result<Array<f64>, ArrayError>, other: Result<Array<f64>, ArrayError>, expected: Result<Array<f64>, ArrayError>) {
    assert_eq!(expected, array.solve(&other.unwrap()))
}

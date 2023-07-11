use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec![true, true])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::flat(vec![false, false])),
case(Array::new(vec!["dd".to_string(), "cde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::new(vec![true, false, false, true], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_equal(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.equal(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec![false, false])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::flat(vec![true, true])),
case(Array::new(vec!["dd".to_string(), "cde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::new(vec![false, true, true, false], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_not_equal(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.not_equal(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec![true, true])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::flat(vec![false, false])),
case(Array::new(vec!["dd".to_string(), "cde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::new(vec![true, false, false, true], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_greater_equal(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.greater_equal(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec![true, true])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::flat(vec![true, true])),
case(Array::new(vec!["dd".to_string(), "cde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::new(vec![true, true, true, true], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_less_equal(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.less_equal(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec![false, false])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::flat(vec![false, false])),
case(Array::new(vec!["gd".to_string(), "gde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::new(vec![true, true, false, false], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_greater(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.greater(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec![false, false])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::flat(vec![true, true])),
case(Array::new(vec!["gd".to_string(), "gde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::new(vec![false, false, true, false], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_less(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.less(&other.unwrap()))
}

#[rstest(
array, other, cmp_op, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["abc".to_string(), "cde".to_string()]), "==", Array::flat(vec![true, true])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), "!=", Array::flat(vec![true, true])),
case(Array::new(vec!["gd".to_string(), "gde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), ">=", Array::new(vec![true, true, false, true], vec![2, 2])),
case(Array::new(vec!["gd".to_string(), "gde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), "<=", Array::new(vec![false, false, true, true], vec![2, 2])),
case(Array::new(vec!["gd".to_string(), "gde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), ">", Array::new(vec![true, true, false, false], vec![2, 2])),
case(Array::new(vec!["gd".to_string(), "gde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), "<", Array::new(vec![false, false, true, false], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), "==", Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_compare(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, cmp_op: &str, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.compare(&other.unwrap(), cmp_op))
}

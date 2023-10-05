use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec![3, 3])),
case(Array::new(vec!["gd".to_string(), "gde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::new(vec![2, 3, 3, 2], vec![2, 2])),
)] fn test_char_str_len(array: Result<Array<String>, ArrayError>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, array.str_len())
}

#[rstest(
array, other, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::single("a".to_string()), Array::flat(vec![1, 0])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["a".to_string(), "c".to_string()]), Array::flat(vec![1, 1])),
case(Array::new(vec!["abc".to_string(), "cde".to_string(), "abc".to_string(), "cde".to_string()], vec![2, 2]), Array::flat(vec!["a".to_string(), "c".to_string()]), Array::new(vec![1, 1, 1, 1], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_count(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, array.count(&other.unwrap()))
}

#[rstest(
array, sub, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::single("a".to_string()), Array::flat(vec![true, false])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["a".to_string(), "c".to_string()]), Array::flat(vec![true, true])),
case(Array::new(vec!["abc".to_string(), "cde".to_string(), "abc".to_string(), "cde".to_string()], vec![2, 2]), Array::flat(vec!["a".to_string(), "b".to_string()]), Array::new(vec![true, false, true, false], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_starts_with(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.starts_with(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::single("c".to_string()), Array::flat(vec![true, false])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["c".to_string(), "e".to_string()]), Array::flat(vec![true, true])),
case(Array::new(vec!["abc".to_string(), "cde".to_string(), "abc".to_string(), "cde".to_string()], vec![2, 2]), Array::flat(vec!["b".to_string(), "e".to_string()]), Array::new(vec![false, true, false, true], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_ends_with(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.ends_with(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::single("a".to_string()), Array::flat(vec![0, -1])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["a".to_string(), "c".to_string()]), Array::flat(vec![0, 0])),
case(Array::new(vec!["abc".to_string(), "cde".to_string(), "abc".to_string(), "cde".to_string()], vec![2, 2]), Array::flat(vec!["a".to_string(), "b".to_string()]), Array::new(vec![0, -1, 0, -1], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_find(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<isize>, ArrayError>) {
    assert_eq!(expected, array.find(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::single("b".to_string()), Array::flat(vec![1, -1])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["c".to_string(), "e".to_string()]), Array::flat(vec![2, 2])),
case(Array::new(vec!["abc".to_string(), "cde".to_string(), "abc".to_string(), "cde".to_string()], vec![2, 2]), Array::flat(vec!["b".to_string(), "f".to_string()]), Array::new(vec![1, -1, 1, -1], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rfind(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<isize>, ArrayError>) {
    assert_eq!(expected, array.rfind(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::single("a".to_string()), Array::flat(vec![0, -1])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["a".to_string(), "c".to_string()]), Array::flat(vec![0, 0])),
case(Array::new(vec!["abc".to_string(), "cde".to_string(), "abc".to_string(), "cde".to_string()], vec![2, 2]), Array::flat(vec!["a".to_string(), "b".to_string()]), Array::new(vec![0, -1, 0, -1], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_index(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<isize>, ArrayError>) {
    assert_eq!(expected, array.index(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::single("b".to_string()), Array::flat(vec![1, -1])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["c".to_string(), "e".to_string()]), Array::flat(vec![2, 2])),
case(Array::new(vec!["abc".to_string(), "cde".to_string(), "abc".to_string(), "cde".to_string()], vec![2, 2]), Array::flat(vec!["b".to_string(), "f".to_string()]), Array::new(vec![1, -1, 1, -1], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rindex(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<isize>, ArrayError>) {
    assert_eq!(expected, array.rindex(&sub.unwrap()))
}

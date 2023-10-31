use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array_flat!(String, "abc", "cde"), array_flat!(usize, 3, 3)),
case(array!(String, [["gd", "gde"], ["abc", "ff"]]), array!(usize, [[2, 3], [3, 2]])),
)] fn test_char_str_len(array: Result<Array<String>, ArrayError>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, array.str_len())
}

#[rstest(
array, other, expected,
case(array_flat!(String, "abc", "cde"), array_single!(String, "a"), array_flat!(usize, 1, 0)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "a", "c"), array_flat!(usize, 1, 1)),
case(array!(String, [["abc", "cde"], ["abc", "cde"]]), array_flat!(String, "a", "c"), array!(usize, [[1, 1], [1, 1]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_count(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, array.count(&other.unwrap()))
}

#[rstest(
array, sub, expected,
case(array_flat!(String, "abc", "cde"), array_single!(String, "a"), array_flat!(bool, true, false)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "a", "c"), array_flat!(bool, true, true)),
case(array!(String, [["abc", "cde"], ["abc", "cde"]]), array_flat!(String, "a", "b"), array!(bool, [[true, false], [true, false]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_starts_with(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.starts_with(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(array_flat!(String, "abc", "cde"), array_single!(String, "c"), array_flat!(bool, true, false)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "c", "e"), array_flat!(bool, true, true)),
case(array!(String, [["abc", "cde"], ["abc", "cde"]]), array_flat!(String, "b", "e"), array!(bool, [[false, true], [false, true]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_ends_with(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.ends_with(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(array_flat!(String, "abc", "cde"), array_single!(String, "a"), array_flat!(isize, 0, -1)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "a", "c"), array_flat!(isize, 0, 0)),
case(array!(String, [["abc", "cde"], ["abc", "cde"]]), array_flat!(String, "a", "b"), array!(isize, [[0, -1], [0, -1]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_find(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<isize>, ArrayError>) {
    assert_eq!(expected, array.find(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(array_flat!(String, "abc", "cde"), array_single!(String, "b"), array_flat!(isize, 1, -1)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "c", "e"), array_flat!(isize, 2, 2)),
case(array!(String, [["abc", "cde"], ["abc", "cde"]]), array_flat!(String, "b", "f"), array!(isize, [[1, -1], [1, -1]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rfind(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<isize>, ArrayError>) {
    assert_eq!(expected, array.rfind(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(array_flat!(String, "abc", "cde"), array_single!(String, "a"), array_flat!(isize, 0, -1)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "a", "c"), array_flat!(isize, 0, 0)),
case(array!(String, [["abc", "cde"], ["abc", "cde"]]), array_flat!(String, "a", "b"), array!(isize, [[0, -1], [0, -1]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_index(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<isize>, ArrayError>) {
    assert_eq!(expected, array.index(&sub.unwrap()))
}

#[rstest(
array, sub, expected,
case(array_flat!(String, "abc", "cde"), array_single!(String, "b"), array_flat!(isize, 1, -1)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "c", "e"), array_flat!(isize, 2, 2)),
case(array!(String, [["abc", "cde"], ["abc", "cde"]]), array_flat!(String, "b", "f"), array!(isize, [[1, -1], [1, -1]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rindex(array: Result<Array<String>, ArrayError>, sub: Result<Array<String>, ArrayError>, expected: Result<Array<isize>, ArrayError>) {
    assert_eq!(expected, array.rindex(&sub.unwrap()))
}

use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, expected,
case(array_flat!(String, "abc", "cde"), array_flat!(String, "abc", "cde"), array_flat!(bool, true, true)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "dd", "ff"), array_flat!(bool, false, false)),
case(array!(String, [["dd", "cde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), array!(bool, [[true, false], [false, true]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_equal(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.equal(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat!(String, "abc", "cde"), array_flat!(String, "abc", "cde"), array_flat!(bool, false, false)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "dd", "ff"), array_flat!(bool, true, true)),
case(array!(String, [["dd", "cde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), array!(bool, [[false, true], [true, false]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_not_equal(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.not_equal(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat!(String, "abc", "cde"), array_flat!(String, "abc", "cde"), array_flat!(bool, true, true)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "dd", "ff"), array_flat!(bool, false, false)),
case(array!(String, [["dd", "cde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), array!(bool, [[true, false], [false, true]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_greater_equal(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.greater_equal(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat!(String, "abc", "cde"), array_flat!(String, "abc", "cde"), array_flat!(bool, true, true)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "dd", "ff"), array_flat!(bool, true, true)),
case(array!(String, [["dd", "cde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), array!(bool, [[true, true], [true, true]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_less_equal(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.less_equal(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat!(String, "abc", "cde"), array_flat!(String, "abc", "cde"), array_flat!(bool, false, false)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "dd", "ff"), array_flat!(bool, false, false)),
case(array!(String, [["gd", "gde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), array!(bool, [[true, true], [false, false]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_greater(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.greater(&other.unwrap()))
}

#[rstest(
array, other, expected,
case(array_flat!(String, "abc", "cde"), array_flat!(String, "abc", "cde"), array_flat!(bool, false, false)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "dd", "ff"), array_flat!(bool, true, true)),
case(array!(String, [["gd", "gde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), array!(bool, [[false, false], [true, false]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_less(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.less(&other.unwrap()))
}

#[rstest(
array, other, cmp_op, expected,
case(array_flat!(String, "abc", "cde"), array_flat!(String, "abc", "cde"), "==", array_flat!(bool, true, true)),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "dd", "ff"), "!=", array_flat!(bool, true, true)),
case(array!(String, [["gd", "gde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), ">=", array!(bool, [[true, true], [false, true]])),
case(array!(String, [["gd", "gde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), "<=", array!(bool, [[false, false], [true, true]])),
case(array!(String, [["gd", "gde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), ">", array!(bool, [[true, true], [false, false]])),
case(array!(String, [["gd", "gde"], ["abc", "ff"]]), array_flat!(String, "dd", "ff"), "<", array!(bool, [[false, false], [true, false]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), ">>>", Err(ArrayError::ParameterError { param: "`op`", message: "must be one of {`==`, `!=`, `>`, `<`, `>=`, `<=`}" })),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), "==", Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_compare(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, cmp_op: &str, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.compare(&other.unwrap(), cmp_op))
}

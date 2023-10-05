use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array_flat!(String, "abc"), array_flat!(bool, true)),
case(array_flat!(String, "aBc"), array_flat!(bool, true)),
case(array_flat!(String, "ab1"), array_flat!(bool, false)),
case(array_flat!(String, "Ab1"), array_flat!(bool, false)),
case(array_flat!(String, " "), array_flat!(bool, false)),
case(array_flat!(String, ""), array_flat!(bool, false)),
)] fn test_char_is_alpha(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_alpha())
}

#[rstest(
array, expected,
case(array_flat!(String, "abc"), array_flat!(bool, true)),
case(array_flat!(String, "aBc"), array_flat!(bool, true)),
case(array_flat!(String, "ab1"), array_flat!(bool, true)),
case(array_flat!(String, "Ab1"), array_flat!(bool, true)),
case(array_flat!(String, " "), array_flat!(bool, false)),
case(array_flat!(String, ""), array_flat!(bool, false)),
)] fn test_char_is_alnum(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_alnum())
}

#[rstest(
array, expected,
case(array_flat!(String, "1"), array_flat!(bool, true)),
case(array_flat!(String, "123"), array_flat!(bool, true)),
case(array_flat!(String, "abc"), array_flat!(bool, false)),
case(array_flat!(String, "aBc"), array_flat!(bool, false)),
case(array_flat!(String, "ab1"), array_flat!(bool, false)),
case(array_flat!(String, "123.32"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "VII"), array_flat!(bool, false)),
case(array_flat!(String, " "), array_flat!(bool, false)),
case(array_flat!(String, ""), array_flat!(bool, false)),
)] fn test_char_is_decimal(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_decimal())
}

#[rstest(
array, expected,
case(array_flat!(String, "1"), array_flat!(bool, true)),
case(array_flat!(String, "123"), array_flat!(bool, true)),
case(array_flat!(String, "abc"), array_flat!(bool, false)),
case(array_flat!(String, "aBc"), array_flat!(bool, false)),
case(array_flat!(String, "ab1"), array_flat!(bool, false)),
case(array_flat!(String, "123.32"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "VII"), array_flat!(bool, false)),
case(array_flat!(String, " "), array_flat!(bool, false)),
case(array_flat!(String, ""), array_flat!(bool, false)),
)] fn test_char_is_numeric(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_numeric())
}

#[rstest(
array, expected,
case(array_flat!(String, "1"), array_flat!(bool, true)),
case(array_flat!(String, "123"), array_flat!(bool, false)),
case(array_flat!(String, "abc"), array_flat!(bool, false)),
case(array_flat!(String, "aBc"), array_flat!(bool, false)),
case(array_flat!(String, "ab1"), array_flat!(bool, false)),
case(array_flat!(String, "123.32"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "VII"), array_flat!(bool, false)),
case(array_flat!(String, " "), array_flat!(bool, false)),
case(array_flat!(String, ""), array_flat!(bool, false)),
)] fn test_char_is_digit(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_digit())
}

#[rstest(
array, expected,
case(array_flat!(String, " "), array_flat!(bool, true)),
case(array_flat!(String, "1"), array_flat!(bool, false)),
case(array_flat!(String, "123"), array_flat!(bool, false)),
case(array_flat!(String, "abc"), array_flat!(bool, false)),
case(array_flat!(String, "aBc"), array_flat!(bool, false)),
case(array_flat!(String, "ab1"), array_flat!(bool, false)),
case(array_flat!(String, "123.32"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "VII"), array_flat!(bool, false)),
case(array_flat!(String, ""), array_flat!(bool, false)),
)] fn test_char_is_space(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_space())
}

#[rstest(
array, expected,
case(array_flat!(String, "abc"), array_flat!(bool, true)),
case(array_flat!(String, "ab1"), array_flat!(bool, true)),
case(array_flat!(String, " "), array_flat!(bool, false)),
case(array_flat!(String, "1"), array_flat!(bool, false)),
case(array_flat!(String, "123"), array_flat!(bool, false)),
case(array_flat!(String, "aBc"), array_flat!(bool, false)),
case(array_flat!(String, "ABC"), array_flat!(bool, false)),
case(array_flat!(String, "123.32"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "VII"), array_flat!(bool, false)),
case(array_flat!(String, ""), array_flat!(bool, false)),
)] fn test_char_is_lower(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_lower())
}

#[rstest(
array, expected,
case(array_flat!(String, "ABC"), array_flat!(bool, true)),
case(array_flat!(String, "VII"), array_flat!(bool, true)),
case(array_flat!(String, " "), array_flat!(bool, false)),
case(array_flat!(String, "1"), array_flat!(bool, false)),
case(array_flat!(String, "123"), array_flat!(bool, false)),
case(array_flat!(String, "abc"), array_flat!(bool, false)),
case(array_flat!(String, "aBc"), array_flat!(bool, false)),
case(array_flat!(String, "ab1"), array_flat!(bool, false)),
case(array_flat!(String, "123.32"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, "123/435"), array_flat!(bool, false)),
case(array_flat!(String, ""), array_flat!(bool, false)),
)] fn test_char_is_upper(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_upper())
}

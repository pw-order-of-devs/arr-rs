use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(Array::single("abc".to_string()), Array::single(true)),
case(Array::single("aBc".to_string()), Array::single(true)),
case(Array::single("ab1".to_string()), Array::single(false)),
case(Array::single("Ab1".to_string()), Array::single(false)),
case(Array::single(" ".to_string()), Array::single(false)),
case(Array::single("".to_string()), Array::single(false)),
)] fn test_char_is_alpha(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_alpha())
}

#[rstest(
array, expected,
case(Array::single("abc".to_string()), Array::single(true)),
case(Array::single("aBc".to_string()), Array::single(true)),
case(Array::single("ab1".to_string()), Array::single(true)),
case(Array::single("Ab1".to_string()), Array::single(true)),
case(Array::single(" ".to_string()), Array::single(false)),
case(Array::single("".to_string()), Array::single(false)),
)] fn test_char_is_alnum(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_alnum())
}

#[rstest(
array, expected,
case(Array::single("1".to_string()), Array::single(true)),
case(Array::single("123".to_string()), Array::single(true)),
case(Array::single("abc".to_string()), Array::single(false)),
case(Array::single("aBc".to_string()), Array::single(false)),
case(Array::single("ab1".to_string()), Array::single(false)),
case(Array::single("123.32".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("VII".to_string()), Array::single(false)),
case(Array::single(" ".to_string()), Array::single(false)),
case(Array::single("".to_string()), Array::single(false)),
)] fn test_char_is_decimal(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_decimal())
}

#[rstest(
array, expected,
case(Array::single("1".to_string()), Array::single(true)),
case(Array::single("123".to_string()), Array::single(true)),
case(Array::single("abc".to_string()), Array::single(false)),
case(Array::single("aBc".to_string()), Array::single(false)),
case(Array::single("ab1".to_string()), Array::single(false)),
case(Array::single("123.32".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("VII".to_string()), Array::single(false)),
case(Array::single(" ".to_string()), Array::single(false)),
case(Array::single("".to_string()), Array::single(false)),
)] fn test_char_is_numeric(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_numeric())
}

#[rstest(
array, expected,
case(Array::single("1".to_string()), Array::single(true)),
case(Array::single("123".to_string()), Array::single(false)),
case(Array::single("abc".to_string()), Array::single(false)),
case(Array::single("aBc".to_string()), Array::single(false)),
case(Array::single("ab1".to_string()), Array::single(false)),
case(Array::single("123.32".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("VII".to_string()), Array::single(false)),
case(Array::single(" ".to_string()), Array::single(false)),
case(Array::single("".to_string()), Array::single(false)),
)] fn test_char_is_digit(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_digit())
}

#[rstest(
array, expected,
case(Array::single(" ".to_string()), Array::single(true)),
case(Array::single("1".to_string()), Array::single(false)),
case(Array::single("123".to_string()), Array::single(false)),
case(Array::single("abc".to_string()), Array::single(false)),
case(Array::single("aBc".to_string()), Array::single(false)),
case(Array::single("ab1".to_string()), Array::single(false)),
case(Array::single("123.32".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("VII".to_string()), Array::single(false)),
case(Array::single("".to_string()), Array::single(false)),
)] fn test_char_is_space(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_space())
}

#[rstest(
array, expected,
case(Array::single("abc".to_string()), Array::single(true)),
case(Array::single("ab1".to_string()), Array::single(true)),
case(Array::single(" ".to_string()), Array::single(false)),
case(Array::single("1".to_string()), Array::single(false)),
case(Array::single("123".to_string()), Array::single(false)),
case(Array::single("aBc".to_string()), Array::single(false)),
case(Array::single("ABC".to_string()), Array::single(false)),
case(Array::single("123.32".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("VII".to_string()), Array::single(false)),
case(Array::single("".to_string()), Array::single(false)),
)] fn test_char_is_lower(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_lower())
}

#[rstest(
array, expected,
case(Array::single("ABC".to_string()), Array::single(true)),
case(Array::single("VII".to_string()), Array::single(true)),
case(Array::single(" ".to_string()), Array::single(false)),
case(Array::single("1".to_string()), Array::single(false)),
case(Array::single("123".to_string()), Array::single(false)),
case(Array::single("abc".to_string()), Array::single(false)),
case(Array::single("aBc".to_string()), Array::single(false)),
case(Array::single("ab1".to_string()), Array::single(false)),
case(Array::single("123.32".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("123/435".to_string()), Array::single(false)),
case(Array::single("".to_string()), Array::single(false)),
)] fn test_char_is_upper(array: Result<Array<String>, ArrayError>, expected: Result<Array<bool>, ArrayError>) {
    assert_eq!(expected, array.is_upper())
}

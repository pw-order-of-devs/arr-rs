use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["".to_string(), "".to_string()]), Array::flat(vec!["abc".to_string(), "cde".to_string()])),
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::flat(vec!["abcdd".to_string(), "cdeff".to_string()])),
case(Array::new(vec!["abc".to_string(), "cde".to_string(), "abc".to_string(), "cde".to_string()], vec![2, 2]), Array::flat(vec!["dd".to_string(), "ff".to_string()]), Array::new(vec!["abcdd".to_string(), "cdeff".to_string(), "abcdd".to_string(), "cdeff".to_string()], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec!["dd".to_string(), "ff".to_string(), "ff".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_add(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.add(&other.unwrap()))
}

#[rstest(
array, counts, expected,
case(Array::flat(vec!["a".to_string(), "b".to_string()]), Array::flat(vec![1, 1]), Array::flat(vec!["a".to_string(), "b".to_string()])),
case(Array::flat(vec!["a".to_string(), "b".to_string()]), Array::flat(vec![3, 4]), Array::flat(vec!["aaa".to_string(), "bbbb".to_string()])),
case(Array::new(vec!["ab".to_string(), "cd".to_string(), "ab".to_string(), "cd".to_string()], vec![2, 2]), Array::flat(vec![2, 2]), Array::new(vec!["abab".to_string(), "cdcd".to_string(), "abab".to_string(), "cdcd".to_string()], vec![2, 2])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec![1, 1, 1]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_multiply(array: Result<Array<String>, ArrayError>, counts: Result<Array<usize>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.multiply(&counts.unwrap()))
}

#[rstest(
array, expected,
case(Array::flat(vec!["a".to_string(), "b".to_string()]), Array::flat(vec!["A".to_string(), "B".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), Array::flat(vec!["Aaa".to_string(), "Bbbb".to_string()])),
case(Array::new(vec!["ab".to_string(), "cd".to_string(), "ab".to_string(), "cd".to_string()], vec![2, 2]), Array::new(vec!["Ab".to_string(), "Cd".to_string(), "Ab".to_string(), "Cd".to_string()], vec![2, 2])),
)] fn test_char_capitalize(array: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.capitalize())
}

#[rstest(
array, expected,
case(Array::flat(vec!["a".to_string(), "b".to_string()]), Array::flat(vec!["a".to_string(), "b".to_string()])),
case(Array::flat(vec!["A".to_string(), "B".to_string()]), Array::flat(vec!["a".to_string(), "b".to_string()])),
case(Array::flat(vec!["AaA".to_string(), "bBbB".to_string()]), Array::flat(vec!["aaa".to_string(), "bbbb".to_string()])),
case(Array::new(vec!["aB21".to_string(), "Cd32".to_string(), "2aA3".to_string(), "c32d".to_string()], vec![2, 2]), Array::new(vec!["ab21".to_string(), "cd32".to_string(), "2aa3".to_string(), "c32d".to_string()], vec![2, 2])),
)] fn test_char_lower(array: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.lower())
}

#[rstest(
array, expected,
case(Array::flat(vec!["a".to_string(), "b".to_string()]), Array::flat(vec!["A".to_string(), "B".to_string()])),
case(Array::flat(vec!["A".to_string(), "B".to_string()]), Array::flat(vec!["A".to_string(), "B".to_string()])),
case(Array::flat(vec!["AaA".to_string(), "bBbB".to_string()]), Array::flat(vec!["AAA".to_string(), "BBBB".to_string()])),
case(Array::new(vec!["aB21".to_string(), "Cd32".to_string(), "2aA3".to_string(), "c32d".to_string()], vec![2, 2]), Array::new(vec!["AB21".to_string(), "CD32".to_string(), "2AA3".to_string(), "C32D".to_string()], vec![2, 2])),
)] fn test_char_upper(array: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.upper())
}

#[rstest(
array, expected,
case(Array::flat(vec!["a".to_string(), "B".to_string()]), Array::flat(vec!["A".to_string(), "b".to_string()])),
case(Array::flat(vec!["AaA".to_string(), "bBbB".to_string()]), Array::flat(vec!["aAa".to_string(), "BbBb".to_string()])),
case(Array::new(vec!["aB21".to_string(), "Cd32".to_string(), "2aA3".to_string(), "c32d".to_string()], vec![2, 2]), Array::new(vec!["Ab21".to_string(), "cD32".to_string(), "2Aa3".to_string(), "C32D".to_string()], vec![2, 2])),
)] fn test_char_swapcase(array: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.swapcase())
}

#[rstest(
array, width, fill_char, expected,
case(Array::flat(vec!["a".to_string(), "b".to_string()]), array_flat!(5).unwrap(), None, Array::flat(vec!["  a  ".to_string(), "  b  ".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(5).unwrap(), None, Array::flat(vec![" aaa ".to_string(), " bbbb".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(5, 6).unwrap(), None, Array::flat(vec![" aaa ".to_string(), " bbbb ".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(6).unwrap(), Some(Array::single('*').unwrap()), Array::flat(vec!["**aaa*".to_string(), "*bbbb*".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(6).unwrap(), Some(Array::flat(vec!['*', '#']).unwrap()), Array::flat(vec!["**aaa*".to_string(), "#bbbb#".to_string()])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec![1, 1, 1]).unwrap(), None, Err(ArrayError::BroadcastShapeMismatch)),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec![1, 1]).unwrap(), Some(Array::flat(vec!['*', '*', '*']).unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_center(array: Result<Array<String>, ArrayError>, width: Array<usize>, fill_char: Option<Array<char>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.center(&width, fill_char))
}

#[rstest(
array, sep, expected,
case(Array::single("aa".to_string()), Array::single("-".to_string()), Array::flat(vec!["a-a".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), Array::flat(vec!["-".to_string(), ".".to_string()]), Array::flat(vec!["a-a-a".to_string(), "b.b.b.b".to_string()])),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Array::flat(vec!["-".to_string(), ".".to_string(), "&".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_join(array: Result<Array<String>, ArrayError>, sep: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.join(&sep.unwrap()))
}

#[rstest(
array, sep, expected,
case(Array::single("a-a-a".to_string()), Array::single("-".to_string()), Array::single(Tuple3("a".to_string(), "-".to_string(), "a-a".to_string()))),
case(Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]), Array::single("-".to_string()), Array::flat(vec![Tuple3("a".to_string(), "-".to_string(), "a-a".to_string()), Tuple3("b".to_string(), "-".to_string(), "b-b-b".to_string())])),
case(Array::flat(vec!["a-a-a".to_string(), "b.b.b.b".to_string()]), Array::flat(vec!["-".to_string(), ".".to_string()]), Array::flat(vec![Tuple3("a".to_string(), "-".to_string(), "a-a".to_string()), Tuple3("b".to_string(), ".".to_string(), "b.b.b".to_string())])),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Array::flat(vec!["-".to_string(), ".".to_string(), "&".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_partition(array: Result<Array<String>, ArrayError>, sep: Result<Array<String>, ArrayError>, expected: Result<Array<Tuple3<String>>, ArrayError>) {
    assert_eq!(expected, array.partition(&sep.unwrap()))
}

#[rstest(
array, sep, expected,
case(Array::single("a-a-a".to_string()), Array::single("-".to_string()), Array::single(Tuple3("a-a".to_string(), "-".to_string(), "a".to_string()))),
case(Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]), Array::single("-".to_string()), Array::flat(vec![Tuple3("a-a".to_string(), "-".to_string(), "a".to_string()), Tuple3("b-b-b".to_string(), "-".to_string(), "b".to_string())])),
case(Array::flat(vec!["a-a-a".to_string(), "b.b.b.b".to_string()]), Array::flat(vec!["-".to_string(), ".".to_string()]), Array::flat(vec![Tuple3("a-a".to_string(), "-".to_string(), "a".to_string()), Tuple3("b.b.b".to_string(), ".".to_string(), "b".to_string())])),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Array::flat(vec!["-".to_string(), ".".to_string(), "&".to_string()]), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rpartition(array: Result<Array<String>, ArrayError>, sep: Result<Array<String>, ArrayError>, expected: Result<Array<Tuple3<String>>, ArrayError>) {
    assert_eq!(expected, array.rpartition(&sep.unwrap()))
}

#[rstest(
array, sep, max_split, expected,
case(Array::single("a a a".to_string()), None, None, Array::single(List(vec!["a".to_string(), "a".to_string(), "a".to_string()]))),
case(Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]), Some(Array::single("-".to_string()).unwrap()), None, Array::flat(vec![List(vec!["a".to_string(), "a".to_string(), "a".to_string()]), List(vec!["b".to_string(), "b".to_string(), "b".to_string(), "b".to_string()])])),
case(Array::flat(vec!["a-a-a".to_string(), "b.b.b.b".to_string()]), Some(Array::flat(vec!["-".to_string(), ".".to_string()]).unwrap()), None, Array::flat(vec![List(vec!["a".to_string(), "a".to_string(), "a".to_string()]), List(vec!["b".to_string(), "b".to_string(), "b".to_string(), "b".to_string()])])),
case(Array::flat(vec!["a-a-a".to_string(), "b.b.b.b".to_string()]), Some(Array::flat(vec!["-".to_string(), ".".to_string()]).unwrap()), Some(Array::single(2).unwrap()), Array::flat(vec![List(vec!["a".to_string(), "a-a".to_string()]), List(vec!["b".to_string(), "b.b.b".to_string()])])),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Some(Array::flat(vec!["-".to_string(), ".".to_string(), "&".to_string()]).unwrap()), None, Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_split(array: Result<Array<String>, ArrayError>, sep: Option<Array<String>>, max_split: Option<Array<usize>>, expected: Result<Array<List<String>>, ArrayError>) {
    assert_eq!(expected, ArrayString::split(&array, sep, max_split))
}

#[rstest(
array, sep, max_split, expected,
case(Array::single("a a a".to_string()), None, None, Array::single(List(vec!["a".to_string(), "a".to_string(), "a".to_string()]))),
case(Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]), Some(Array::single("-".to_string()).unwrap()), None, Array::flat(vec![List(vec!["a".to_string(), "a".to_string(), "a".to_string()]), List(vec!["b".to_string(), "b".to_string(), "b".to_string(), "b".to_string()])])),
case(Array::flat(vec!["a-a-a".to_string(), "b.b.b.b".to_string()]), Some(Array::flat(vec!["-".to_string(), ".".to_string()]).unwrap()), None, Array::flat(vec![List(vec!["a".to_string(), "a".to_string(), "a".to_string()]), List(vec!["b".to_string(), "b".to_string(), "b".to_string(), "b".to_string()])])),
case(Array::flat(vec!["a-a-a".to_string(), "b.b.b.b".to_string()]), Some(Array::flat(vec!["-".to_string(), ".".to_string()]).unwrap()), Some(Array::single(2).unwrap()), Array::flat(vec![List(vec!["a-a".to_string(), "a".to_string()]), List(vec!["b.b.b".to_string(), "b".to_string()])])),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Some(Array::flat(vec!["-".to_string(), ".".to_string(), "&".to_string()]).unwrap()), None, Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rsplit(array: Result<Array<String>, ArrayError>, sep: Option<Array<String>>, max_split: Option<Array<usize>>, expected: Result<Array<List<String>>, ArrayError>) {
    assert_eq!(expected, ArrayString::rsplit(&array, sep, max_split))
}

#[rstest(
array, keep_ends, expected,
case(Array::single("a\na\ra\r\na".to_string()), None, Array::single(List(vec!["a".to_string(), "a".to_string(), "a".to_string(), "a".to_string()]))),
case(Array::single("a\na\ra\r\na".to_string()), Some(Array::single(true).unwrap()), Array::single(List(vec!["a\n".to_string(), "a\r".to_string(), "a\r\n".to_string(), "a".to_string()]))),
case(Array::flat(vec!["a\na".to_string(), "b\nb".to_string()]), Some(Array::single(true).unwrap()), Array::flat(vec![List(vec!["a\n".to_string(), "a".to_string()]), List(vec!["b\n".to_string(), "b".to_string()])])),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Some(Array::flat(vec![false, false, false]).unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_splitlines(array: Result<Array<String>, ArrayError>, keep_ends: Option<Array<bool>>, expected: Result<Array<List<String>>, ArrayError>) {
    assert_eq!(expected, ArrayString::splitlines(&array.unwrap(), keep_ends))
}

#[rstest(
array, old, new, count, expected,
case(Array::single("abba".to_string()), Array::single("bb".to_string()), Array::single("cc".to_string()), None, Array::single("acca".to_string())),
case(Array::single("abbddbba".to_string()), Array::single("bb".to_string()), Array::single("cc".to_string()), Some(1), Array::single("accddbba".to_string())),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Array::flat(vec!["aa".to_string(), "bb".to_string()]), Array::flat(vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]), None, Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_replace(array: Result<Array<String>, ArrayError>, old: Result<Array<String>, ArrayError>, new: Result<Array<String>, ArrayError>, count: Option<usize>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, ArrayString::replace(&array, &old.unwrap(), &new.unwrap(), count))
}

#[rstest(
array, chars, expected,
case(Array::single(" abcba ".to_string()), None, Array::single("abcba".to_string())),
case(Array::single("abcba".to_string()), Some(Array::single("ab".to_string()).unwrap()), Array::single("c".to_string())),
case(Array::flat(vec!["abcba".to_string()]), Some(Array::flat(vec!["ab".to_string(), "a".to_string()]).unwrap()), Array::flat(vec!["c".to_string(), "bcb".to_string()])),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Some(Array::flat(vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]).unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_strip(array: Result<Array<String>, ArrayError>, chars: Option<Array<String>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.strip(chars))
}

#[rstest(
array, chars, expected,
case(Array::single(" abcba ".to_string()), None, Array::single("abcba ".to_string())),
case(Array::single("abcba".to_string()), Some(Array::single("ab".to_string()).unwrap()), Array::single("cba".to_string())),
case(Array::flat(vec!["abcba".to_string()]), Some(Array::flat(vec!["ab".to_string(), "a".to_string()]).unwrap()), Array::flat(vec!["cba".to_string(), "bcba".to_string()])),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Some(Array::flat(vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]).unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_lstrip(array: Result<Array<String>, ArrayError>, chars: Option<Array<String>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.lstrip(chars))
}

#[rstest(
array, chars, expected,
case(Array::single(" abcba ".to_string()), None, Array::single(" abcba".to_string())),
case(Array::single("abcba".to_string()), Some(Array::single("ab".to_string()).unwrap()), Array::single("abc".to_string())),
case(Array::flat(vec!["abcba".to_string()]), Some(Array::flat(vec!["ab".to_string(), "a".to_string()]).unwrap()), Array::flat(vec!["abc".to_string(), "abcb".to_string()])),
case(Array::flat(vec!["aa".to_string(), "bb".to_string()]), Some(Array::flat(vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]).unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rstrip(array: Result<Array<String>, ArrayError>, chars: Option<Array<String>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.rstrip(chars))
}

#[rstest(
array, width, fill_char, expected,
case(Array::flat(vec!["a".to_string(), "b".to_string()]), array_flat!(5).unwrap(), None, Array::flat(vec!["a    ".to_string(), "b    ".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(5).unwrap(), None, Array::flat(vec!["aaa  ".to_string(), "bbbb ".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(5, 6).unwrap(), None, Array::flat(vec!["aaa  ".to_string(), "bbbb  ".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(6).unwrap(), Some(Array::single('*').unwrap()), Array::flat(vec!["aaa***".to_string(), "bbbb**".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(6).unwrap(), Some(Array::flat(vec!['*', '#']).unwrap()), Array::flat(vec!["aaa***".to_string(), "bbbb##".to_string()])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec![1, 1, 1]).unwrap(), None, Err(ArrayError::BroadcastShapeMismatch)),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec![1, 1]).unwrap(), Some(Array::flat(vec!['*', '*', '*']).unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_ljust(array: Result<Array<String>, ArrayError>, width: Array<usize>, fill_char: Option<Array<char>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.ljust(&width, fill_char))
}

#[rstest(
array, width, fill_char, expected,
case(Array::flat(vec!["a".to_string(), "b".to_string()]), array_flat!(5).unwrap(), None, Array::flat(vec!["    a".to_string(), "    b".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(5).unwrap(), None, Array::flat(vec!["  aaa".to_string(), " bbbb".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(5, 6).unwrap(), None, Array::flat(vec!["  aaa".to_string(), "  bbbb".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(6).unwrap(), Some(Array::single('*').unwrap()), Array::flat(vec!["***aaa".to_string(), "**bbbb".to_string()])),
case(Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]), array_flat!(6).unwrap(), Some(Array::flat(vec!['*', '#']).unwrap()), Array::flat(vec!["***aaa".to_string(), "##bbbb".to_string()])),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec![1, 1, 1]).unwrap(), None, Err(ArrayError::BroadcastShapeMismatch)),
case(Array::flat(vec!["a".to_string(), "a".to_string()]), Array::flat(vec![1, 1]).unwrap(), Some(Array::flat(vec!['*', '*', '*']).unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rjust(array: Result<Array<String>, ArrayError>, width: Array<usize>, fill_char: Option<Array<char>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.rjust(&width, fill_char))
}

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

#[rstest(
array, expected,
case(Array::flat(vec!["abc".to_string(), "cde".to_string()]), Array::flat(vec![3, 3])),
case(Array::new(vec!["gd".to_string(), "gde".to_string(), "abc".to_string(), "ff".to_string()], vec![2, 2]), Array::new(vec![2, 3, 3, 2], vec![2, 2])),
)] fn test_char_str_len(array: Result<Array<String>, ArrayError>, expected: Result<Array<usize>, ArrayError>) {
    assert_eq!(expected, array.str_len())
}

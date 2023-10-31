use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, other, expected,
case(array_flat!(String, "abc", "cde"), array_flat!(String, "", ""), array_flat!(String, "abc", "cde")),
case(array_flat!(String, "abc", "cde"), array_flat!(String, "dd", "ff"), array_flat!(String, "abcdd", "cdeff")),
case(array!(String, [["abc", "cde"], ["abc", "cde"]]), array_flat!(String, "dd", "ff"), array!(String, [["abcdd", "cdeff"], ["abcdd", "cdeff"]])),
case(array_flat!(String, "a", "a"), array_flat!(String, "dd", "ff", "ff"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_add(array: Result<Array<String>, ArrayError>, other: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.add(&other.unwrap()))
}

#[rstest(
array, counts, expected,
case(array_flat!(String, "a", "b"), array_flat!(usize, 1, 1), array_flat!(String, "a", "b")),
case(array_flat!(String, "a", "b"), array_flat!(usize, 3, 4), array_flat!(String, "aaa", "bbbb")),
case(array!(String, [["ab", "cd"], ["ab", "cd"]]), array_flat!(usize, 2, 2), array!(String, [["abab", "cdcd"], ["abab", "cdcd"]])),
case(array_flat!(String, "a", "a"), array_flat!(usize, 1, 1, 1), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_multiply(array: Result<Array<String>, ArrayError>, counts: Result<Array<usize>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.multiply(&counts.unwrap()))
}

#[rstest(
array, expected,
case(array_flat!(String, "a", "b"), array_flat!(String, "A", "B")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(String, "Aaa", "Bbbb")),
case(array!(String, [["ab", "cd"], ["ab", "cd"]]), array!(String, [["Ab", "Cd"], ["Ab", "Cd"]])),
)] fn test_char_capitalize(array: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.capitalize())
}

#[rstest(
array, expected,
case(array_flat!(String, "a", "b"), array_flat!(String, "a", "b")),
case(array_flat!(String, "A", "B"), array_flat!(String, "a", "b")),
case(array_flat!(String, "AaA", "bBbB"), array_flat!(String, "aaa", "bbbb")),
case(array!(String, [["aB21", "Cd32"], ["2aA3", "c32d"]]), array!(String, [["ab21", "cd32"], ["2aa3", "c32d"]])),
)] fn test_char_lower(array: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.lower())
}

#[rstest(
array, expected,
case(array_flat!(String, "a", "b"), array_flat!(String, "A", "B")),
case(array_flat!(String, "A", "B"), array_flat!(String, "A", "B")),
case(array_flat!(String, "AaA", "bBbB"), array_flat!(String, "AAA", "BBBB")),
case(array!(String, [["aB21", "Cd32"], ["2aA3", "c32d"]]), array!(String, [["AB21", "CD32"], ["2AA3", "C32D"]])),
)] fn test_char_upper(array: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.upper())
}

#[rstest(
array, expected,
case(array_flat!(String, "a", "B"), array_flat!(String, "A", "b")),
case(array_flat!(String, "AaA", "bBbB"), array_flat!(String, "aAa", "BbBb")),
case(array!(String, [["aB21", "Cd32"], ["2aA3", "c32d"]]), array!(String, [["Ab21", "cD32"], ["2Aa3", "C32D"]])),
)] fn test_char_swapcase(array: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.swapcase())
}

#[rstest(
array, width, fill_char, expected,
case(array_flat!(String, "a", "b"), array_flat!(usize, 5).unwrap(), None, array_flat!(String, "  a  ", "  b  ")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 5).unwrap(), None, array_flat!(String, " aaa ", " bbbb")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 5, 6).unwrap(), None, array_flat!(String, " aaa ", " bbbb ")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 6).unwrap(), Some(array_single!(char, '*').unwrap()), array_flat!(String, "**aaa*", "*bbbb*")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 6).unwrap(), Some(array_flat!(char, '*', '#').unwrap()), array_flat!(String, "**aaa*", "#bbbb#")),
case(array_flat!(String, "a", "a"), array_flat!(usize, 1, 1, 1).unwrap(), None, Err(ArrayError::BroadcastShapeMismatch)),
case(array_flat!(String, "a", "a"), array_flat!(usize, 1, 1).unwrap(), Some(array_flat!(char, '*', '*', '*').unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_center(array: Result<Array<String>, ArrayError>, width: Array<usize>, fill_char: Option<Array<char>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.center(&width, fill_char))
}

#[rstest(
array, sep, expected,
case(array_single!(String, "aa"), array_single!(String, "-"), array_flat!(String, "a-a")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(String, "-", "."), array_flat!(String, "a-a-a", "b.b.b.b")),
case(array_flat!(String, "aa", "bb"), array_flat!(String, "-", ".", "&"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_join(array: Result<Array<String>, ArrayError>, sep: Result<Array<String>, ArrayError>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.join(&sep.unwrap()))
}

#[rstest(
array, sep, expected,
case(array_single!(String, "a-a-a"), array_single!(String, "-"), array_single!(Tuple3<String, String, String>, ("a", "-", "a-a"))),
case(array_flat!(String, "a-a-a", "b-b-b-b"), array_single!(String, "-"), array_flat!(Tuple3<String, String, String>, ("a", "-", "a-a"), ("b", "-", "b-b-b"))),
case(array_flat!(String, "a-a-a", "b.b.b.b"), array_flat!(String, "-", "."), array_flat!(Tuple3<String, String, String>, ("a", "-", "a-a"), ("b", ".", "b.b.b"))),
case(array_flat!(String, "aa", "bb"), array_flat!(String, "-", ".", "&"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_partition(array: Result<Array<String>, ArrayError>, sep: Result<Array<String>, ArrayError>, expected: Result<Array<Tuple3<String, String, String>>, ArrayError>) {
    assert_eq!(expected, ArrayStringManipulate::partition(&array.unwrap(), &sep.unwrap()))
}

#[rstest(
array, sep, expected,
case(array_single!(String, "a-a-a"), array_single!(String, "-"), array_single!(Tuple3<String, String, String>, ("a-a", "-", "a"))),
case(array_flat!(String, "a-a-a", "b-b-b-b"), array_single!(String, "-"), array_flat!(Tuple3<String, String, String>, ("a-a", "-", "a"), ("b-b-b", "-", "b"))),
case(array_flat!(String, "a-a-a", "b.b.b.b"), array_flat!(String, "-", "."), array_flat!(Tuple3<String, String, String>, ("a-a", "-", "a"), ("b.b.b", ".", "b"))),
case(array_flat!(String, "aa", "bb"), array_flat!(String, "-", ".", "&"), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rpartition(array: Result<Array<String>, ArrayError>, sep: Result<Array<String>, ArrayError>, expected: Result<Array<Tuple3<String, String, String>>, ArrayError>) {
    assert_eq!(expected, array.rpartition(&sep.unwrap()))
}

#[rstest(
array, sep, max_split, expected,
case(array_single!(String, "a a a"), None, None, array_single!(List<String>, vec!["a", "a", "a"])),
case(array_single!(String, "a a a"), None, Some(array_single!(usize, 3).unwrap()), array_single!(List<String>, vec!["a", "a", "a"])),
case(array_flat!(String, "a-a-a", "b-b-b-b"), Some(array_single!(String, "-").unwrap()), None, array_flat!(List<String>, vec!["a", "a", "a"], vec!["b", "b", "b", "b"])),
case(array_flat!(String, "a-a-a", "b.b.b.b"), Some(array_flat!(String, "-", ".").unwrap()), None, array_flat!(List<String>, vec!["a", "a", "a"], vec!["b", "b", "b", "b"])),
case(array_flat!(String, "a-a-a", "b.b.b.b"), Some(array_flat!(String, "-", ".").unwrap()), Some(array_single!(usize, 2).unwrap()), array_flat!(List<String>, vec!["a", "a-a"], vec!["b", "b.b.b"])),
case(array_flat!(String, "aa", "bb"), Some(array_flat!(String, "-", ".", "&").unwrap()), None, Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_split(array: Result<Array<String>, ArrayError>, sep: Option<Array<String>>, max_split: Option<Array<usize>>, expected: Result<Array<List<String>>, ArrayError>) {
    assert_eq!(expected, ArrayStringManipulate::split(&array, sep, max_split))
}

#[rstest(
array, sep, max_split, expected,
case(array_single!(String, "a a a"), None, None, array_single!(List<String>, vec!["a", "a", "a"])),
case(array_flat!(String, "a-a-a", "b-b-b-b"), Some(array_single!(String, "-").unwrap()), None, array_flat!(List<String>, vec!["a", "a", "a"], vec!["b", "b", "b", "b"])),
case(array_flat!(String, "a-a-a", "b.b.b.b"), Some(array_flat!(String, "-", ".").unwrap()), None, array_flat!(List<String>, vec!["a", "a", "a"], vec!["b", "b", "b", "b"])),
case(array_flat!(String, "a-a-a", "b.b.b.b"), Some(array_flat!(String, "-", ".").unwrap()), Some(array_single!(usize, 2).unwrap()), array_flat!(List<String>, vec!["a-a", "a"], vec!["b.b.b", "b"])),
case(array_flat!(String, "aa", "bb"), Some(array_flat!(String, "-", ".", "&").unwrap()), None, Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rsplit(array: Result<Array<String>, ArrayError>, sep: Option<Array<String>>, max_split: Option<Array<usize>>, expected: Result<Array<List<String>>, ArrayError>) {
    assert_eq!(expected, ArrayStringManipulate::rsplit(&array, sep, max_split))
}

#[rstest(
array, keep_ends, expected,
case(array_single!(String, "a\na\ra\r\na"), None, array_single!(List<String>, vec!["a", "a", "a", "a"])),
case(array_single!(String, "a\na\ra\r\na"), Some(array_single!(bool, true).unwrap()), array_single!(List<String>, vec!["a\n", "a\r", "a\r\n", "a"])),
case(array_flat!(String, "a\na", "b\nb"), Some(array_single!(bool, true).unwrap()), array_flat!(List<String>, vec!["a\n", "a"], vec!["b\n", "b"])),
case(array_flat!(String, "aa", "bb"), Some(array_flat!(bool, false, false, false).unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_splitlines(array: Result<Array<String>, ArrayError>, keep_ends: Option<Array<bool>>, expected: Result<Array<List<String>>, ArrayError>) {
    assert_eq!(expected, ArrayStringManipulate::splitlines(&array.unwrap(), keep_ends))
}

#[rstest(
array, old, new, count, expected,
case(array_single!(String, "abba"), array_single!(String, "bb"), array_single!(String, "cc"), None, array_single!(String, "acca")),
case(array_single!(String, "abbddbba"), array_single!(String, "bb"), array_single!(String, "cc"), Some(1), array_single!(String, "accddbba")),
case(array_flat!(String, "aa", "bb"), array_flat!(String, "aa", "bb"), array_flat!(String, "aa", "bb", "cc"), None, Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_replace(array: Result<Array<String>, ArrayError>, old: Result<Array<String>, ArrayError>, new: Result<Array<String>, ArrayError>, count: Option<usize>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, ArrayStringManipulate::replace(&array, &old.unwrap(), &new.unwrap(), count))
}

#[rstest(
array, chars, expected,
case(array_single!(String, " abcba "), None, array_single!(String, "abcba")),
case(array_single!(String, "abcba"), Some(array_single!(String, "ab").unwrap()), array_single!(String, "c")),
case(array_flat!(String, "abcba"), Some(array_flat!(String, "ab", "a").unwrap()), array_flat!(String, "c", "bcb")),
case(array_flat!(String, "aa", "bb"), Some(array_flat!(String, "aa", "bb", "cc").unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_strip(array: Result<Array<String>, ArrayError>, chars: Option<Array<String>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.strip(chars))
}

#[rstest(
array, chars, expected,
case(array_single!(String, " abcba "), None, array_single!(String, "abcba ")),
case(array_single!(String, "abcba"), Some(array_single!(String, "ab").unwrap()), array_single!(String, "cba")),
case(array_flat!(String, "abcba"), Some(array_flat!(String, "ab", "a").unwrap()), array_flat!(String, "cba", "bcba")),
case(array_flat!(String, "aa", "bb"), Some(array_flat!(String, "aa", "bb", "cc").unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_lstrip(array: Result<Array<String>, ArrayError>, chars: Option<Array<String>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.lstrip(chars))
}

#[rstest(
array, chars, expected,
case(array_single!(String, " abcba "), None, array_single!(String, " abcba")),
case(array_single!(String, "abcba"), Some(array_single!(String, "ab").unwrap()), array_single!(String, "abc")),
case(array_flat!(String, "abcba"), Some(array_flat!(String, "ab", "a").unwrap()), array_flat!(String, "abc", "abcb")),
case(array_flat!(String, "aa", "bb"), Some(array_flat!(String, "aa", "bb", "cc").unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rstrip(array: Result<Array<String>, ArrayError>, chars: Option<Array<String>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.rstrip(chars))
}

#[rstest(
array, width, fill_char, expected,
case(array_flat!(String, "a", "b"), array_flat!(usize, 5).unwrap(), None, array_flat!(String, "a    ", "b    ")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 5).unwrap(), None, array_flat!(String, "aaa  ", "bbbb ")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 5, 6).unwrap(), None, array_flat!(String, "aaa  ", "bbbb  ")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 6).unwrap(), Some(array_single!(char, '*').unwrap()), array_flat!(String, "aaa***", "bbbb**")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 6).unwrap(), Some(array_flat!(char, '*', '#').unwrap()), array_flat!(String, "aaa***", "bbbb##")),
case(array_flat!(String, "a", "a"), array_flat!(usize, 1, 1, 1).unwrap(), None, Err(ArrayError::BroadcastShapeMismatch)),
case(array_flat!(String, "a", "a"), array_flat!(usize, 1, 1).unwrap(), Some(array_flat!(char, '*', '*', '*').unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_ljust(array: Result<Array<String>, ArrayError>, width: Array<usize>, fill_char: Option<Array<char>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.ljust(&width, fill_char))
}

#[rstest(
array, width, fill_char, expected,
case(array_flat!(String, "a", "b"), array_flat!(usize, 5).unwrap(), None, array_flat!(String, "    a", "    b")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 5).unwrap(), None, array_flat!(String, "  aaa", " bbbb")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 5, 6).unwrap(), None, array_flat!(String, "  aaa", "  bbbb")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 6).unwrap(), Some(array_single!(char, '*').unwrap()), array_flat!(String, "***aaa", "**bbbb")),
case(array_flat!(String, "aaa", "bbbb"), array_flat!(usize, 6).unwrap(), Some(array_flat!(char, '*', '#').unwrap()), array_flat!(String, "***aaa", "##bbbb")),
case(array_flat!(String, "a", "a"), array_flat!(usize, 1, 1, 1).unwrap(), None, Err(ArrayError::BroadcastShapeMismatch)),
case(array_flat!(String, "a", "a"), array_flat!(usize, 1, 1).unwrap(), Some(array_flat!(char, '*', '*', '*').unwrap()), Err(ArrayError::BroadcastShapeMismatch)),
)] fn test_char_rjust(array: Result<Array<String>, ArrayError>, width: Array<usize>, fill_char: Option<Array<char>>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.rjust(&width, fill_char))
}

#[rstest(
array, width, expected,
case(array_flat!(String, "5", "-5"), 4, array_flat!(String, "0005", "-005")),
case(array_flat!(String, "12345", "67890"), 3, array_flat!(String, "12345", "67890")),
case(array_flat!(String, "-12345", "-67890"), 3, array_flat!(String, "-12345", "-67890")),
case(array_flat!(String, "12345", "67890"), 10, array_flat!(String, "0000012345", "0000067890")),
case(array_flat!(String, "-12345", "-67890"), 10, array_flat!(String, "-000012345", "-000067890")),
)] fn test_char_zfill(array: Result<Array<String>, ArrayError>, width: usize, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.zfill(width))
}

#[rstest(
case(array_flat!(String, "hello", "world"), vec![('l', 'w'), ('o', 'd')], array_flat!(String, "hewwd", "wdrwd")),
case(array_flat!(String, "abc", "def"), vec![('a', 'x'), ('b', 'y'), ('c', 'z')], array_flat!(String, "xyz", "def")),
case(array_flat!(String, "apple", "banana"), vec![('a', 'A'), ('e', 'E'), ('i', 'I'), ('o', 'O'), ('u', 'U')], array_flat!(String, "ApplE", "bAnAnA")),
case(array_flat!(String, "12345", "67890"), vec![('1', 'a'), ('3', 'b'), ('5', 'c'), ('7', 'd'), ('9', 'e')], array_flat!(String, "a2b4c", "6d8e0")),
array, table, expected,
)] fn test_char_translate(array: Result<Array<String>, ArrayError>, table: Vec<(char, char)>, expected: Result<Array<String>, ArrayError>) {
    assert_eq!(expected, array.translate(table))
}

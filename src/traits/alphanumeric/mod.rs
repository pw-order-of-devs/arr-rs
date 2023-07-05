use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::{
        alphanumeric::Alphanumeric,
        collection::List,
        tuple::tuple3::Tuple3,
    },
};

/// ArrayTrait - Alphanumeric Array operations
pub trait ArrayString<N: Alphanumeric> where Self: Sized + Clone {

    /// Return element-wise string concatenation for two arrays of String
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["abcdd".to_string(), "cdeff".to_string()]);
    /// let arr = Array::flat(vec!["abc".to_string(), "cde".to_string()]);
    /// let other = Array::flat(vec!["dd".to_string(), "ff".to_string()]).unwrap();
    /// assert_eq!(expected, arr.add(&other));
    /// ```
    fn add(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Return (a * i), that is string multiple concatenation, element-wise
    ///
    /// # Arguments
    ///
    /// * `counts` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["aaa".to_string(), "bbbbb".to_string()]);
    /// let arr = Array::flat(vec!["a".to_string(), "b".to_string()]);
    /// let counts = Array::flat(vec![3, 5]).unwrap();
    /// assert_eq!(expected, arr.multiply(&counts));
    /// ```
    fn multiply(&self, counts: &Array<usize>) -> Result<Array<N>, ArrayError>;

    /// Capitalizes first character of each element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["A1a".to_string(), "2bb".to_string()]);
    /// let arr = Array::flat(vec!["a1a".to_string(), "2bb".to_string()]);
    /// assert_eq!(expected, arr.capitalize());
    /// ```
    fn capitalize(&self) -> Result<Array<N>, ArrayError>;

    /// Turns characters to lower-case
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["a1a".to_string(), "2bb".to_string()]);
    /// let arr = Array::flat(vec!["A1a".to_string(), "2bB".to_string()]);
    /// assert_eq!(expected, arr.lower());
    /// ```
    fn lower(&self) -> Result<Array<N>, ArrayError>;

    /// Turns characters to upper-case
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["A1A".to_string(), "2BB".to_string()]);
    /// let arr = Array::flat(vec!["a1a".to_string(), "2bb".to_string()]);
    /// assert_eq!(expected, arr.upper());
    /// ```
    fn upper(&self) -> Result<Array<N>, ArrayError>;

    /// Swap characters case
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["A1a".to_string(), "2bB".to_string()]);
    /// let arr = Array::flat(vec!["a1A".to_string(), "2Bb".to_string()]);
    /// assert_eq!(expected, arr.swapcase());
    /// ```
    fn swapcase(&self) -> Result<Array<N>, ArrayError>;

    /// Centers elements in a string of length of `width`
    ///
    /// # Arguments
    ///
    /// * `width` - length of the resulting strings
    /// * `fill_char` - padding character to use. defaults to space
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["***aaa***".to_string(), "***bbbb**".to_string()]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]);
    /// assert_eq!(expected, arr.center(9, Some('*')));
    /// ```
    fn center(&self, width: usize, fill_char: Option<char>) -> Result<Array<N>, ArrayError>;

    /// Concatenate the strings in the sequence
    ///
    /// # Arguments
    ///
    /// * `sep` - array of separators to concatenate strings with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["a-a-a".to_string(), "b.b.b.b".to_string()]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]);
    /// let sep = Array::flat(vec!["-".to_string(), ".".to_string()]).unwrap();
    /// assert_eq!(expected, arr.join(&sep));
    /// ```
    fn join(&self, sep: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Partition each element around first occurrence of sep
    ///
    /// # Arguments
    ///
    /// * `sep` - separator to split each string element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![Tuple3("a".to_string(), "-".to_string(), "a-a".to_string()), Tuple3("b".to_string(), "-".to_string(), "b-b-b".to_string())]);
    /// let arr = Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]);
    /// assert_eq!(expected, arr.partition(&Array::single("-".to_string()).unwrap()));
    /// ```
    fn partition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N>>, ArrayError>;

    /// Partition each element around last occurrence of sep
    ///
    /// # Arguments
    ///
    /// * `sep` - separator to split each string element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![Tuple3("a-a".to_string(), "-".to_string(), "a".to_string()), Tuple3("b-b-b".to_string(), "-".to_string(), "b".to_string())]);
    /// let arr = Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]);
    /// assert_eq!(expected, arr.rpartition(&Array::single("-".to_string()).unwrap()));
    /// ```
    fn rpartition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N>>, ArrayError>;

    /// Returns a list of the words in the string, using sep as the delimiter string.
    ///
    /// # Arguments
    ///
    /// * `sep` - separator to split each string element. defaults to space
    /// * `max_split` - at most <max_split> splits are done
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![List(vec!["a".to_string(), "a".to_string(), "a".to_string()]), List(vec!["b".to_string(), "b".to_string(), "b".to_string(), "b".to_string()])]);
    /// let arr = Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]).unwrap();
    /// assert_eq!(expected, ArrayString::split(&arr, Some(Array::single("-".to_string()).unwrap()), None));
    /// ```
    fn split(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError>;

    /// Returns a list of the words in the string, using sep as the delimiter string.
    ///
    /// # Arguments
    ///
    /// * `sep` - separator to split each string element. defaults to space
    /// * `max_split` - at most <max_split> splits are done
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![List(vec!["a".to_string(), "a".to_string(), "a".to_string()]), List(vec!["b".to_string(), "b".to_string(), "b".to_string(), "b".to_string()])]);
    /// let arr = Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]).unwrap();
    /// assert_eq!(expected, ArrayString::rsplit(&arr, Some(Array::single("-".to_string()).unwrap()), None));
    /// ```
    fn rsplit(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError>;

    /// Returns a list of the words in the string, using line break character as the delimiter string.
    ///
    /// # Arguments
    ///
    /// * `keep_ends` - if true, line break character will be kept
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![List(vec!["aa".to_string(), "a".to_string()]), List(vec!["bb".to_string(), "bb".to_string()])]);
    /// let arr = Array::flat(vec!["aa\na".to_string(), "bb\nbb".to_string()]).unwrap();
    /// assert_eq!(expected, arr.splitlines(None));
    /// ```
    fn splitlines(&self, keep_ends: Option<Array<bool>>) -> Result<Array<List<N>>, ArrayError>;

    /// Replaces all occurrences of <old> with <new>
    ///
    /// # Arguments
    ///
    /// * `old` - string to replace
    /// * `new` - new string
    /// * `count` - maximum occurrences to replace
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["new string".to_string()]);
    /// let arr = Array::flat(vec!["old string".to_string()]);
    /// let new = Array::flat(vec!["new".to_string()]).unwrap();
    /// let old = Array::flat(vec!["old".to_string()]).unwrap();
    /// assert_eq!(expected, arr.replace(&old, &new, None));
    /// ```
    fn replace(&self, old: &Array<N>, new: &Array<N>, count: Option<usize>) -> Result<Array<N>, ArrayError>;

    /// Trims leading and trailing characters
    ///
    /// # Arguments
    ///
    /// * `chars` - characters to trim. defaults to whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["b".to_string(), "bbbb".to_string()]);
    /// let arr = Array::flat(vec!["aaba".to_string(), "ccbbbbc".to_string()]);
    /// assert_eq!(expected, arr.strip(Some(Array::flat(vec!["a".to_string(), "c".to_string()]).unwrap())));
    /// ```
    fn strip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError>;

    /// Left-justifies elements in a string of length of `width`
    ///
    /// # Arguments
    ///
    /// * `width` - length of the resulting strings
    /// * `fill_char` - padding character to use. defaults to space
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["aaa******".to_string(), "bbbb*****".to_string()]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]);
    /// assert_eq!(expected, arr.ljust(9, Some('*')));
    /// ```
    fn ljust(&self, width: usize, fill_char: Option<char>) -> Result<Array<N>, ArrayError>;

    /// Trims leading characters
    ///
    /// # Arguments
    ///
    /// * `chars` - characters to trim. defaults to whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["ba".to_string(), "c".to_string()]);
    /// let arr = Array::flat(vec!["aaba".to_string(), "bbbbc".to_string()]);
    /// assert_eq!(expected, arr.lstrip(Some(Array::flat(vec!["a".to_string(), "b".to_string()]).unwrap())));
    /// ```
    fn lstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError>;

    /// Right-justifies elements in a string of length of `width`
    ///
    /// # Arguments
    ///
    /// * `width` - length of the resulting strings
    /// * `fill_char` - padding character to use. defaults to space
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["******aaa".to_string(), "*****bbbb".to_string()]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "bbbb".to_string()]);
    /// assert_eq!(expected, arr.rjust(9, Some('*')));
    /// ```
    fn rjust(&self, width: usize, fill_char: Option<char>) -> Result<Array<N>, ArrayError>;

    /// Trims trailing characters
    ///
    /// # Arguments
    ///
    /// * `chars` - characters to trim. defaults to whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["ba".to_string(), "c".to_string()]);
    /// let arr = Array::flat(vec!["aaba".to_string(), "bbbbc".to_string()]);
    /// assert_eq!(expected, arr.lstrip(Some(Array::flat(vec!["a".to_string(), "b".to_string()]).unwrap())));
    /// ```
    fn rstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError>;

    /// Return (self == other) element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, false]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "bbbxx".to_string()]);
    /// let other = Array::flat(vec!["aaa".to_string(), "bbbbb".to_string()]).unwrap();
    /// assert_eq!(expected, arr.equal(&other));
    /// ```
    fn equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError>;

    /// Return (self != other) element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![false, true]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "bbbxx".to_string()]);
    /// let other = Array::flat(vec!["aaa".to_string(), "bbbbb".to_string()]).unwrap();
    /// assert_eq!(expected, arr.not_equal(&other));
    /// ```
    fn not_equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError>;

    /// Return (self >= other) element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, false, true]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "aaa".to_string(), "bbbxx".to_string()]);
    /// let other = Array::flat(vec!["aaa".to_string(), "aba".to_string(), "bbbbb".to_string()]).unwrap();
    /// assert_eq!(expected, arr.greater_equal(&other));
    /// ```
    fn greater_equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError>;

    /// Return (self <= other) element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, true, false]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "aaa".to_string(), "bbbxx".to_string()]);
    /// let other = Array::flat(vec!["aaa".to_string(), "aba".to_string(), "bbbbb".to_string()]).unwrap();
    /// assert_eq!(expected, arr.less_equal(&other));
    /// ```
    fn less_equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError>;

    /// Return (self > other) element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![false, false, true]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "aaa".to_string(), "bbbxx".to_string()]);
    /// let other = Array::flat(vec!["aaa".to_string(), "aba".to_string(), "bbbbb".to_string()]).unwrap();
    /// assert_eq!(expected, arr.greater(&other));
    /// ```
    fn greater(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError>;

    /// Return (self < other) element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![false, true, false]);
    /// let arr = Array::flat(vec!["aaa".to_string(), "aaa".to_string(), "bbbxx".to_string()]);
    /// let other = Array::flat(vec!["aaa".to_string(), "aba".to_string(), "bbbbb".to_string()]).unwrap();
    /// assert_eq!(expected, arr.less(&other));
    /// ```
    fn less(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError>;

    /// Performs element-wise comparison of two string arrays using the comparison operator specified by cmp_op
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    /// * `cmp_op` - Type of comparison: {“<”, “<=”, “==”, “>=”, “>”, “!=”}
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec!["aaa".to_string(), "bbbxx".to_string()]);
    /// let other = Array::flat(vec!["aaa".to_string(), "bbbbb".to_string()]).unwrap();
    ///
    /// let expected = Array::flat(vec![true, false]);
    /// assert_eq!(expected, arr.compare(&other, "=="));
    ///
    /// let expected = Array::flat(vec![false, true]);
    /// assert_eq!(expected, arr.compare(&other, "!="));
    ///
    /// let expected = Array::flat(vec![true, true]);
    /// assert_eq!(expected, arr.compare(&other, ">="));
    ///
    /// let expected = Array::flat(vec![true, false]);
    /// assert_eq!(expected, arr.compare(&other, "<="));
    ///
    /// let expected = Array::flat(vec![false, true]);
    /// assert_eq!(expected, arr.compare(&other, ">"));
    ///
    /// let expected = Array::flat(vec![false, false]);
    /// assert_eq!(expected, arr.compare(&other, "<"));
    /// ```
    fn compare(&self, other: &Array<N>, cmp_op: &str) -> Result<Array<bool>, ArrayError>;

    /// Return string.len() element-wise
    ///
    /// # Arguments
    ///
    /// * `sub` - substring to search for
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![6, 9, 3]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string(), "aAaAaAacc".to_string(), "abc".to_string()]);
    /// assert_eq!(expected, arr.str_len());
    /// ```
    fn str_len(&self) -> Result<Array<usize>, ArrayError>;

    /// Returns an array with the number of non-overlapping occurrences of substring sub
    ///
    /// # Arguments
    ///
    /// * `sub` - substring to search for
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![3, 2, 1]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string(), "aAaAaA".to_string(), "bbAabb".to_string()]);
    /// assert_eq!(expected, arr.count("Aa"));
    ///
    /// let expected = Array::flat(vec![1]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string()]);
    /// assert_eq!(expected, arr.count("AaAa"));
    /// ```
    fn count(&self, sub: &str) -> Result<Array<usize>, ArrayError>;

    /// Checks if string element starts with prefix
    ///
    /// # Arguments
    ///
    /// * `prefix` - substring to search for
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, false, false]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string(), "aAaAaA".to_string(), "bbAabb".to_string()]);
    /// assert_eq!(expected, arr.starts_with("Aa"));
    /// ```
    fn starts_with(&self, prefix: &str) -> Result<Array<bool>, ArrayError>;

    /// Checks if string element ends with suffix
    ///
    /// # Arguments
    ///
    /// * `suffix` - substring to search for
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![false, true, false]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string(), "aAaAaA".to_string(), "bbAabb".to_string()]);
    /// assert_eq!(expected, arr.ends_with("aA"));
    /// ```
    fn ends_with(&self, suffix: &str) -> Result<Array<bool>, ArrayError>;

    /// For each element, return the lowest index in the string where substring sub is found
    ///
    /// # Arguments
    ///
    /// * `sub` - substring to search for
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![1, 0, -1]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string(), "aAaAaA".to_string(), "bbAabb".to_string()]);
    /// assert_eq!(expected, arr.find("aA"));
    /// ```
    fn find(&self, sub: &str) -> Result<Array<isize>, ArrayError>;

    /// For each element, return the highest index in the string where substring sub is found
    ///
    /// # Arguments
    ///
    /// * `sub` - substring to search for
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![3, 4, -1]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string(), "aAaAaA".to_string(), "bbAabb".to_string()]);
    /// assert_eq!(expected, arr.rfind("aA"));
    /// ```
    fn rfind(&self, sub: &str) -> Result<Array<isize>, ArrayError>;

    /// For each element, return the lowest index in the string where substring sub is found;
    /// alias on `find`
    ///
    /// # Arguments
    ///
    /// * `sub` - substring to search for
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![1, 0, -1]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string(), "aAaAaA".to_string(), "bbAabb".to_string()]);
    /// assert_eq!(expected, arr.index("aA"));
    /// ```
    fn index(&self, sub: &str) -> Result<Array<isize>, ArrayError>;

    /// For each element, return the highest index in the string where substring sub is found;
    /// alias on `rfind`
    ///
    /// # Arguments
    ///
    /// * `sub` - substring to search for
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![3, 4, -1]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string(), "aAaAaA".to_string(), "bbAabb".to_string()]);
    /// assert_eq!(expected, arr.rindex("aA"));
    /// ```
    fn rindex(&self, sub: &str) -> Result<Array<isize>, ArrayError>;
}

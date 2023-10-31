use crate::{
    alphanumeric::prelude::*,
    core::prelude::*,
    errors::prelude::*,
};

/// `ArrayTrait` - Alphanumeric Array operations
pub trait ArrayStringManipulate<N: Alphanumeric> where Self: Sized + Clone {

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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// assert_eq!(expected, arr.center(&Array::single(9).unwrap(), Some(Array::single('*').unwrap())));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn center(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError>;

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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// assert_eq!(expected, ArrayStringManipulate::partition(&arr, &Array::single("-".to_string()).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn partition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N, N, N>>, ArrayError>;

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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn rpartition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N, N, N>>, ArrayError>;

    /// Returns a list of the words in the string, using sep as the delimiter string.
    ///
    /// # Arguments
    ///
    /// * `sep` - separator to split each string element. defaults to space
    /// * `max_split` - at most <`max_split`> splits are done
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![List(vec!["a".to_string(), "a".to_string(), "a".to_string()]), List(vec!["b".to_string(), "b".to_string(), "b".to_string(), "b".to_string()])]);
    /// let arr = Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]).unwrap();
    /// assert_eq!(expected, ArrayStringManipulate::split(&arr, Some(Array::single("-".to_string()).unwrap()), None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn split(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError>;

    /// Returns a list of the words in the string, using sep as the delimiter string.
    ///
    /// # Arguments
    ///
    /// * `sep` - separator to split each string element. defaults to space
    /// * `max_split` - at most <`max_split`> splits are done
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![List(vec!["a".to_string(), "a".to_string(), "a".to_string()]), List(vec!["b".to_string(), "b".to_string(), "b".to_string(), "b".to_string()])]);
    /// let arr = Array::flat(vec!["a-a-a".to_string(), "b-b-b-b".to_string()]).unwrap();
    /// assert_eq!(expected, ArrayStringManipulate::rsplit(&arr, Some(Array::single("-".to_string()).unwrap()), None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn strip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError>;

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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn lstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError>;

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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn rstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError>;

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
    /// assert_eq!(expected, arr.ljust(&Array::single(9).unwrap(), Some(Array::single('*').unwrap())));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn ljust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError>;

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
    /// assert_eq!(expected, arr.rjust(&Array::single(9).unwrap(), Some(Array::single('*').unwrap())));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn rjust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError>;

    /// Return the numeric string left-filled with zeros
    ///
    /// # Arguments
    ///
    /// * `width` - length of the resulting strings
    /// 
    /// # Examples
    /// 
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["0005".to_string(), "-005".to_string()]);
    /// let arr = Array::flat(vec!["5".to_string(), "-5".to_string()]);
    /// assert_eq!(expected, arr.zfill(4));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn zfill(&self, width: usize) -> Result<Array<N>, ArrayError>;

    /// Remove elements from `delete_char` vec and translate string remaining characters through `table`
    ///
    /// # Arguments
    ///
    /// * `table` - chars to replace
    /// * `delete_chars` - chars to delete
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec!["hewwd".to_string(), "wdrwd".to_string()]);
    /// let arr = Array::flat(vec!["hello".to_string(), "world".to_string()]);
    /// assert_eq!(expected, arr.translate(vec![('l', 'w'), ('o', 'd')]));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn translate(&self, table: Vec<(char, char)>) -> Result<Array<N>, ArrayError>;
}

impl <N: Alphanumeric> ArrayStringManipulate<N> for Array<N> {

    fn add(&self, other: &Self) -> Result<Self, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._append(tuple.1))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn multiply(&self, counts: &Array<usize>) -> Result<Self, ArrayError> {
        let (array, counts) = self.broadcast_h2(counts)?;
        let elements = array.clone().into_iter().zip(counts)
            .map(|tuple| tuple.0._multiply(tuple.1))
            .collect();
        Self::new(elements, array.get_shape()?)
    }

    fn capitalize(&self) -> Result<Self, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s._capitalize())
            .collect();
        Self::new(elements, self.get_shape()?)
    }

    fn lower(&self) -> Result<Self, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s._lower())
            .collect();
        Self::new(elements, self.get_shape()?)
    }

    fn upper(&self) -> Result<Self, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s._upper())
            .collect();
        Self::new(elements, self.get_shape()?)
    }

    fn swapcase(&self) -> Result<Self, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s._swapcase())
            .collect();
        Self::new(elements, self.get_shape()?)
    }

    fn center(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Self, ArrayError> {
        let fill_char = fill_char.unwrap_or(Array::single(' ')?);
        let (array, width, fill_char) = self.broadcast_h3(width, &fill_char)?;

        let elements = array.into_iter().enumerate()
            .map(|(idx, s)| s._center(width[idx], fill_char[idx]))
            .collect();
        Self::new(elements, self.get_shape()?)
    }

    fn join(&self, sep: &Self) -> Result<Self, ArrayError> {
        let broadcasted = self.broadcast(sep)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._join(tuple.1))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn partition(&self, sep: &Self) -> Result<Array<Tuple3<N, N, N>>, ArrayError> {
        let broadcasted = self.broadcast(sep)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._partition(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn rpartition(&self, sep: &Self) -> Result<Array<Tuple3<N, N, N>>, ArrayError> {
        let broadcasted = self.broadcast(sep)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._rpartition(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn split(&self, sep: Option<Self>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        let sep = sep.unwrap_or(Self::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&sep)?;
        let max_split =
            if let Some(counts) = max_split { Some(counts.broadcast_to(broadcasted.get_shape()?)?) }
            else { None };
        let elements = broadcasted.clone().into_iter().enumerate()
            .map(|(idx, tuple)| tuple.0._split(tuple.1, max_split.clone().map(|s| s[idx])))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn rsplit(&self, sep: Option<Self>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        let sep = sep.unwrap_or(Self::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&sep)?;
        let max_split =
            if let Some(counts) = max_split { Some(counts.broadcast_to(broadcasted.get_shape()?)?) } else { None };
        let elements = broadcasted.clone().into_iter().enumerate()
            .map(|(idx, tuple)| tuple.0._rsplit(tuple.1, max_split.clone().map(|s| s[idx])))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn splitlines(&self, keep_ends: Option<Array<bool>>) -> Result<Array<List<N>>, ArrayError> {
        let keep_ends = keep_ends.unwrap_or(Array::single(false)?);
        let (array, keep_ends) = self.broadcast_h2(&keep_ends)?;
        let elements = array.clone().into_iter().enumerate()
            .map(|(idx, elem)| elem._splitlines(keep_ends[idx]))
            .collect();
        Array::new(elements, array.get_shape()?)
    }

    fn replace(&self, old: &Self, new: &Self, count: Option<usize>) -> Result<Self, ArrayError> {
        let broadcasted = Self::broadcast_arrays(vec![self.clone(), old.clone(), new.clone()])?;
        let tupled = (0..broadcasted[0].len()?).map(|i| Tuple3(
            broadcasted[0][i].clone(),
            broadcasted[1][i].clone(),
            broadcasted[2][i].clone()))
            .map(|tuple| tuple.0._replace(tuple.1, tuple.2, count))
            .collect();
        Self::new(tupled, broadcasted[0].get_shape()?)
    }

    fn strip(&self, chars: Option<Self>) -> Result<Self, ArrayError> {
        self.lstrip(chars.clone()).rstrip(chars)
    }

    fn lstrip(&self, chars: Option<Self>) -> Result<Self, ArrayError> {
        let chars = chars.unwrap_or(Self::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&chars)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._lstrip(tuple.1))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn rstrip(&self, chars: Option<Self>) -> Result<Self, ArrayError> {
        let chars = chars.unwrap_or(Self::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&chars)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._rstrip(tuple.1))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn ljust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Self, ArrayError> {
        let fill_char = fill_char.unwrap_or(Array::single(' ')?);
        let (array, width, fill_char) = self.broadcast_h3(width, &fill_char)?;

        let elements = array.into_iter().enumerate()
            .map(|(idx, s)| s._ljust(width[idx], fill_char[idx]))
            .collect();
        Self::new(elements, self.get_shape()?)
    }

    fn rjust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Self, ArrayError> {
        let fill_char = fill_char.unwrap_or(Array::single(' ')?);
        let tmp_fill_char = Self::single(N::from_str(" ")).broadcast_to(fill_char.get_shape()?)?;
        let tmp_width = Self::single(N::from_str(" ")).broadcast_to(width.get_shape()?)?;
        let broadcasted = Self::broadcast_arrays(vec![self.clone(), tmp_width, tmp_fill_char])?;

        let array = broadcasted[0].clone();
        let width = width.broadcast_to(array.get_shape()?)?;
        let fill_char = fill_char.broadcast_to(array.get_shape()?)?;

        let elements = array.into_iter().enumerate()
            .map(|(idx, s)| s._rjust(width[idx], fill_char[idx]))
            .collect();
        Self::new(elements, self.get_shape()?)
    }

    fn zfill(&self, width: usize) -> Result<Self, ArrayError> {
        if self.get_elements()?.iter().any(|item| item.to_string().parse::<f64>().is_err()) {
            return Err(ArrayError::ParameterError { param: "`array`", message: "must contain numeric strings only" })
        }
        self.map(|item| {
            let (mut prefix, mut str) = ("", item.to_string());
            if str.starts_with('-') {
                str.remove(0);
                prefix = "-";
            }
            let zeros_len = width - prefix.len();
            if str.len() < zeros_len {
                for _ in 0..zeros_len - str.len() { str.insert(0, '0') }
            }
            if prefix == "-" { str.insert(0, prefix.chars().next().unwrap()); }
            N::from_str(&str)
        })
    }

    fn translate(&self, table: Vec<(char, char)>) -> Result<Self, ArrayError> {
        self.map(|item| {
            let str = item.to_string().chars()
                .map(|c| table.iter()
                    .find(|(t, _)| c == *t)
                    .map_or(c, |t| t.1))
                .collect::<String>();
            N::from_str(&str)
        })
    }
}

impl <N: Alphanumeric> ArrayStringManipulate<N> for Result<Array<N>, ArrayError> {

    fn add(&self, other: &Array<N>) -> Self {
        self.clone()?.add(other)
    }

    fn multiply(&self, counts: &Array<usize>) -> Self {
        self.clone()?.multiply(counts)
    }

    fn capitalize(&self) -> Self {
        self.clone()?.capitalize()
    }

    fn lower(&self) -> Self {
        self.clone()?.lower()
    }

    fn upper(&self) -> Self {
        self.clone()?.upper()
    }

    fn swapcase(&self) -> Self {
        self.clone()?.swapcase()
    }

    fn center(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Self {
        self.clone()?.center(width, fill_char)
    }

    fn join(&self, sep: &Array<N>) -> Self {
        self.clone()?.join(sep)
    }

    fn partition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N, N, N>>, ArrayError> {
        ArrayStringManipulate::partition(&self.clone()?, sep)
    }

    fn rpartition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N, N, N>>, ArrayError> {
        self.clone()?.rpartition(sep)
    }

    fn split(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        ArrayStringManipulate::split(&self.clone()?, sep, max_split)
    }

    fn rsplit(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        self.clone()?.rsplit(sep, max_split)
    }

    fn splitlines(&self, keep_ends: Option<Array<bool>>) -> Result<Array<List<N>>, ArrayError> {
        self.clone()?.splitlines(keep_ends)
    }

    fn replace(&self, old: &Array<N>, new: &Array<N>, count: Option<usize>) -> Self {
        self.clone()?.replace(old, new, count)
    }

    fn strip(&self, chars: Option<Array<N>>) -> Self {
        self.clone()?.strip(chars)
    }

    fn lstrip(&self, chars: Option<Array<N>>) -> Self {
        self.clone()?.lstrip(chars)
    }

    fn rstrip(&self, chars: Option<Array<N>>) -> Self {
        self.clone()?.rstrip(chars)
    }

    fn ljust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Self {
        self.clone()?.ljust(width, fill_char)
    }

    fn rjust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Self {
        self.clone()?.rjust(width, fill_char)
    }

    fn zfill(&self, width: usize) -> Self {
        self.clone()?.zfill(width)
    }

    fn translate(&self, table: Vec<(char, char)>) -> Self {
        self.clone()?.translate(table)
    }
}

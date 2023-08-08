use crate::{
    alphanumeric::prelude::*,
    core::prelude::*,
    errors::prelude::*,
};

/// ArrayTrait - Alphanumeric Array operations
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
    /// assert_eq!(expected, arr.center(&Array::single(9).unwrap(), Some(Array::single('*').unwrap())));
    /// ```
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
    fn rpartition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N, N, N>>, ArrayError>;

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
    /// assert_eq!(expected, ArrayStringManipulate::split(&arr, Some(Array::single("-".to_string()).unwrap()), None));
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
    /// assert_eq!(expected, ArrayStringManipulate::rsplit(&arr, Some(Array::single("-".to_string()).unwrap()), None));
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
    fn rjust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError>;
}

impl <N: Alphanumeric> ArrayStringManipulate<N> for Array<N> {

    // manipulation

    fn add(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._append(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn multiply(&self, counts: &Array<usize>) -> Result<Array<N>, ArrayError> {
        let (array, counts) = self.broadcast_h2(counts)?;
        let elements = array.clone().into_iter().zip(counts)
            .map(|tuple| tuple.0._multiply(tuple.1))
            .collect();
        Array::new(elements, array.get_shape()?)
    }

    fn capitalize(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s._capitalize())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn lower(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s._lower())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn upper(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s._upper())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn swapcase(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s._swapcase())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn center(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError> {
        let fill_char = fill_char.unwrap_or(Array::single(' ')?);
        let (array, width, fill_char) = self.broadcast_h3(width, &fill_char)?;

        let elements = array.into_iter().enumerate()
            .map(|(idx, s)| s._center(width[idx], fill_char[idx]))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn join(&self, sep: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(sep)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._join(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn partition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N, N, N>>, ArrayError> {
        let broadcasted = self.broadcast(sep)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._partition(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn rpartition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N, N, N>>, ArrayError> {
        let broadcasted = self.broadcast(sep)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._rpartition(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn split(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        let sep = sep.unwrap_or(Array::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&sep)?;
        let max_split =
            if let Some(counts) = max_split { Some(counts.broadcast_to(broadcasted.get_shape()?)?) } else { None };
        let elements = broadcasted.clone().into_iter().enumerate()
            .map(|(idx, tuple)| tuple.0._split(tuple.1, max_split.clone().map(|s| s[idx])))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn rsplit(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        let sep = sep.unwrap_or(Array::single(N::from_str(" "))?);
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

    fn replace(&self, old: &Array<N>, new: &Array<N>, count: Option<usize>) -> Result<Array<N>, ArrayError> {
        let broadcasted = Self::broadcast_arrays(vec![self.clone(), old.clone(), new.clone()])?;
        let tupled = (0..broadcasted[0].len()?).map(|i| {
            Tuple3(broadcasted[0][i].clone(), broadcasted[1][i].clone(), broadcasted[2][i].clone())
        }).collect::<Vec<Tuple3<N, N, N>>>();
        let elements = tupled.into_iter()
            .map(|tuple| tuple.0._replace(tuple.1, tuple.2, count))
            .collect();
        Array::new(elements, broadcasted[0].get_shape()?)
    }

    fn strip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        self.lstrip(chars.clone()).rstrip(chars)
    }

    fn lstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        let chars = chars.unwrap_or(Array::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&chars)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._lstrip(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn rstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        let chars = chars.unwrap_or(Array::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&chars)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._rstrip(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn ljust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError> {
        let fill_char = fill_char.unwrap_or(Array::single(' ')?);
        let (array, width, fill_char) = self.broadcast_h3(width, &fill_char)?;

        let elements = array.into_iter().enumerate()
            .map(|(idx, s)| s._ljust(width[idx], fill_char[idx]))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn rjust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError> {
        let fill_char = fill_char.unwrap_or(Array::single(' ')?);
        let tmp_fill_char = Array::single(N::from_str(" ")).broadcast_to(fill_char.get_shape()?)?;
        let tmp_width = Array::single(N::from_str(" ")).broadcast_to(width.get_shape()?)?;
        let broadcasted = Self::broadcast_arrays(vec![self.clone(), tmp_width, tmp_fill_char])?;

        let array = broadcasted[0].clone();
        let width = width.broadcast_to(array.get_shape()?)?;
        let fill_char = fill_char.broadcast_to(array.get_shape()?)?;

        let elements = array.into_iter().enumerate()
            .map(|(idx, s)| s._rjust(width[idx], fill_char[idx]))
            .collect();
        Array::new(elements, self.get_shape()?)
    }
}

impl <N: Alphanumeric> ArrayStringManipulate<N> for Result<Array<N>, ArrayError> {
    fn add(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.add(other)
    }

    fn multiply(&self, counts: &Array<usize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.multiply(counts)
    }

    fn capitalize(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.capitalize()
    }

    fn lower(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.lower()
    }

    fn upper(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.upper()
    }

    fn swapcase(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.swapcase()
    }

    fn center(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.center(width, fill_char)
    }

    fn join(&self, sep: &Array<N>) -> Result<Array<N>, ArrayError> {
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

    fn replace(&self, old: &Array<N>, new: &Array<N>, count: Option<usize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.replace(old, new, count)
    }

    fn strip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.strip(chars)
    }

    fn lstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.lstrip(chars)
    }

    fn rstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.rstrip(chars)
    }

    fn ljust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.ljust(width, fill_char)
    }

    fn rjust(&self, width: &Array<usize>, fill_char: Option<Array<char>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.rjust(width, fill_char)
    }
}

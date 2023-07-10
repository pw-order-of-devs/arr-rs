use crate::{
    alphanumeric::prelude::*,
    core::prelude::*,
    errors::prelude::*,
};

/// ArrayTrait - Alphanumeric Array operations
pub trait ArrayStringIndexing<N: Alphanumeric> where Self: Sized + Clone {

    /// Return string.len() element-wise
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
    /// assert_eq!(expected, arr.count(&Array::single("Aa".to_string()).unwrap()));
    ///
    /// let expected = Array::flat(vec![1]);
    /// let arr = Array::flat(vec!["AaAaAa".to_string()]);
    /// assert_eq!(expected, arr.count(&Array::single("AaAa".to_string()).unwrap()));
    /// ```
    fn count(&self, sub: &Array<N>) -> Result<Array<usize>, ArrayError>;

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
    /// assert_eq!(expected, arr.starts_with(&Array::single("Aa".to_string()).unwrap()));
    /// ```
    fn starts_with(&self, prefix: &Array<N>) -> Result<Array<bool>, ArrayError>;

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
    /// assert_eq!(expected, arr.ends_with(&Array::single("aA".to_string()).unwrap()));
    /// ```
    fn ends_with(&self, suffix: &Array<N>) -> Result<Array<bool>, ArrayError>;

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
    /// assert_eq!(expected, arr.find(&Array::single("aA".to_string()).unwrap()));
    /// ```
    fn find(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError>;

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
    /// assert_eq!(expected, arr.rfind(&Array::single("aA".to_string()).unwrap()));
    /// ```
    fn rfind(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError>;

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
    /// assert_eq!(expected, arr.index(&Array::single("aA".to_string()).unwrap()));
    /// ```
    fn index(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError>;

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
    /// assert_eq!(expected, arr.rindex(&Array::single("aA".to_string()).unwrap()));
    /// ```
    fn rindex(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError>;
}

impl <N: Alphanumeric> ArrayStringIndexing<N> for Array<N> {

    fn str_len(&self) -> Result<Array<usize>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|item| item.to_string().len())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn count(&self, sub: &Array<N>) -> Result<Array<usize>, ArrayError> {
        let broadcasted = self.broadcast(sub)?;
        let elements = broadcasted.clone().into_iter()
            .map(|item| item.0._count(item.1.to_string().as_str()))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn starts_with(&self, prefix: &Array<N>) -> Result<Array<bool>, ArrayError> {
        let broadcasted = self.broadcast(prefix)?;
        let elements = broadcasted.clone().into_iter()
            .map(|item| item.0.to_string().starts_with(&item.1.to_string()))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn ends_with(&self, suffix: &Array<N>) -> Result<Array<bool>, ArrayError> {
        let broadcasted = self.broadcast(suffix)?;
        let elements = broadcasted.clone().into_iter()
            .map(|item| item.0.to_string().ends_with(&item.1.to_string()))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn find(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError> {
        let broadcasted = self.broadcast(sub)?;
        let elements = broadcasted.clone().into_iter()
            .map(|item| match item.0.to_string().find(&item.1.to_string()) {
                Some(idx) => idx as isize,
                None => -1,
            })
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn rfind(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError> {
        let broadcasted = self.broadcast(sub)?;
        let elements = broadcasted.clone().into_iter()
            .map(|item| match item.0.to_string().rfind(&item.1.to_string()) {
                Some(idx) => idx as isize,
                None => -1,
            })
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn index(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError> {
        self.find(sub)
    }

    fn rindex(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError> {
        self.rfind(sub)
    }
}

impl <N: Alphanumeric> ArrayStringIndexing<N> for Result<Array<N>, ArrayError> {

    fn str_len(&self) -> Result<Array<usize>, ArrayError> {
        self.clone()?.str_len()
    }

    fn count(&self, sub: &Array<N>) -> Result<Array<usize>, ArrayError> {
        self.clone()?.count(sub)
    }

    fn starts_with(&self, prefix: &Array<N>) -> Result<Array<bool>, ArrayError> {
        self.clone()?.starts_with(prefix)
    }

    fn ends_with(&self, suffix: &Array<N>) -> Result<Array<bool>, ArrayError> {
        self.clone()?.ends_with(suffix)
    }

    fn find(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError> {
        self.clone()?.find(sub)
    }

    fn rfind(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError> {
        self.clone()?.rfind(sub)
    }

    fn index(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError> {
        self.clone()?.index(sub)
    }

    fn rindex(&self, sub: &Array<N>) -> Result<Array<isize>, ArrayError> {
        self.clone()?.rindex(sub)
    }
}

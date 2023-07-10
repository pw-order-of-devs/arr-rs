use crate::{
    alphanumeric::prelude::*,
    core::prelude::*,
    errors::prelude::*,
};

/// ArrayTrait - Alphanumeric Array operations
pub trait ArrayStringValidate<N: Alphanumeric> where Self: Sized + Clone {

    /// Check if all characters in the string are alphabetic and there is at least one character
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, false, false]);
    /// let arr = Array::flat(vec!["abcd".to_string(), "abc12".to_string(), "".to_string()]);
    /// assert_eq!(expected, arr.is_alpha());
    /// ```
    fn is_alpha(&self) -> Result<Array<bool>, ArrayError>;

    /// Check if all characters in the string are alphanumeric and there is at least one character
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, true, false]);
    /// let arr = Array::flat(vec!["abcd".to_string(), "abc12".to_string(), "".to_string()]);
    /// assert_eq!(expected, arr.is_alnum());
    /// ```
    fn is_alnum(&self) -> Result<Array<bool>, ArrayError>;

    /// Check if all characters in the string are decimal and there is at least one character
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, false, false]);
    /// let arr = Array::flat(vec!["12345".to_string(), "abc12".to_string(), "".to_string()]);
    /// assert_eq!(expected, arr.is_decimal());
    /// ```
    fn is_decimal(&self) -> Result<Array<bool>, ArrayError>;

    /// Check if all characters in the string are numeric and there is at least one character
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, true, false, false, false, false, false]);
    /// let arr = Array::flat(vec!["2".to_string(), "12345".to_string(), "a".to_string(), "abc12".to_string(), "1/4".to_string(), "VIII".to_string(), "".to_string()]);
    /// assert_eq!(expected, arr.is_numeric());
    /// ```
    fn is_numeric(&self) -> Result<Array<bool>, ArrayError>;

    /// Check if all characters in the string are digits and there is at least one character
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, false, false, false, false]);
    /// let arr = Array::flat(vec!["2".to_string(), "12345".to_string(), "a".to_string(), "abc12".to_string(), "".to_string()]);
    /// assert_eq!(expected, arr.is_digit());
    /// ```
    fn is_digit(&self) -> Result<Array<bool>, ArrayError>;

    /// Check if all characters in the string are whitespace and there is at least one character
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![false, false, false, false, true, true]);
    /// let arr = Array::flat(vec!["2".to_string(), "12345".to_string(), "a".to_string(), "abc12".to_string(), " ".to_string(), "    ".to_string()]);
    /// assert_eq!(expected, arr.is_space());
    /// ```
    fn is_space(&self) -> Result<Array<bool>, ArrayError>;

    /// Check if all characters in the string are lowercase and there is at least one character
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, true, false, false, false]);
    /// let arr = Array::flat(vec!["a".to_string(), "abc12".to_string(), "1234".to_string(), "aBc12".to_string(), "".to_string()]);
    /// assert_eq!(expected, arr.is_lower());
    /// ```
    fn is_lower(&self) -> Result<Array<bool>, ArrayError>;

    /// Check if all characters in the string are uppercase and there is at least one character
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![true, true, false, false, false]);
    /// let arr = Array::flat(vec!["A".to_string(), "ABC12".to_string(), "1234".to_string(), "aBc12".to_string(), "".to_string()]);
    /// assert_eq!(expected, arr.is_upper());
    /// ```
    fn is_upper(&self) -> Result<Array<bool>, ArrayError>;
}

impl <N: Alphanumeric> ArrayStringValidate<N> for Array<N> {

    fn is_alpha(&self) -> Result<Array<bool>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|item| !item.to_string().is_empty() && item.to_string().chars().all(|c| c.is_alphabetic()))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn is_alnum(&self) -> Result<Array<bool>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|item| !item.to_string().is_empty() && item.to_string().chars().all(|c| c.is_alphanumeric()))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn is_decimal(&self) -> Result<Array<bool>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|item| !item.to_string().is_empty() && item.to_string().chars().all(|c| c.is_ascii_digit()))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn is_numeric(&self) -> Result<Array<bool>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|item| !item.to_string().is_empty() && item.to_string().chars().all(|c| c.is_numeric()))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn is_digit(&self) -> Result<Array<bool>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|item| item.to_string().len() == 1 && item.to_string().chars().all(|c| c.is_ascii_digit()))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn is_space(&self) -> Result<Array<bool>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|item| !item.to_string().is_empty() && item.to_string().chars().all(|c| c.is_whitespace()))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn is_lower(&self) -> Result<Array<bool>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|item| {
                let filtered = item.to_string().chars().filter(|c| c.is_alphabetic()).collect::<String>();
                !filtered.is_empty() && filtered.chars().all(|c| c.is_lowercase())
            })
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn is_upper(&self) -> Result<Array<bool>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|item| {
                let filtered = item.to_string().chars().filter(|c| c.is_alphabetic()).collect::<String>();
                !filtered.is_empty() && filtered.chars().all(|c| c.is_uppercase())
            })
            .collect();
        Array::new(elements, self.get_shape()?)
    }
}

impl <N: Alphanumeric> ArrayStringValidate<N> for Result<Array<N>, ArrayError> {

    fn is_alpha(&self) -> Result<Array<bool>, ArrayError> {
        self.clone()?.is_alpha()
    }

    fn is_alnum(&self) -> Result<Array<bool>, ArrayError> {
        self.clone()?.is_alnum()
    }

    fn is_decimal(&self) -> Result<Array<bool>, ArrayError> {
        self.clone()?.is_decimal()
    }

    fn is_numeric(&self) -> Result<Array<bool>, ArrayError> {
        self.clone()?.is_numeric()
    }

    fn is_digit(&self) -> Result<Array<bool>, ArrayError> {
        self.clone()?.is_digit()
    }

    fn is_space(&self) -> Result<Array<bool>, ArrayError> {
        self.clone()?.is_space()
    }

    fn is_lower(&self) -> Result<Array<bool>, ArrayError> {
        self.clone()?.is_lower()
    }

    fn is_upper(&self) -> Result<Array<bool>, ArrayError> {
        self.clone()?.is_upper()
    }
}

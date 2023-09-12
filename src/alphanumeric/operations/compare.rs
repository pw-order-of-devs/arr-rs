use crate::{
    alphanumeric::prelude::*,
    core::prelude::*,
    errors::prelude::*,
};
use crate::core::types::compare::CompareOpType;

/// ArrayTrait - Alphanumeric Array operations
pub trait ArrayStringCompare<N: Alphanumeric> where Self: Sized + Clone {

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
    fn compare(&self, other: &Array<N>, cmp_op: impl CompareOpType) -> Result<Array<bool>, ArrayError>;
}

impl <N: Alphanumeric> ArrayStringCompare<N> for Array<N> {

    fn equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._equal(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn not_equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._not_equal(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn greater_equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._greater_equal(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn less_equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._less_equal(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn greater(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._greater(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn less(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0._less(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn compare(&self, other: &Array<N>, cmp_op: impl CompareOpType) -> Result<Array<bool>, ArrayError> {
        let cmp_op = cmp_op.parse_type()?;
        match cmp_op {
            CompareOp::Equals => self.equal(other),
            CompareOp::NotEquals => self.not_equal(other),
            CompareOp::GreaterEqual => self.greater_equal(other),
            CompareOp::LessEqual => self.less_equal(other),
            CompareOp::Greater => self.greater(other),
            CompareOp::Less => self.less(other),
        }
    }
}

impl <N: Alphanumeric> ArrayStringCompare<N> for Result<Array<N>, ArrayError> {

    fn equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        self.clone()?.equal(other)
    }

    fn not_equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        self.clone()?.not_equal(other)
    }

    fn greater_equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        self.clone()?.greater_equal(other)
    }

    fn less_equal(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        self.clone()?.less_equal(other)
    }

    fn greater(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        self.clone()?.greater(other)
    }

    fn less(&self, other: &Array<N>) -> Result<Array<bool>, ArrayError> {
        self.clone()?.less(other)
    }

    fn compare(&self, other: &Array<N>, cmp_op: impl CompareOpType) -> Result<Array<bool>, ArrayError> {
        self.clone()?.compare(other, cmp_op)
    }
}

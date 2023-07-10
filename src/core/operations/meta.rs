use crate::{
    core::prelude::*,
    errors::prelude::*,
};

/// ArrayTrait - Array Metadata functions
pub trait ArrayMeta<T: Clone> where Self: Sized + Clone {

    /// Obtain the vector containing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4], vec![4]).unwrap();
    /// assert_eq!(vec![1, 2, 3, 4], arr.get_elements().unwrap());
    /// ```
    fn get_elements(&self) -> Result<Vec<T>, ArrayError>;

    /// Obtain the vector containing array shape
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4], vec![4]).unwrap();
    /// assert_eq!(vec![4], arr.get_shape().unwrap());
    /// ```
    fn get_shape(&self) -> Result<Vec<usize>, ArrayError>;

    /// Count of array dimensions
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr_1 = Array::new(vec![1,2,3,4], vec![4]).unwrap();
    /// let ndim_1 = arr_1.ndim().unwrap();
    /// assert_eq!(1, ndim_1);
    ///
    /// let arr_2 = Array::new(vec![1,2,3,4], vec![2, 2]).unwrap();
    /// let ndim_2 = arr_2.ndim().unwrap();
    /// assert_eq!(2, ndim_2);
    /// ```
    fn ndim(&self) -> Result<usize, ArrayError>;

    /// Count of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4], vec![4]).unwrap();
    /// let len = arr.len().unwrap();
    /// assert_eq!(4, len);
    /// ```
    fn len(&self) -> Result<usize, ArrayError>;

    /// Check if array element count equals zero
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr_1 = Array::new(vec![1,2,3,4], vec![4]).unwrap();
    /// let empty_1 = arr_1.is_empty().unwrap();
    /// assert_eq!(false, empty_1);
    ///
    /// let arr_2: Array<f64> = Array::empty().unwrap();
    /// let empty_2 = arr_2.is_empty().unwrap();
    /// assert_eq!(true, empty_2);
    /// ```
    fn is_empty(&self) -> Result<bool, ArrayError>;
}

impl <T: ArrayElement> ArrayMeta<T> for Array<T> {

    fn get_elements(&self) -> Result<Vec<T>, ArrayError> {
        Ok(self.elements.clone())
    }

    fn get_shape(&self) -> Result<Vec<usize>, ArrayError> {
        Ok(self.shape.clone())
    }

    fn ndim(&self) -> Result<usize, ArrayError> {
        Ok(self.shape.len())
    }

    fn len(&self) -> Result<usize, ArrayError> {
        Ok(self.elements.len())
    }

    fn is_empty(&self) -> Result<bool, ArrayError> {
        Ok(self.len()? == 0)
    }
}

impl <T: ArrayElement> ArrayMeta<T> for Result<Array<T>, ArrayError> {

    fn get_elements(&self) -> Result<Vec<T>, ArrayError> {
        self.clone()?.get_elements()
    }

    fn get_shape(&self) -> Result<Vec<usize>, ArrayError> {
        self.clone()?.get_shape()
    }

    fn ndim(&self) -> Result<usize, ArrayError> {
        self.clone()?.ndim()
    }

    fn len(&self) -> Result<usize, ArrayError> {
        self.clone()?.len()
    }

    fn is_empty(&self) -> Result<bool, ArrayError> {
        self.clone()?.is_empty()
    }
}

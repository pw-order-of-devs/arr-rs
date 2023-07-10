use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Binary Array operations
pub trait ArrayBinary<N: Numeric> where Self: Sized + Clone {

    /// Compute the bit-wise AND of two arrays element-wise
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
    /// assert_eq!(Array::<i32>::flat(vec![1]), array!([13]).bitwise_and(&array!([17]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![0, 1]), array!([11, 7]).bitwise_and(&array!([4, 25]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![2, 4, 16]), array!([2, 5, 255]).bitwise_and(&array!([3, 14, 16]).unwrap()));
    /// ```
    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Compute the bit-wise OR of two arrays element-wise
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
    /// assert_eq!(Array::<i32>::flat(vec![29]), array!([13]).bitwise_or(&array!([16]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![33, 6]), array!([33, 4]).bitwise_or(&array!([1, 2]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![6, 5, 255]), array!([2, 5, 255]).bitwise_or(&array!([4, 4, 4]).unwrap()));
    /// ```
    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Compute the bit-wise XOR of two arrays element-wise
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
    /// assert_eq!(Array::<i32>::flat(vec![28]), array!([13]).bitwise_xor(&array!([17]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![26]), array!([31]).bitwise_xor(&array!([5]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![26, 5]), array!([31, 3]).bitwise_xor(&array!([5, 6]).unwrap()));
    /// ```
    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Compute bit-wise inversion, or bit-wise NOT, element-wise
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
    /// assert_eq!(Array::<u8>::flat(vec![242]), array!([13]).bitwise_not());
    /// assert_eq!(Array::<u16>::flat(vec![65522]), array!([13]).bitwise_not());
    /// assert_eq!(Array::<i32>::flat(vec![-14]), array!([13]).bitwise_not());
    /// ```
    fn bitwise_not(&self) -> Result<Array<N>, ArrayError>;

    /// Compute bit-wise inversion, or bit-wise NOT, element-wise. Alias on bitwise_not
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
    /// assert_eq!(Array::<u8>::flat(vec![242]), array!([13]).invert());
    /// assert_eq!(Array::<u16>::flat(vec![65522]), array!([13]).invert());
    /// assert_eq!(Array::<i32>::flat(vec![-14]), array!([13]).invert());
    /// ```
    fn invert(&self) -> Result<Array<N>, ArrayError>;

    /// Shift the bits of an integer to the left
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
    /// assert_eq!(Array::<u8>::flat(vec![20]), array!([5]).left_shift(&array!([2]).unwrap()));
    /// assert_eq!(Array::<u8>::flat(vec![10, 20, 40]), array!([5]).left_shift(&array!([1, 2, 3]).unwrap()));
    /// ```
    fn left_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Shift the bits of an integer to the right
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
    /// assert_eq!(Array::<u8>::flat(vec![5]), array!([10]).right_shift(&array!([1]).unwrap()));
    /// assert_eq!(Array::<u8>::flat(vec![5, 2, 1]), array!([10]).right_shift(&array!([1, 2, 3]).unwrap()));
    /// ```
    fn right_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Return the binary representation of the input number as a string
    ///
    /// # Arguments
    ///
    /// * `num` - integer decimal number
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!("10".to_string(), (2 as u8).binary_repr());
    /// assert_eq!("11".to_string(), (3 as u8).binary_repr());
    /// assert_eq!("11111101".to_string(), (-3 as i8).binary_repr());
    /// assert_eq!("11111111".to_string(), (255 as u8).binary_repr());
    ///
    /// assert_eq!("10".to_string(), Array::<u8>::binary_repr(2));
    /// assert_eq!("11".to_string(), Array::<u8>::binary_repr(3));
    /// assert_eq!("11111101".to_string(), Array::<i8>::binary_repr(-3));
    /// assert_eq!("11111111".to_string(), Array::<u8>::binary_repr(255));
    /// ```
    fn binary_repr(num: N) -> String;
}

impl <N: Numeric> ArrayBinary<N> for Array<N> {

    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.bitwise_and(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.bitwise_or(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let elements = self.broadcast(other)?.into_iter()
            .map(|tuple| tuple.0.bitwise_xor(&tuple.1))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn bitwise_not(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.into_iter()
            .map(|&a| a.bitwise_not())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn invert(&self) -> Result<Array<N>, ArrayError> {
        self.bitwise_not()
    }

    fn left_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.left_shift(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn right_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.right_shift(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn binary_repr(num: N) -> String {
        num.binary_repr()
    }
}

impl <N: Numeric> ArrayBinary<N> for Result<Array<N>, ArrayError> {
    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_and(other)
    }

    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_or(other)
    }

    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_xor(other)
    }

    fn bitwise_not(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_not()
    }

    fn invert(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.invert()
    }

    fn left_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.left_shift(other)
    }

    fn right_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.right_shift(other)
    }

    fn binary_repr(num: N) -> String {
        num.binary_repr()
    }
}

use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Binary Array operations
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
    /// assert_eq!(Array::<i32>::flat(vec![1]), array!(i32, [13]).bitwise_and(&array!(i32, [17]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![0, 1]), array!(i32, [11, 7]).bitwise_and(&array!(i32, [4, 25]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![2, 4, 16]), array!(i32, [2, 5, 255]).bitwise_and(&array!(i32, [3, 14, 16]).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// assert_eq!(Array::<i32>::flat(vec![29]), array!(i32, [13]).bitwise_or(&array!(i32, [16]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![33, 6]), array!(i32, [33, 4]).bitwise_or(&array!(i32, [1, 2]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![6, 5, 255]), array!(i32, [2, 5, 255]).bitwise_or(&array!(i32, [4, 4, 4]).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// assert_eq!(Array::<i32>::flat(vec![28]), array!(i32, [13]).bitwise_xor(&array!(i32, [17]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![26]), array!(i32, [31]).bitwise_xor(&array!(i32, [5]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![26, 5]), array!(i32, [31, 3]).bitwise_xor(&array!(i32, [5, 6]).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// assert_eq!(Array::<u8>::flat(vec![242]), array!(u8, [13]).bitwise_not());
    /// assert_eq!(Array::<u16>::flat(vec![65522]), array!(u16, [13]).bitwise_not());
    /// assert_eq!(Array::<i32>::flat(vec![-14]), array!(i32, [13]).bitwise_not());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn bitwise_not(&self) -> Result<Array<N>, ArrayError>;

    /// Compute bit-wise inversion, or bit-wise NOT, element-wise. Alias on `bitwise_not`
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
    /// assert_eq!(Array::<u8>::flat(vec![242]), array!(u8, [13]).invert());
    /// assert_eq!(Array::<u16>::flat(vec![65522]), array!(u16, [13]).invert());
    /// assert_eq!(Array::<i32>::flat(vec![-14]), array!(i32, [13]).invert());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// assert_eq!(Array::<u8>::flat(vec![20]), array!(u8, [5]).left_shift(&array!(u8, [2]).unwrap()));
    /// assert_eq!(Array::<u8>::flat(vec![10, 20, 40]), array!(u8, [5]).left_shift(&array!(u8, [1, 2, 3]).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// assert_eq!(Array::<u8>::flat(vec![5]), array!(u8, [10]).right_shift(&array!(u8, [1]).unwrap()));
    /// assert_eq!(Array::<u8>::flat(vec![5, 2, 1]), array!(u8, [10]).right_shift(&array!(u8, [1, 2, 3]).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// assert_eq!("10".to_string(), 2u8.binary_repr());
    /// assert_eq!("11".to_string(), 3u8.binary_repr());
    /// assert_eq!("11111101".to_string(), (-3i8).binary_repr());
    /// assert_eq!("11111111".to_string(), 255u8.binary_repr());
    ///
    /// assert_eq!("10".to_string(), Array::<u8>::binary_repr(2));
    /// assert_eq!("11".to_string(), Array::<u8>::binary_repr(3));
    /// assert_eq!("11111101".to_string(), Array::<i8>::binary_repr(-3));
    /// assert_eq!("11111111".to_string(), Array::<u8>::binary_repr(255));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn binary_repr(num: N) -> String;
}

impl <N: Numeric> ArrayBinary<N> for Array<N> {

    fn bitwise_and(&self, other: &Self) -> Result<Self, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.bitwise_and(&tuple.1))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn bitwise_or(&self, other: &Self) -> Result<Self, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.bitwise_or(&tuple.1))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn bitwise_xor(&self, other: &Self) -> Result<Self, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let elements = self.broadcast(other)?.into_iter()
            .map(|tuple| tuple.0.bitwise_xor(&tuple.1))
            .collect();
        Self::new(elements, self.get_shape()?)
    }

    fn bitwise_not(&self) -> Result<Self, ArrayError> {
        self.map(|&a| a.bitwise_not())
    }

    fn invert(&self) -> Result<Self, ArrayError> {
        self.bitwise_not()
    }

    fn left_shift(&self, other: &Self) -> Result<Self, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.left_shift(&tuple.1))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn right_shift(&self, other: &Self) -> Result<Self, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.right_shift(&tuple.1))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn binary_repr(num: N) -> String {
        num.binary_repr()
    }
}

impl <N: Numeric> ArrayBinary<N> for Result<Array<N>, ArrayError> {

    fn bitwise_and(&self, other: &Array<N>) -> Self {
        self.clone()?.bitwise_and(other)
    }

    fn bitwise_or(&self, other: &Array<N>) -> Self {
        self.clone()?.bitwise_or(other)
    }

    fn bitwise_xor(&self, other: &Array<N>) -> Self {
        self.clone()?.bitwise_xor(other)
    }

    fn bitwise_not(&self) -> Self {
        self.clone()?.bitwise_not()
    }

    fn invert(&self) -> Self {
        self.clone()?.invert()
    }

    fn left_shift(&self, other: &Array<N>) -> Self {
        self.clone()?.left_shift(other)
    }

    fn right_shift(&self, other: &Array<N>) -> Self {
        self.clone()?.right_shift(other)
    }

    fn binary_repr(num: N) -> String {
        num.binary_repr()
    }
}

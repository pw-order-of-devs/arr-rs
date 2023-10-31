use crate::{
    core::prelude::*,
    errors::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Array Create functions
pub trait ArrayCreate<T: ArrayElement> where Self: Sized + Clone {

    /// Creates new array
    ///
    /// # Arguments
    ///
    /// * `elements` - vector representing array elements
    /// * `shape` - vector representing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1, 2, 3, 4], vec![4]).unwrap();
    /// assert_eq!("[1, 2, 3, 4]", format!("{arr}"));
    ///
    /// let arr = Array::new(vec![1, 2, 3, 4], vec![2, 2]).unwrap();
    /// assert_eq!("[[1, 2], [3, 4]]", format!("{arr}"));
    /// assert_eq!("[[1, 2],\n [3, 4]]", format!("{arr:#}"));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn new(elements: Vec<T>, shape: Vec<usize>) -> Result<Self, ArrayError>;

    /// Creates new array
    ///
    /// # Arguments
    ///
    /// * `elements` - vector representing array elements
    /// * `shape` - vector representing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::create(vec![1, 2, 3, 4], vec![4], None).unwrap();
    /// assert_eq!("[1, 2, 3, 4]", format!("{arr}"));
    ///
    /// let arr = Array::create(vec![1, 2, 3, 4], vec![2, 2], Some(1)).unwrap();
    /// assert_eq!("[[1, 2], [3, 4]]", format!("{arr}"));
    /// assert_eq!("[[1, 2],\n [3, 4]]", format!("{arr:#}"));
    ///
    /// let arr = Array::create(vec![1, 2, 3, 4], vec![2, 2], Some(3)).unwrap();
    /// assert_eq!("[[[1, 2], [3, 4]]]", format!("{arr}"));
    /// assert_eq!("[[[1, 2],\n  [3, 4]]]", format!("{arr:#}"));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn create(elements: Vec<T>, shape: Vec<usize>, ndmin: Option<usize>) -> Result<Self, ArrayError>;

    /// Creates new array with single element
    ///
    /// # Arguments
    ///
    /// * `element` - array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::single(1);
    /// assert_eq!(vec![1], arr.get_elements().unwrap());
    /// assert_eq!(vec![1], arr.get_shape().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn single(element: T) -> Result<Self, ArrayError>;

    /// Creates new flat array
    ///
    /// # Arguments
    ///
    /// * `elements` - vector representing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!(i32, [1, 2, 3, 4]), Array::<i32>::flat(vec![1, 2, 3, 4]));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn flat(elements: Vec<T>) -> Result<Self, ArrayError>;

    /// Creates new empty array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert!(Array::<i32>::empty().is_empty().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn empty() -> Result<Self, ArrayError>;
}

impl <T: ArrayElement> ArrayCreate<T> for Array<T> {

    fn new(elements: Vec<T>, shape: Vec<usize>) -> Result<Self, ArrayError> {
        shape.matches_values_len(&elements)?;
        Ok(Self { elements, shape, })
    }

    fn create(elements: Vec<T>, shape: Vec<usize>, ndmin: Option<usize>) -> Result<Self, ArrayError> {
        let ndmin = ndmin.unwrap_or(0);
        let array = Self::new(elements, shape.clone());
        if ndmin > shape.len() {
            let mut new_shape = vec![1; ndmin - shape.len()];
            new_shape.extend_from_slice(&shape);
            array.reshape(&new_shape)
        } else {
            array
        }
    }

    fn single(element: T) -> Result<Self, ArrayError> {
        Self::new(vec![element], vec![1])
    }

    fn flat(elements: Vec<T>) -> Result<Self, ArrayError> {
        Self::new(elements.clone(), vec![elements.len()])
    }

    fn empty() -> Result<Self, ArrayError> {
        Self::new(vec![], vec![0])
    }
}

use crate::{
    core::prelude::*,
    errors::prelude::*,
};

impl <N: ArrayElement> IntoIterator for Array<N> {
    type Item = N;
    type IntoIter = std::vec::IntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, N: ArrayElement> IntoIterator for &'a Array<N> {
    type Item = &'a N;
    type IntoIter = std::slice::Iter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl <N: ArrayElement> FromIterator<N> for Array<N> {

    fn from_iter<T: IntoIterator<Item=N>>(iter: T) -> Self {
        Self::flat(iter.into_iter().collect()).unwrap()
    }
}

/// ArrayTrait - Array Iterable functions
pub trait ArrayIter<T: ArrayElement> where Self: Sized + Clone {

    /// Loop over array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.for_each(|item| println!("{item}")).unwrap();
    /// ```
    fn for_each<F: FnMut(&T)>(&self, f: F) -> Result<(), ArrayError>;

    /// Loop over enumerated array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.for_each_e(|idx, item| println!("{idx}:{item}")).unwrap();
    /// ```
    fn for_each_e<F: FnMut(usize, &T)>(&self, f: F) -> Result<(), ArrayError>;

    /// Map over array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.map(|item| item * 2).unwrap();
    /// ```
    fn map<F: FnMut(&T) -> T>(&self, f: F)-> Result<Array<T>, ArrayError>;

    /// Map over enumerated array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.map_e(|idx, item| item * idx as i32).unwrap();
    /// ```
    fn map_e<F: FnMut(usize, &T) -> T>(&self, f: F)-> Result<Array<T>, ArrayError>;

    /// Filter over array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.filter(|item| item % 2 == 0).unwrap();
    /// ```
    fn filter<F: FnMut(&T) -> bool>(&self, f: F)-> Result<Array<T>, ArrayError>;

    /// Filter over enumerated array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.filter_e(|idx, item| item % (idx + 1) as i32 == 0).unwrap();
    /// ```
    fn filter_e<F: FnMut(usize, &T) -> bool>(&self, f: F)-> Result<Array<T>, ArrayError>;

    /// Filter and map over array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.filter_map(|item| if item % 2 == 0 { Some(*item) } else { None }).unwrap();
    /// ```
    fn filter_map<F: FnMut(&T) -> Option<T>>(&self, f: F)-> Result<Array<T>, ArrayError>;

    /// Filter and map over enumerated array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.filter_map_e(|idx, item| if item % (idx + 1) as i32 == 0 { Some(*item) } else { None }).unwrap();
    /// ```
    fn filter_map_e<F: FnMut(usize, &T) -> Option<T>>(&self, f: F)-> Result<Array<T>, ArrayError>;

    /// Fold elements of array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.fold(0, |a, b| a + b).unwrap();
    /// arr.fold(1, |a, b| a * b).unwrap();
    /// ```
    fn fold<F: FnMut(&T, &T) -> T>(&self, init: T, f: F) -> Result<T, ArrayError>;
}

impl <T: ArrayElement> ArrayIter<T> for Array<T> {

    fn for_each<F: FnMut(&T)>(&self, f: F) -> Result<(), ArrayError> {
        self.elements.iter()
            .for_each(f);
        Ok(())
    }

    fn for_each_e<F: FnMut(usize, &T)>(&self, mut f: F) -> Result<(), ArrayError> {
        self.elements.iter().enumerate()
            .for_each(|(idx, item)| f(idx, item));
        Ok(())
    }

    fn map<F: FnMut(&T) -> T>(&self, f: F) -> Result<Array<T>, ArrayError> {
        let result = self.elements.iter()
            .map(f)
            .collect();
        Ok(result)
    }

    fn map_e<F: FnMut(usize, &T) -> T>(&self, mut f: F) -> Result<Array<T>, ArrayError> {
        let result = self.elements.iter().enumerate()
            .map(|(idx, item)| f(idx, item))
            .collect();
        Ok(result)
    }

    fn filter<F: FnMut(&T) -> bool>(&self, mut f: F) -> Result<Array<T>, ArrayError> {
        self.elements.clone().into_iter()
            .filter(|item| f(item))
            .collect::<Array<T>>()
            .ravel()
    }

    fn filter_e<F: FnMut(usize, &T) -> bool>(&self, mut f: F) -> Result<Array<T>, ArrayError> {
        self.elements.clone().into_iter().enumerate()
            .filter(|(idx, item)| f(*idx, item))
            .map(|i| i.1)
            .collect::<Array<T>>()
            .ravel()
    }

    fn filter_map<F: FnMut(&T) -> Option<T>>(&self, f: F) -> Result<Array<T>, ArrayError> {
        self.elements.clone().iter()
            .filter_map(f)
            .collect::<Array<T>>()
            .ravel()
    }

    fn filter_map_e<F: FnMut(usize, &T) -> Option<T>>(&self, mut f: F) -> Result<Array<T>, ArrayError> {
        self.elements.clone().iter().enumerate()
            .filter_map(|(idx, item)| f(idx, item))
            .collect::<Array<T>>()
            .ravel()
    }

    fn fold<F: FnMut(&T, &T) -> T>(&self, init: T, mut f: F) -> Result<T, ArrayError> {
        let result = self.elements.iter().fold(init, |a, b| f(&a, b));
        Ok(result)
    }
}

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
    /// let arr = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
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
    /// let arr = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
    /// arr.for_each_e(|idx, item| println!("{idx}:{item}")).unwrap();
    /// ```
    fn for_each_e<F: FnMut(usize, &T)>(&self, f: F) -> Result<(), ArrayError>;

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
    /// let arr: Array<i32> = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
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
    /// let arr: Array<i32> = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
    /// arr.filter_e(|idx, item| item % (idx + 1) as i32 == 0).unwrap();
    /// ```
    fn filter_e<F: FnMut(usize, &T) -> bool>(&self, f: F)-> Result<Array<T>, ArrayError>;
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
}

/// ArrayTrait - Array Iterable functions
pub trait ArrayIterMut<S: ArrayElement, T: ArrayElement> where Self: Sized + Clone {

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
    /// let arr: Array<i32> = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
    /// arr.map(|item| item * 2).unwrap();
    /// ```
    fn map<F: FnMut(&T) -> S>(&self, f: F)-> Result<Array<S>, ArrayError>;

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
    /// let arr: Array<i32> = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
    /// arr.map_e(|idx, item| item * idx as i32).unwrap();
    /// ```
    fn map_e<F: FnMut(usize, &T) -> S>(&self, f: F)-> Result<Array<S>, ArrayError>;

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
    /// let arr: Array<i32> = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
    /// arr.filter_map(|item| if item % 2 == 0 { Some(*item) } else { None }).unwrap();
    /// ```
    fn filter_map<F: FnMut(&T) -> Option<S>>(&self, f: F)-> Result<Array<S>, ArrayError>;

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
    /// let arr: Array<i32> = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
    /// arr.filter_map_e(|idx, item| if item % (idx + 1) as i32 == 0 { Some(*item) } else { None }).unwrap();
    /// ```
    fn filter_map_e<F: FnMut(usize, &T) -> Option<S>>(&self, f: F)-> Result<Array<S>, ArrayError>;

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
    /// let arr: Array<i32> = Array::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
    /// arr.fold(0, |a, b| a + b).unwrap();
    /// arr.fold(1, |a, b| a * b).unwrap();
    /// ```
    fn fold<F: FnMut(&S, &T) -> S>(&self, init: S, f: F) -> Result<S, ArrayError>;

    /// 'Zips up' two iterators into a single iterator of pairs
    ///
    /// # Arguments
    ///
    /// * `other` - array to zip with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr_1: Array<i32> = Array::flat(vec![1, 2]).unwrap();
    /// let arr_2: Array<i32> = Array::flat(vec![5, 6]).unwrap();
    /// assert_eq!(Array::flat(vec![Tuple2(1, 5), Tuple2(2, 6)]), arr_1.zip(&arr_2));
    /// ```
    fn zip(&self, other: &Array<S>) -> Result<Array<Tuple2<T, S>>, ArrayError>;
}

impl <S: ArrayElement, T: ArrayElement> ArrayIterMut<S, T> for Array<T> {

    fn map<F: FnMut(&T) -> S>(&self, f: F) -> Result<Array<S>, ArrayError> {
        self.elements.iter()
            .map(f)
            .collect::<Array<S>>()
            .reshape(&self.get_shape()?)
    }

    fn map_e<F: FnMut(usize, &T) -> S>(&self, mut f: F) -> Result<Array<S>, ArrayError> {
        self.elements.iter().enumerate()
            .map(|(idx, item)| f(idx, item))
            .collect::<Array<S>>()
            .reshape(&self.get_shape()?)
    }

    fn filter_map<F: FnMut(&T) -> Option<S>>(&self, f: F) -> Result<Array<S>, ArrayError> {
        self.elements.clone().iter()
            .filter_map(f)
            .collect::<Array<S>>()
            .ravel()
    }

    fn filter_map_e<F: FnMut(usize, &T) -> Option<S>>(&self, mut f: F) -> Result<Array<S>, ArrayError> {
        self.elements.clone().iter().enumerate()
            .filter_map(|(idx, item)| f(idx, item))
            .collect::<Array<S>>()
            .ravel()
    }

    fn fold<F: FnMut(&S, &T) -> S>(&self, init: S, mut f: F) -> Result<S, ArrayError> {
        let result = self.elements.iter().fold(init, |a, b| f(&a, b));
        Ok(result)
    }

    fn zip(&self, other: &Array<S>) -> Result<Array<Tuple2<T, S>>, ArrayError> {
        let other = other.broadcast_to(self.get_shape()?)?;
        self.get_elements()?.into_iter()
            .zip(other.get_elements()?)
            .map(|item| Tuple2(item.0, item.1))
            .collect::<Array<Tuple2<T, S>>>()
            .reshape(&self.get_shape()?)
    }
}

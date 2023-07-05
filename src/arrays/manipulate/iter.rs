use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    manipulate::{
        ArrayManipulate,
        iter::ArrayIter,
    },
    types::ArrayElement,
};

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

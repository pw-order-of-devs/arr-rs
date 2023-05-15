use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    manipulate::{
        ArrayManipulate,
        iter::ArrayIter,
    },
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayIter<N> for Array<N> {

    fn for_each<F: FnMut(&N)>(&self, f: F) -> Result<(), ArrayError> {
        self.elements.iter()
            .for_each(f);
        Ok(())
    }

    fn for_each_e<F: FnMut(usize, &N)>(&self, mut f: F) -> Result<(), ArrayError> {
        self.elements.iter().enumerate()
            .for_each(|(idx, item)| f(idx, item));
        Ok(())
    }

    fn map<F: FnMut(&N) -> N>(&self, f: F) -> Result<Array<N>, ArrayError> {
        let result = self.elements.iter()
            .map(f)
            .collect();
        Ok(result)
    }

    fn map_e<F: FnMut(usize, &N) -> N>(&self, mut f: F) -> Result<Array<N>, ArrayError> {
        let result = self.elements.iter().enumerate()
            .map(|(idx, item)| f(idx, item))
            .collect();
        Ok(result)
    }

    fn filter<F: FnMut(&N) -> bool>(&self, mut f: F) -> Result<Array<N>, ArrayError> {
        self.elements.clone().into_iter()
            .filter(|item| f(item))
            .collect::<Array<N>>()
            .ravel()
    }

    fn filter_e<F: FnMut(usize, &N) -> bool>(&self, mut f: F) -> Result<Array<N>, ArrayError> {
        self.elements.clone().into_iter().enumerate()
            .filter(|(idx, item)| f(*idx, item))
            .map(|i| i.1)
            .collect::<Array<N>>()
            .ravel()
    }

    fn filter_map<F: FnMut(&N) -> Option<N>>(&self, f: F) -> Result<Array<N>, ArrayError> {
        self.elements.clone().iter()
            .filter_map(f)
            .collect::<Array<N>>()
            .ravel()
    }

    fn filter_map_e<F: FnMut(usize, &N) -> Option<N>>(&self, mut f: F) -> Result<Array<N>, ArrayError> {
        self.elements.clone().iter().enumerate()
            .filter_map(|(idx, item)| f(idx, item))
            .collect::<Array<N>>()
            .ravel()
    }

    fn fold<F: FnMut(&N, &N) -> N>(&self, init: N, mut f: F) -> Result<N, ArrayError> {
        let result = self.elements.iter().fold(init, |a, b| f(&a, b));
        Ok(result)
    }
}

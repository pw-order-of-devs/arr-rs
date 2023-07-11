use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

use crate::core::prelude::*;

// ==== Indexing

impl <T: ArrayElement> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl <T: ArrayElement> Index<&[usize]> for Array<T> {
    type Output = T;

    fn index(&self, coords: &[usize]) -> &Self::Output {
        let index = self.index_at(coords).unwrap_or_else(|err| panic!("{err}"));
        &self.elements[index]
    }
}

impl <T: ArrayElement> IndexMut<usize> for Array<T> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

// ==== Compare

impl <T: ArrayElement> PartialEq for Array<T> {

    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.get_shape(), other.get_shape());
        self.elements.iter()
            .zip(&other.elements)
            .all(|(a, b)| a == b)
    }
}

impl <T: ArrayElement> PartialOrd for Array<T> {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert_eq!(self.get_shape(), other.get_shape());
        self.elements.partial_cmp(&other.elements)
    }

    fn lt(&self, other: &Self) -> bool {
        assert_eq!(self.get_shape(), other.get_shape());
        self.elements.lt(&other.elements)
    }

    fn le(&self, other: &Self) -> bool {
        assert_eq!(self.get_shape(), other.get_shape());
        self.elements.le(&other.elements)
    }

    fn gt(&self, other: &Self) -> bool {
        assert_eq!(self.get_shape(), other.get_shape());
        self.elements.gt(&other.elements)
    }

    fn ge(&self, other: &Self) -> bool {
        assert_eq!(self.get_shape(), other.get_shape());
        self.elements.ge(&other.elements)
    }
}

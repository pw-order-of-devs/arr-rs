use std::cmp::Ordering;
use crate::prelude::{Array, Numeric};

impl <N: Numeric> PartialEq for Array<N> {

    fn eq(&self, other: &Self) -> bool {
        compare_shape(self, other);
        self.elements.iter()
            .zip(&other.elements)
            .all(|(a, b)| a == b)
    }
}

impl <N: Numeric> PartialOrd for Array<N> {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        compare_shape(self, other);
        self.elements.partial_cmp(&other.elements)
    }

    fn lt(&self, other: &Self) -> bool {
        compare_shape(self, other);
        self.elements.lt(&other.elements)
    }

    fn le(&self, other: &Self) -> bool {
        compare_shape(self, other);
        self.elements.le(&other.elements)
    }

    fn gt(&self, other: &Self) -> bool {
        compare_shape(self, other);
        self.elements.gt(&other.elements)
    }

    fn ge(&self, other: &Self) -> bool {
        compare_shape(self, other);
        self.elements.ge(&other.elements)
    }
}

fn compare_shape<N: Numeric>(arr1: &Array<N>, arr2: &Array<N>) {
    if arr1.shape != arr2.shape {
        panic!("Cannot compare arrays of different shape: {:?} and {:?}", arr1.shape, arr2.shape)
    }
}

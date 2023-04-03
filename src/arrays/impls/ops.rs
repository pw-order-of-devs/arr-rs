use std::cmp::Ordering;
use std::ops::{
    Add, AddAssign,
    Div, DivAssign,
    Mul, MulAssign,
    Rem, RemAssign,
    Sub, SubAssign,
};

use crate::arrays::Array;
use crate::traits::{
    create::ArrayCreate,
    manipulate::ArrayManipulate,
    meta::ArrayMeta,
    types::Numeric,
};

// ==== Compare

impl <N: Numeric> PartialEq for Array<N> {

    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.get_shape(), other.get_shape());
        self.elements.iter()
            .zip(&other.elements)
            .all(|(a, b)| a == b)
    }
}

impl <N: Numeric> PartialOrd for Array<N> {

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

// ==== Ops

macro_rules! impl_op {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl<N: Numeric> $trait<Array<N>> for Array<N> {
            type Output = Array<N>;

            fn $fn(self, other: Self) -> Self::Output {
                assert_eq!(self.get_shape(), other.get_shape());

                let elements = self.elements.into_iter()
                    .zip(other.elements.into_iter())
                    .map(|(a, b)| a $op b)
                    .collect();

                Array::new(elements, self.shape)
            }
        }

        impl<N: Numeric> $trait<N> for Array<N> {
            type Output = Array<N>;

            fn $fn(self, other: N) -> Self::Output {
                self.map(|i| *i $op other)
                    .reshape(self.shape)
            }
        }
    };
}

macro_rules! impl_op_assign {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl<N: Numeric> $trait<Array<N>> for Array<N> {
            fn $fn(&mut self, other: Self) -> () {
                assert_eq!(self.get_shape(), other.get_shape());

                self.elements.iter_mut()
                    .zip(other.elements.into_iter())
                    .for_each(|(a, b)| *a $op b);
            }
        }

        impl<N: Numeric> $trait<N> for Array<N> {
            fn $fn(&mut self, other: N) -> () {
                self.elements.iter_mut()
                    .for_each(|a| *a $op other);
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);
impl_op!(Rem, rem, %);

impl_op_assign!(AddAssign, add_assign, +=);
impl_op_assign!(SubAssign, sub_assign, -=);
impl_op_assign!(MulAssign, mul_assign, *=);
impl_op_assign!(DivAssign, div_assign, /=);
impl_op_assign!(RemAssign, rem_assign, %=);

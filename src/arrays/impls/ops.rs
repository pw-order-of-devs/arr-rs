use std::cmp::Ordering;
use std::ops::{
    Add, AddAssign,
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Div, DivAssign,
    Index, IndexMut,
    Mul, MulAssign,
    Neg, Not,
    Rem, RemAssign,
    Sub, SubAssign,
};

use crate::arrays::Array;
use crate::traits::{
    create::ArrayCreate,
    errors::ArrayError,
    indexing::ArrayIndexing,
    manipulate::ArrayManipulate,
    meta::ArrayMeta,
    types::{
        numeric::Numeric,
        numeric_ops::NumericOps,
        signed_numeric::SignedNumeric,
        bool_numeric::BoolNumeric,
    },
};

// ==== Indexing

impl <N: Numeric> Index<usize> for Array<N> {
    type Output = N;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl <N: Numeric> Index<&[usize]> for Array<N> {
    type Output = N;

    fn index(&self, coords: &[usize]) -> &Self::Output {
        let index = self.index_at(coords).unwrap_or_else(|err| panic!("{err}"));
        &self.elements[index]
    }
}

impl <N: Numeric> IndexMut<usize> for Array<N> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl <N: Numeric> IndexMut<&[usize]> for Array<N> {

    fn index_mut(&mut self, coords: &[usize]) -> &mut Self::Output {
        let index = self.index_at(coords).unwrap_or_else(|err| panic!("{err}"));
        &mut self.elements[index]
    }
}

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
    ($op_trait: ident, $op_func: ident, $op_assign_trait: ident, $op_assign_func: ident) => {
        impl<N: NumericOps> $op_trait<Array<N>> for Array<N> {
            type Output = Array<N>;

            fn $op_func(self, other: Self) -> Self::Output {
                assert_eq!(self.get_shape(), other.get_shape());

                let elements = self.elements.into_iter()
                    .zip(other.elements.into_iter())
                    .map(|(a, b)| a.$op_func(b))
                    .collect();

                Array::new(elements, self.shape).unwrap()
            }
        }

        impl<N: NumericOps> $op_trait<N> for Array<N> {
            type Output = Result<Array<N>, ArrayError>;

            fn $op_func(self, other: N) -> Self::Output {
                self.map(|i| i.$op_func(other))
                    .reshape(self.shape)
            }
        }

        impl<N: NumericOps> $op_assign_trait<Array<N>> for Array<N> {
            fn $op_assign_func(&mut self, other: Self) -> () {
                assert_eq!(self.get_shape(), other.get_shape());

                self.elements.iter_mut()
                    .zip(other.elements.into_iter())
                    .for_each(|(a, b)| a.$op_assign_func(b));
            }
        }

        impl<N: NumericOps> $op_assign_trait<N> for Array<N> {
            fn $op_assign_func(&mut self, other: N) -> () {
                self.elements.iter_mut()
                    .for_each(|a| a.$op_assign_func(other));
            }
        }
    };
}

impl_op!(Add, add, AddAssign, add_assign);
impl_op!(Sub, sub, SubAssign, sub_assign);
impl_op!(Mul, mul, MulAssign, mul_assign);
impl_op!(Div, div, DivAssign, div_assign);
impl_op!(Rem, rem, RemAssign, rem_assign);

// ==== Signed Ops

impl <N: SignedNumeric> Neg for Array<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let elements = self.elements.into_iter()
            .map(|a| -a)
            .collect();

        Array::new(elements, self.shape).unwrap()
    }
}

// ==== Bool Ops

impl <N: BoolNumeric + From<<N as Not>::Output>> Not for Array<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        let elements: Vec<N> = self.elements.into_iter()
            .map(|x| (!x).into())
            .collect();

        Array::new(elements, self.shape).unwrap()
    }
}

macro_rules! impl_bitwise_ops {
    ($op_trait: ident, $op_func: ident, $op_assign_trait: ident, $op_assign_func: ident) => {
        impl<N: Numeric + $op_trait<Output = N>> $op_trait<Array<N>> for Array<N> {
            type Output = Array<N>;

            fn $op_func(self, other: Array<N>) -> Self::Output {
                assert_eq!(self.get_shape(), other.get_shape());

                let elements = self.elements.into_iter()
                    .zip(other.elements.into_iter())
                    .map(|(a, b)| a.$op_func(b))
                    .collect();
                Array { elements,shape: self.shape, }
            }
        }

        impl<N: Numeric + $op_trait<Output = N>> $op_trait<N> for Array<N> {
            type Output = Array<N>;

            fn $op_func(self, other: N) -> Self::Output {
                let elements = self.elements.into_iter()
                    .map(|a| a.$op_func(other))
                    .collect();
                Array { elements,shape: self.shape, }
            }
        }

        impl<N: Numeric + $op_trait<Output = N>> $op_assign_trait<Array<N>> for Array<N> {
            fn $op_assign_func(&mut self, other: Array<N>) {
                assert_eq!(self.get_shape(), other.get_shape());

                self.elements.iter_mut()
                    .zip(other.elements.into_iter())
                    .for_each(|(a, b)| *a = a.$op_func(b));
            }
        }

        impl<N: Numeric + $op_trait<Output = N>> $op_assign_trait<N> for Array<N> {
            fn $op_assign_func(&mut self, other: N) {
                self.elements.iter_mut()
                    .for_each(|a| *a = a.$op_func(other));
            }
        }
    };
}

impl_bitwise_ops!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_bitwise_ops!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_bitwise_ops!(BitXor, bitxor, BitXorAssign, bitxor_assign);

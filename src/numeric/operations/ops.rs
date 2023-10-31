use std::ops::{
    Add, AddAssign,
    Div, DivAssign,
    Mul, MulAssign,
    Neg,
    Rem, RemAssign,
    Sub, SubAssign,
};

use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};

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
                    .reshape(&self.shape)
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

impl <N: SignedNumericOps> Neg for Array<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let elements = self.elements.into_iter()
            .map(|a| -a)
            .collect();

        Self::new(elements, self.shape).unwrap()
    }
}

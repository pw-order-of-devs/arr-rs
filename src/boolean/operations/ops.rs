use std::ops::{
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Not,
};

use crate::{
    boolean::prelude::*,
    core::prelude::*,
    numeric::prelude::*,
};

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

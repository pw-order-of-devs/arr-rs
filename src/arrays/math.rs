use crate::arrays::Array;
use crate::traits::{
    math::ArrayMath,
    types::Numeric,
};

impl <N: Numeric> ArrayMath<N> for Array<N> {

    fn product(&self) -> N {
        self.elements.iter().fold(N::ONE, |acc, x| acc * *x)
    }

    fn sum(&self) -> N {
        self.elements.iter().fold(N::ZERO, |acc, x| acc + *x)
    }
}

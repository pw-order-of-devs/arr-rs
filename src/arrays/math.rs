use crate::arrays::Array;
use crate::traits::{
    manipulate::ArrayManipulate,
    math::ArrayMath,
    types::numeric_ops::NumericOps,
};

impl <N: NumericOps> ArrayMath<N> for Array<N> {

    fn power(&self, value: N) -> Self {
        self.map(|i| N::from(i.to_f64().powf(value.to_f64())))
    }

    fn product(&self) -> N {
        self.elements.iter().fold(N::ONE, |acc, x| acc * *x)
    }

    fn sum(&self) -> N {
        self.elements.iter().fold(N::ZERO, |acc, x| acc + *x)
    }
}

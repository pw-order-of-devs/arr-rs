use crate::arrays::Array;
use crate::traits::{
    manipulate::ArrayManipulate,
    math::ArrayMath,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayMath<N> for Array<N> {

    fn power(&self, value: N) -> Self {
        self.map(|i| N::from(i.to_f64().powf(value.to_f64())))
    }

    fn product(&self) -> N {
        self.elements.iter().fold(N::ONE, |acc, x| N::from(acc.to_f64() * x.to_f64()))
    }

    fn sum(&self) -> N {
        self.elements.iter().fold(N::ZERO, |acc, x| N::from(acc.to_f64() + x.to_f64()))
    }

    fn cumsum(&self) -> Self {
        let mut acc = N::ZERO;
        self.map(|&x| {
            acc = N::from(acc.to_f64() + x.to_f64());
            acc
        })
    }
}

use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    manipulate::iter::ArrayIter,
    math::ArrayMath,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayMath<N> for Array<N> {

    fn power(&self, value: N) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().powf(value.to_f64())))
    }

    fn product(&self) -> Result<N, ArrayError> {
        Ok(self.elements.iter().fold(N::ONE, |acc, x| N::from(acc.to_f64() * x.to_f64())))
    }

    fn sum(&self) -> Result<N, ArrayError> {
        Ok(self.elements.iter().fold(N::ZERO, |acc, x| N::from(acc.to_f64() + x.to_f64())))
    }

    fn cumsum(&self) -> Result<Array<N>, ArrayError> {
        let mut acc = N::ZERO;
        self.map(|&x| {
            acc = N::from(acc.to_f64() + x.to_f64());
            acc
        })
    }
}

impl <N: Numeric> ArrayMath<N> for Result<Array<N>, ArrayError> {

    fn power(&self, value: N) -> Result<Array<N>, ArrayError> {
        self.clone()?.power(value)
    }

    fn product(&self) -> Result<N, ArrayError> {
        self.clone()?.product()
    }

    fn sum(&self) -> Result<N, ArrayError> {
        self.clone()?.sum()
    }

    fn cumsum(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.cumsum()
    }
}

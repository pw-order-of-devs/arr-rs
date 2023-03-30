use std::ops::{Add, Div, Mul, Sub};

use crate::base::base_type::Numeric;
use crate::prelude::{Array, ArrayBase};

// ==== Add

impl <N: Numeric> Add<Array<N>> for Array<N> {
    type Output = Array<N>;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.get_shape(), other.get_shape(), "Arrays must have the same shape");

        let elements = self.elements.into_iter()
            .zip(other.elements.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Array::new(elements, self.shape)
    }
}

impl <N: Numeric> Add<N> for Array<N> {
    type Output = Array<N>;

    fn add(self, other: N) -> Self::Output {
        self.map(|i| *i + other)
            .reshape(self.shape)
    }
}

// ==== Sub

impl <N: Numeric> Sub<Array<N>> for Array<N> {
    type Output = Array<N>;

    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.get_shape(), other.get_shape(), "Arrays must have the same shape");

        let elements = self.elements.into_iter()
            .zip(other.elements.into_iter())
            .map(|(a, b)| a - b)
            .collect();

        Array::new(elements, self.shape)
    }
}

impl <N: Numeric> Sub<N> for Array<N> {
    type Output = Array<N>;

    fn sub(self, other: N) -> Self::Output {
        self.map(|i| *i - other)
            .reshape(self.shape)
    }
}

// ==== Mul

impl <N: Numeric> Mul<Array<N>> for Array<N> {
    type Output = Array<N>;

    fn mul(self, other: Self) -> Self::Output {
        assert_eq!(self.get_shape(), other.get_shape(), "Arrays must have the same shape");

        let elements = self.elements.into_iter()
            .zip(other.elements.into_iter())
            .map(|(a, b)| a * b)
            .collect();

        Array::new(elements, self.shape)
    }
}

impl <N: Numeric> Mul<N> for Array<N> {
    type Output = Array<N>;

    fn mul(self, other: N) -> Self::Output {
        self.map(|i| *i * other)
            .reshape(self.shape)
    }
}

// ==== Div

impl <N: Numeric> Div<Array<N>> for Array<N> {
    type Output = Array<N>;

    fn div(self, other: Self) -> Self::Output {
        assert_eq!(self.get_shape(), other.get_shape(), "Arrays must have the same shape");

        let elements = self.elements.into_iter()
            .zip(other.elements.into_iter())
            .map(|(a, b)| a / b)
            .collect();

        Array::new(elements, self.shape)
    }
}

impl <N: Numeric> Div<N> for Array<N> {
    type Output = Array<N>;

    fn div(self, other: N) -> Self::Output {
        self.map(|i| *i / other)
            .reshape(self.shape)
    }
}

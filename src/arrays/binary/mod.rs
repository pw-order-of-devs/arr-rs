use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    binary::ArrayBinary,
    create::ArrayCreate,
    manipulate::broadcast::ArrayBroadcast,
    meta::ArrayMeta,
    types::numeric::Numeric,
    validators::validate_shape::ValidateShape,
};

impl <N: Numeric> ArrayBinary<N> for Array<N> {

    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.bitwise_and(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.bitwise_or(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let elements = self.broadcast(other)?.into_iter()
            .map(|tuple| tuple.0.bitwise_xor(&tuple.1))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn bitwise_not(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.into_iter()
            .map(|&a| a.bitwise_not())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn invert(&self) -> Result<Array<N>, ArrayError> {
        self.bitwise_not()
    }

    fn left_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.left_shift(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn right_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.right_shift(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }
}

impl <N: Numeric> ArrayBinary<N> for Result<Array<N>, ArrayError> {

    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_and(other)
    }

    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_or(other)
    }

    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_xor(other)
    }

    fn bitwise_not(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_not()
    }

    fn invert(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.invert()
    }

    fn left_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.left_shift(other)
    }

    fn right_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.right_shift(other)
    }
}

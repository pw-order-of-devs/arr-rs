use crate::errors::prelude::*;

/// the ord of norm operation
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TensorAxes {
    /// isize
    Int(isize),
    /// vector of isize
    Vec(Vec<isize>),
    /// tuple of isize
    Tuple((isize, isize)),
    /// tuple of vector isize
    TupleVec((Vec<isize>, Vec<isize>)),
}

/// `TensorAxes` trait
pub trait TensorAxesType: Clone {

    /// Parse input to `TensorAxes` type
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn to_axes(self) -> Result<TensorAxes, ArrayError>;

    /// Parse `TensorAxes` type to tuple of vec of isize
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn get_vecs(self) -> Result<(Vec<isize>, Vec<isize>), ArrayError> {
        Err(ArrayError::NotImplemented)
    }
}

impl TensorAxesType for TensorAxes {
    fn to_axes(self) -> Result<TensorAxes, ArrayError> {
        Ok(self)
    }

    fn get_vecs(self) -> Result<(Vec<isize>, Vec<isize>), ArrayError> {
        let result = match self {
            Self::Int(value) => ((-value..0).collect(), (0..value).collect()),
            Self::Vec(value) => {
                if value.len() != 2 { return Err(ArrayError::OutOfBounds { value: "axes vecs must be of length `2`" }) }
                (vec![value[0]], vec![value[1]])
            },
            Self::Tuple((value_a, value_b)) => (vec![value_a], vec![value_b]),
            Self::TupleVec((value_a, value_b)) => {
                if value_a.len() != 2 || value_b.len() != 2 { return Err(ArrayError::OutOfBounds { value: "axes vecs must be of length `2`" }) }
                (value_a, value_b)
            },
        };
        Ok(result)
    }
}

impl TensorAxesType for isize {
    fn to_axes(self) -> Result<TensorAxes, ArrayError> {
        Ok(TensorAxes::Int(self))
    }
}

impl TensorAxesType for Vec<isize> {
    fn to_axes(self) -> Result<TensorAxes, ArrayError> {
        Ok(TensorAxes::Vec(self))
    }
}

impl TensorAxesType for (isize, isize) {
    fn to_axes(self) -> Result<TensorAxes, ArrayError> {
        Ok(TensorAxes::Tuple(self))
    }
}

impl TensorAxesType for (Vec<isize>, Vec<isize>) {
    fn to_axes(self) -> Result<TensorAxes, ArrayError> {
        Ok(TensorAxes::TupleVec(self))
    }
}

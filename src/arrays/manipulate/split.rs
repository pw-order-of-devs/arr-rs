use crate::arrays::Array;
use crate::ext::vec_ext::{VecInsertAt, VecRemoveAt};
use crate::traits::{
    errors::ArrayError,
    manipulate::{
        ArrayManipulate,
        axis::ArrayAxis,
        split::ArraySplit,
    },
    math::ArrayMath,
    meta::ArrayMeta,
    types::numeric::Numeric,
    validators::{
        validate_axis::ValidateAxis,
        validate_dimension::ValidateDimension,
        validate_has_error::ValidateHasError,
    },
};

impl <N: Numeric> ArraySplit<N> for Array<N> {

    fn array_split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<N>>, ArrayError> {
        if parts == 0 { return Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", }) }
        self.axis_opt_in_bounds(axis)?;
        if self.is_empty() { return Ok(vec![self.clone()]) }

        let axis = axis.unwrap_or(0);
        let n_total = self.len();

        let (sections, extras) = (n_total / parts, n_total % parts);
        let section_sizes = std::iter::repeat(sections + 1)
            .take(extras)
            .chain(std::iter::repeat(sections).take(parts - extras))
            .collect::<Vec<usize>>()
            .insert_at(0, 0);
        let div_points = section_sizes.into_iter()
            .collect::<Array<usize>>()
            .cumsum()
            .get_elements();

        let arr = self.rollaxis(axis as isize, None);
        if let Ok(arr) = arr {
            let result = div_points
                .windows(2)
                .map(|w| arr.clone().into_iter()
                    .skip(w[0]).take(w[1] - w[0])
                    .collect::<Self>())
                .map(|m| {
                    if self.ndim() == 1 { Ok(m) }
                    else {
                        let mut new_shape = self.get_shape();
                        new_shape[axis] /= self.get_shape().remove_at(axis).iter().product::<usize>();
                        if new_shape[axis] == 0 { new_shape[axis] = 1 }
                        m.reshape(new_shape)
                    }
                })
                .collect::<Vec<Result<Self, _>>>();
            if let Err(err) = result.has_error() { Err(err) }
            else { Ok(result.into_iter().map(|a| a.unwrap()).collect()) }
        } else {
            Err(arr.err().unwrap())
        }
    }

    fn split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<N>>, ArrayError> {
        self.axis_opt_in_bounds(axis)?;
        if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            if self.is_empty() { return Ok(vec![self.clone()]) }
            let n_total = self.shape[axis.unwrap_or(0)];

            if n_total % parts != 0 { Err(ArrayError::ParameterError { param: "parts", message: "array split does not result in an equal division", }) }
            else { self.array_split(parts, axis) }
        }
    }

    fn hsplit(&self, parts: usize) -> Result<Vec<Array<N>>, ArrayError> {
        self.is_dim_unsupported(&[0])?;
        if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            match self.ndim() {
                1 => self.split(parts, Some(0)),
                _ => self.split(parts, Some(1)),
            }
        }
    }

    fn vsplit(&self, parts: usize) -> Result<Vec<Array<N>>, ArrayError> {
        self.is_dim_unsupported(&[0, 1])?;
        if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            self.split(parts, Some(0))
        }
    }

    fn dsplit(&self, parts: usize) -> Result<Vec<Array<N>>, ArrayError> {
        self.is_dim_unsupported(&[0, 1, 2])?;
        if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            self.split(parts, Some(2))
        }
    }
}

impl <N: Numeric> ArraySplit<N> for Result<Array<N>, ArrayError> {

    fn array_split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<N>>, ArrayError> {
        self.clone()?.array_split(parts, axis)
    }

    fn split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<N>>, ArrayError> {
        self.clone()?.split(parts, axis)
    }

    fn hsplit(&self, parts: usize) -> Result<Vec<Array<N>>, ArrayError> {
        self.clone()?.hsplit(parts)
    }

    fn vsplit(&self, parts: usize) -> Result<Vec<Array<N>>, ArrayError> {
        self.clone()?.vsplit(parts)
    }

    fn dsplit(&self, parts: usize) -> Result<Vec<Array<N>>, ArrayError> {
        self.clone()?.dsplit(parts)
    }
}

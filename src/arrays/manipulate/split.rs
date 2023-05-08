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
};

impl <N: Numeric> ArraySplit<N> for Array<N> {

    fn array_split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Self>, ArrayError> {
        if parts == 0 { return Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", }) }
        if let Some(axis) = axis { if axis >= self.ndim() { return Err(ArrayError::AxisOutOfBounds) } }
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
            let has_error = result.clone().into_iter().find(|a| a.is_err());
            if let Some(error) = has_error { Err(error.err().unwrap()) }
            else { Ok(result.into_iter().map(|a| a.unwrap()).collect()) }
        } else {
            Err(arr.err().unwrap())
        }
    }

    fn split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Self>, ArrayError> {
        if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else if axis.is_some() && axis.unwrap() >= self.ndim() {
            Err(ArrayError::AxisOutOfBounds)
        } else {
            if self.is_empty() { return Ok(vec![self.clone()]) }
            let n_total = self.shape[axis.unwrap_or(0)];

            if n_total % parts != 0 { Err(ArrayError::ParameterError { param: "parts", message: "array split does not result in an equal division", }) }
            else { self.array_split(parts, axis) }
        }
    }

    fn hsplit(&self, parts: usize) -> Result<Vec<Self>, ArrayError> {
        if self.ndim() == 0 {
            Err(ArrayError::UnsupportedDimension { fun: "hsplit", supported: "at least 1D", })
        } else if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            match self.ndim() {
                1 => self.split(parts, Some(0)),
                _ => self.split(parts, Some(1)),
            }
        }
    }

    fn vsplit(&self, parts: usize) -> Result<Vec<Self>, ArrayError> {
        if self.ndim() < 2 {
            Err(ArrayError::UnsupportedDimension { fun: "vsplit", supported: "at least 2D", })
        } else if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            self.split(parts, Some(0))
        }
    }

    fn dsplit(&self, parts: usize) -> Result<Vec<Self>, ArrayError> {
        if self.ndim() < 3 {
            Err(ArrayError::UnsupportedDimension { fun: "dsplit", supported: "at least 3D", })
        } else if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            self.split(parts, Some(2))
        }
    }
}

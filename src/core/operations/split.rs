use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    validators::prelude::*,
};
use crate::prelude::Numeric;

/// `ArrayTrait` - Array Split functions
pub trait ArraySplit<T: ArrayElement> where Self: Sized + Clone {

    /// Split an array into multiple sub-arrays
    ///
    /// # Arguments
    ///
    /// * `parts` - indices defining how to split the array
    /// * `axis` - the axis along which to split. optional, defaults to 0
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(i32, 0, 7);
    /// let split = arr.array_split(3, None).unwrap();
    /// assert_eq!(vec![array_flat!(i32, 0, 1, 2).unwrap(), array_flat!(i32, 3, 4, 5).unwrap(), array_flat!(i32, 6, 7).unwrap()], split);
    ///
    /// let arr = array_arange!(i32, 0, 8);
    /// let split = arr.array_split(4, None).unwrap();
    /// assert_eq!(vec![array_flat!(i32, 0, 1, 2).unwrap(), array_flat!(i32, 3, 4).unwrap(), array_flat!(i32, 5, 6).unwrap(), array_flat!(i32, 7, 8).unwrap()], split);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn array_split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<T>>, ArrayError>;

    /// Split an array into multiple sub-arrays of equal size
    ///
    /// # Arguments
    ///
    /// * `parts` - indices defining how to split the array
    /// * `axis` - the axis along which to split. optional, defaults to 0
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(i32, 0, 8);
    /// let split = arr.split(3, None).unwrap();
    /// assert_eq!(vec![array_flat!(i32, 0, 1, 2).unwrap(), array_flat!(i32, 3, 4, 5).unwrap(), array_flat!(i32, 6, 7, 8).unwrap()], split);
    ///
    /// let arr = array_arange!(i32, 0, 7);
    /// let split = arr.split(4, None).unwrap();
    /// assert_eq!(vec![array_flat!(i32, 0, 1).unwrap(), array_flat!(i32, 2, 3).unwrap(), array_flat!(i32, 4, 5).unwrap(), array_flat!(i32, 6, 7).unwrap()], split);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<T>>, ArrayError>;

    /// Split an array into multiple sub-arrays of equal size by axis
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to split
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(i32, 0, 3).reshape(&[2, 2]);
    /// let split = arr.split_axis(0).unwrap();
    /// assert_eq!(vec![array!(i32, [[0, 1]]).unwrap(), array!(i32, [[2, 3]]).unwrap()], split);
    ///
    /// let arr = array_arange!(i32, 0, 7).reshape(&[2, 2, 2]);
    /// let split = arr.split_axis(1).unwrap();
    /// assert_eq!(vec![array!(i32, [[[0, 1]], [[4, 5]]]).unwrap(), array!(i32, [[[2, 3]], [[6, 7]]]).unwrap()], split);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn split_axis(&self, axis: usize) -> Result<Vec<Array<T>>, ArrayError>;

    /// Split an array into multiple sub-arrays horizontally (column-wise)
    ///
    /// # Arguments
    ///
    /// * `parts` - indices defining how to split the array
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(i32, 0, 7).reshape(&[2, 2, 2]).unwrap();
    /// let split = arr.hsplit(2).unwrap();
    /// assert_eq!(vec![array!(i32, [[[0, 1]], [[4, 5]]]).unwrap(), array!(i32, [[[2, 3]], [[6, 7]]]).unwrap()], split);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn hsplit(&self, parts: usize) -> Result<Vec<Array<T>>, ArrayError>;

    /// Split an array into multiple sub-arrays vertically (row-wise)
    ///
    /// # Arguments
    ///
    /// * `parts` - indices defining how to split the array
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(i32, 0, 7).reshape(&[2, 2, 2]).unwrap();
    /// let split = arr.vsplit(2).unwrap();
    /// assert_eq!(vec![array!(i32, [[[0, 1], [2, 3]]]).unwrap(), array!(i32, [[[4, 5], [6, 7]]]).unwrap()], split);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn vsplit(&self, parts: usize) -> Result<Vec<Array<T>>, ArrayError>;

    /// Split an array into multiple sub-arrays along the 3rd axis (depth)
    ///
    /// # Arguments
    ///
    /// * `parts` - indices defining how to split the array
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(i32, 0, 7).reshape(&[2, 2, 2]).unwrap();
    /// let split = arr.dsplit(2).unwrap();
    /// assert_eq!(vec![array!(i32, [[[0], [2]], [[4], [6]]]).unwrap(), array!(i32, [[[1], [3]], [[5], [7]]]).unwrap()], split);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn dsplit(&self, parts: usize) -> Result<Vec<Array<T>>, ArrayError>;
}

impl <T: ArrayElement> ArraySplit<T> for Array<T> {

    fn array_split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Self>, ArrayError> {
        if parts == 0 { return Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", }) }
        self.axis_opt_in_bounds(axis)?;
        if self.is_empty()? { return Ok(vec![self.clone()]) }

        let axis = axis.unwrap_or(0);
        let n_total = self.len()?;

        let (sections, extras) = (n_total / parts, n_total % parts);
        let section_sizes = std::iter::repeat(sections + 1)
            .take(extras)
            .chain(std::iter::repeat(sections).take(parts - extras))
            .collect::<Vec<usize>>()
            .insert_at(0, 0);
        let mut div_points = vec![0; section_sizes.len()];
        (0..div_points.len()).for_each(|i| {
            div_points[i] = section_sizes.clone()[0 ..= i]
                .iter_mut()
                .fold(0, |acc, x| { *x += acc; *x });
        });

        let arr = self.rollaxis(axis.to_isize(), None);
        arr.clone().map_or_else(|_| Err(arr.err().unwrap()), |arr| {
            let result = div_points
                .windows(2)
                .map(|w| arr.clone().into_iter()
                    .skip(w[0]).take(w[1] - w[0])
                    .collect::<Self>())
                .map(|m| {
                    if self.ndim()? == 1 { Ok(m) }
                    else {
                        let mut new_shape = self.get_shape()?;
                        new_shape[axis] /= parts;
                        m.reshape(&new_shape)
                    }
                })
                .collect::<Vec<Result<Self, _>>>();
            if let Err(err) = result.has_error() { Err(err) }
            else { Ok(result.into_iter().map(Result::unwrap).collect()) }
        })
    }

    fn split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Self>, ArrayError> {
        self.axis_opt_in_bounds(axis)?;
        if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            if self.is_empty()? { return Ok(vec![self.clone()]) }
            let n_total = self.shape[axis.unwrap_or(0)];

            if n_total % parts == 0 {
                self.array_split(parts, axis)
            } else {
                Err(ArrayError::ParameterError { param: "parts", message: "array split does not result in an equal division", })
            }
        }
    }

    fn split_axis(&self, axis: usize) -> Result<Vec<Self>, ArrayError> {
        self.axis_in_bounds(axis)?;
        if self.is_empty()? || self.ndim()? == 1 { Ok(vec![self.clone()]) }
        else { self.array_split(self.shape[axis], Some(axis)) }
    }

    fn hsplit(&self, parts: usize) -> Result<Vec<Self>, ArrayError> {
        self.is_dim_unsupported(&[0])?;
        if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            match self.ndim()? {
                1 => self.split(parts, Some(0)),
                _ => self.split(parts, Some(1)),
            }
        }
    }

    fn vsplit(&self, parts: usize) -> Result<Vec<Self>, ArrayError> {
        self.is_dim_unsupported(&[0, 1])?;
        if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            self.split(parts, Some(0))
        }
    }

    fn dsplit(&self, parts: usize) -> Result<Vec<Self>, ArrayError> {
        self.is_dim_unsupported(&[0, 1, 2])?;
        if parts == 0 {
            Err(ArrayError::ParameterError { param: "parts", message: "number of sections must be larger than 0", })
        } else {
            self.split(parts, Some(2))
        }
    }
}

impl <T: ArrayElement> ArraySplit<T> for Result<Array<T>, ArrayError> {

    fn array_split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<T>>, ArrayError> {
        self.clone()?.array_split(parts, axis)
    }

    fn split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<T>>, ArrayError> {
        self.clone()?.split(parts, axis)
    }

    fn split_axis(&self, axis: usize) -> Result<Vec<Array<T>>, ArrayError> {
        self.clone()?.split_axis(axis)
    }

    fn hsplit(&self, parts: usize) -> Result<Vec<Array<T>>, ArrayError> {
        self.clone()?.hsplit(parts)
    }

    fn vsplit(&self, parts: usize) -> Result<Vec<Array<T>>, ArrayError> {
        self.clone()?.vsplit(parts)
    }

    fn dsplit(&self, parts: usize) -> Result<Vec<Array<T>>, ArrayError> {
        self.clone()?.dsplit(parts)
    }
}

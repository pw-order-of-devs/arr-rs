use std::collections::HashMap;

use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Reordering functions
pub trait ArrayReorder<T: ArrayElement> where Self: Sized + Clone {

    /// Reverse the order of elements in an array along the given axis
    ///
    /// # Arguments
    ///
    /// * `axes` - axes along which to flip over. if None, will flip over all of the axes.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// assert_eq!(array!([[[1, 0], [3, 2]], [[5, 4], [7, 6]]]), arr.flip(Some(vec![2])));
    /// ```
    fn flip(&self, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError>;

    /// Reverse the order of elements along axis 0 (up/down)
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// assert_eq!(array!([[[4, 5], [6, 7]], [[0, 1], [2, 3]]]), arr.flipud());
    /// ```
    fn flipud(&self) -> Result<Array<T>, ArrayError>;

    /// Reverse the order of elements along axis 1 (left/right)
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// assert_eq!(array!([[[2, 3], [0, 1]], [[6, 7], [4, 5]]]), arr.fliplr());
    /// ```
    fn fliplr(&self) -> Result<Array<T>, ArrayError>;

    /// Roll array elements along a given axis
    ///
    /// # Arguments
    ///
    /// * `shift` - number of places by which elements are shifted.
    /// if a tuple, then axis must be a tuple of the same size, and each of the given axes is shifted by the corresponding number.
    /// if an int while axis is a tuple of ints, then the same value is used for all given axes.
    /// * `axes` - axes along which to roll over. if None, will flip over all of the axes.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// assert_eq!(array!([[[4, 5], [6, 7]], [[0, 1], [2, 3]]]), arr.roll(vec![1], Some(vec![0])));
    /// ```
    fn roll(&self, shift: Vec<isize>, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError>;

    /// Rotate an array by 90 degrees in the plane specified by axes.
    /// Rotation direction is from the first towards the second axis.
    ///
    /// # Arguments
    ///
    /// * `k` - number of times the array is rotated by 90 degrees.
    /// * `axes` - the array is rotated in the plane defined by the axes. axes must be different.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 4]);
    /// assert_eq!(array!([[3, 7], [2, 6], [1, 5], [0, 4]]), arr.rot90(1, vec![0, 1]));
    /// ```
    fn rot90(&self, k: usize, axes: Vec<isize>) -> Result<Array<T>, ArrayError>;
}

impl <T: ArrayElement> ArrayReorder<T> for Array<T> {

    fn flip(&self, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError> {
        match axes {
            None => {
                let mut flipped_elements = self.get_elements()?;
                flipped_elements.reverse();
                Array::new(flipped_elements, self.get_shape()?)
            },
            Some(axes) => {
                let self_shape = self.shape.clone();
                let axes = axes.into_iter().map(|i| self.normalize_axis(i)).collect::<Vec<usize>>();

                let mut elements = self.elements.clone();
                for ax in axes {
                    let flatten = Array::flat(elements);
                    elements =
                        if ax == 0 { flatten
                            .split(self_shape[0], Some(0))?.reverse_ext().into_iter()
                            .flatten().collect::<Vec<T>>()
                        } else if ax == self.ndim()? - 1 { flatten
                            .split(self_shape[0 .. ax].iter().product(), None)?.iter_mut()
                            .flat_map(|arr| arr.elements.reverse_ext())
                            .collect::<Vec<T>>()
                        } else { flatten
                            .split(self_shape[ax], None)?.into_iter()
                            .map(|i| i.reshape(self.shape.clone().remove_at(ax)))
                            .map(|i| i.flip(Some(vec![ax as isize - 1])))
                            .collect::<Vec<Result<Array<T>, _>>>()
                            .has_error()?.into_iter()
                            .flat_map(|i| i.unwrap())
                            .collect::<Vec<T>>()
                        }
                };

                Array::flat(elements).reshape(self_shape)
            }
        }
    }

    fn flipud(&self) -> Result<Array<T>, ArrayError> {
        self.is_dim_unsupported(&[0])?;
        self.clone().flip(Some(vec![0]))
    }

    fn fliplr(&self) -> Result<Array<T>, ArrayError> {
        self.is_dim_unsupported(&[0, 1])?;
        self.clone().flip(Some(vec![1]))
    }

    fn roll(&self, shift: Vec<isize>, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError> {
        let array = if axes.is_none() { self.ravel()? } else { self.clone() };
        let axes = axes.unwrap_or(vec![0]);

        let broadcasted = Array::flat(shift).broadcast(&Array::flat(axes)?)?;
        if broadcasted.ndim()? > 1 { return Err(ArrayError::ParameterError { param: "'shift' and 'axis'", message: "should be 1D" }); }
        let broadcasted = broadcasted.into_iter().map(|a| (
            self.normalize_axis(a.1),
            a.0,
        )).collect::<Vec<(usize, isize)>>();

        let mut shifts: HashMap<usize, isize> = HashMap::new();
        broadcasted.iter().for_each(|(a, b)| {
            *shifts.entry(*a).or_insert(0) += *b;
        });

        let mut elements = array.get_elements()?;
        match array.ndim()? {
            0 => Array::empty(),
            1 => {
                shifts.iter().for_each(|(_, &sh)| {
                    if sh >= 0 { elements.rotate_right(sh as usize); }
                    else { elements.rotate_left(sh.unsigned_abs()); }
                });
                Array::flat(elements).reshape(self.shape.clone())
            },
            _ => {
                for (ax, sh) in shifts.clone() {
                    let flatten = Array::flat(elements.clone());
                    elements = if ax == 0 {
                        let mut split = flatten.split(self.shape[0], Some(0))?;
                        if sh >= 0 { split.rotate_right(sh as usize); }
                        else { split.rotate_left(sh.unsigned_abs()); }
                        split.into_iter().flatten().collect()
                    } else if ax == array.ndim()? - 1 { flatten
                        .split(self.shape[0 .. ax].iter().product(), None)?.iter()
                        .flat_map(|item| {
                            let mut tmp_item = item.elements.clone();
                            if sh >= 0 { tmp_item.rotate_right(sh as usize); }
                            else { tmp_item.rotate_left(sh.unsigned_abs()); }
                            tmp_item
                        }).collect()
                    } else { flatten
                        .split(self.shape[ax], None)?.into_iter()
                        .map(|i| i.reshape(self.shape.clone().remove_at(ax)))
                        .map(|i| i.roll(vec![shifts[&ax]], Some(vec![ax as isize - 1])))
                        .collect::<Vec<Result<Array<T>, _>>>()
                        .has_error()?.into_iter()
                        .flat_map(|i| i.unwrap())
                        .collect::<Vec<T>>()
                    }
                }
                Array::new(elements, self.shape.clone())
            }
        }
    }

    fn rot90(&self, k: usize, axes: Vec<isize>) -> Result<Array<T>, ArrayError> {
        self.is_dim_unsupported(&[0, 1])?;
        if axes.len() != 2 {
            return Err(ArrayError::ParameterError { param: "axes", message: "axes length must be 2" })
        }
        let self_ndim = self.ndim()? as isize;
        if axes[0] >= self_ndim || axes[0] < -self_ndim || axes[1] >= self_ndim || axes[1] < -self_ndim {
            return Err(ArrayError::ParameterError { param: "axes", message: "out of range" })
        }

        let k = k % 4;
        if k == 0 { return Ok(self.clone()) }
        if k == 2 { return self.flip(Some(vec![axes[1]])).flip(Some(vec![axes[0]])) }

        let axes = axes.into_iter().map(|i| self.normalize_axis(i)).collect::<Vec<usize>>();
        let mut axes_list = (0 .. self_ndim).collect::<Vec<isize>>();
        (axes_list[axes[0]], axes_list[axes[1]]) = (axes_list[axes[1]], axes_list[axes[0]]);

        if k == 1 { self.flip(Some(vec![axes[1] as isize])).transpose(Some(axes_list)) }
        else { self.transpose(Some(axes_list)).flip(Some(vec![axes[1] as isize])) }
    }
}

impl <T: ArrayElement> ArrayReorder<T> for Result<Array<T>, ArrayError> {

    fn flip(&self, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError> {
        self.clone()?.flip(axes)
    }

    fn flipud(&self) -> Result<Array<T>, ArrayError> {
        self.clone()?.flipud()
    }

    fn fliplr(&self) -> Result<Array<T>, ArrayError> {
        self.clone()?.fliplr()
    }

    fn roll(&self, shift: Vec<isize>, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError> {
        self.clone()?.roll(shift, axes)
    }

    fn rot90(&self, k: usize, axes: Vec<isize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.rot90(k, axes)
    }
}

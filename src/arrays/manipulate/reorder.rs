use std::collections::HashMap;
use crate::arrays::Array;
use crate::ext::vec_ext::{VecRemoveAt, VecRevert};
use crate::traits::{
    create::{
        ArrayCreate,
        ArrayCreateNumeric,
    },
    errors::ArrayError,
    manipulate::{
        ArrayManipulate,
        axis::ArrayAxis,
        broadcast::ArrayBroadcast,
        reorder::ArrayReorder,
        split::ArraySplit,
    },
    meta::ArrayMeta,
    types::numeric::Numeric,
    validators::{
        validate_dimension::ValidateDimension,
        validate_has_error::ValidateHasError,
    },
};

impl <N: Numeric> ArrayReorder<N> for Array<N> {

    fn flip(&self, axes: Option<Vec<isize>>) -> Result<Array<N>, ArrayError> {
        match axes {
            None => {
                let mut flipped_elements = self.get_elements()?;
                flipped_elements.reverse();
                Array::new(flipped_elements, self.get_shape()?)
            },
            Some(axes) => {
                let self_ndim = self.ndim()?;
                let self_shape = self.shape.clone();
                let axes = axes.into_iter().map(|i| Self::normalize_axis(i, self_ndim)).collect::<Vec<usize>>();

                let mut elements = self.elements.clone();
                for ax in axes {
                    let flatten = Array::flat(elements);
                    elements =
                        if ax == 0 { flatten
                            .split(self_shape[0], Some(0))?.reverse_ext().into_iter()
                            .flatten().collect::<Vec<N>>()
                        } else if ax == self_ndim - 1 { flatten
                            .split(self_shape[0 .. ax].iter().product(), None)?.iter_mut()
                            .flat_map(|arr| arr.elements.reverse_ext())
                            .collect::<Vec<N>>()
                        } else { flatten
                            .split(self_shape[ax], None)?.into_iter()
                            .map(|i| i.reshape(self.shape.clone().remove_at(ax)))
                            .map(|i| i.flip(Some(vec![ax as isize - 1])))
                            .collect::<Vec<Result<Array<N>, _>>>()
                            .has_error()?.into_iter()
                            .flat_map(|i| i.unwrap())
                            .collect::<Vec<N>>()
                        }
                    };

                Array::flat(elements).reshape(self_shape)
            }
        }
    }

    fn flipud(&self) -> Result<Array<N>, ArrayError> {
        self.is_dim_unsupported(&[0])?;
        self.clone().flip(Some(vec![0]))
    }

    fn fliplr(&self) -> Result<Array<N>, ArrayError> {
        self.is_dim_unsupported(&[0, 1])?;
        self.clone().flip(Some(vec![1]))
    }

    fn roll(&self, shift: Vec<isize>, axes: Option<Vec<isize>>) -> Result<Array<N>, ArrayError> {
        let array = if axes.is_none() { self.ravel()? } else { self.clone() };
        let axes = axes.unwrap_or(vec![0]);

        let self_ndim = array.ndim()?;
        let broadcasted = Array::flat(shift).broadcast(&Array::flat(axes)?)?;
        if broadcasted.ndim()? > 1 { return Err(ArrayError::ParameterError { param: "'shift' and 'axis'", message: "should be 1D" }); }
        let broadcasted = broadcasted.into_iter().map(|a| (
            Self::normalize_axis(a.1, self_ndim),
            a.0,
        )).collect::<Vec<(usize, isize)>>();

        let mut shifts: HashMap<usize, isize> = HashMap::new();
        broadcasted.iter().for_each(|(a, b)| {
            *shifts.entry(*a).or_insert(0) += *b;
        });

        let mut elements = array.get_elements()?;
        match self_ndim {
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
                    } else if ax == self_ndim - 1 { flatten
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
                        .collect::<Vec<Result<Array<N>, _>>>()
                        .has_error()?.into_iter()
                        .flat_map(|i| i.unwrap())
                        .collect::<Vec<N>>()
                    }
                }
                Array::new(elements, self.shape.clone())
            }
        }
    }

    fn rot90(&self, k: usize, axes: Vec<isize>) -> Result<Array<N>, ArrayError> {
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

        let axes = axes.into_iter().map(|i| Self::normalize_axis(i, self_ndim as usize)).collect::<Vec<usize>>();
        let mut axes_list = Array::arange(0, self_ndim - 1, None)?.get_elements()?;
        (axes_list[axes[0]], axes_list[axes[1]]) = (axes_list[axes[1]], axes_list[axes[0]]);

        if k == 1 { self.flip(Some(vec![axes[1] as isize])).transpose(Some(axes_list)) }
        else { self.transpose(Some(axes_list)).flip(Some(vec![axes[1] as isize])) }
    }
}

#[cfg(test)] mod test {
    use crate::prelude::*;

    #[test] fn test() {
        let arr: Result<Array<i32>, _> = array!([[1, 2], [3, 4]]);
        let res = arr.rot90(1, vec![0, 1, 2]);
        println!("{res:?}");
    }
}

impl <N: Numeric> ArrayReorder<N> for Result<Array<N>, ArrayError> {

    fn flip(&self, axes: Option<Vec<isize>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.flip(axes)
    }

    fn flipud(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.flipud()
    }

    fn fliplr(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.fliplr()
    }

    fn roll(&self, shift: Vec<isize>, axes: Option<Vec<isize>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.roll(shift, axes)
    }

    fn rot90(&self, k: usize, axes: Vec<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.rot90(k, axes)
    }
}

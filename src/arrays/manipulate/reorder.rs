use crate::arrays::Array;
use crate::ext::vec_ext::{VecRemoveAt, VecRevert};
use crate::traits::{
    create::ArrayCreate,
    errors::ArrayError,
    manipulate::{
        ArrayManipulate,
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

                let mut indexer: Vec<(Option<isize>, Option<isize>, Option<isize>)> = vec![(None, None, None); self_ndim];
                axes.iter().for_each(|&i| indexer[i] = (None, None, Some(-1)));

                let mut elements = self.elements.clone();

                for ax in axes {
                    elements = if ax == 0 {
                        Array::flat(elements)
                            .split(self_shape[0], Some(ax))?.reverse_ext().into_iter()
                            .flatten().collect::<Vec<N>>()
                    } else if ax == self_ndim - 1 {
                        Array::flat(elements)
                            .split(self_shape[0 .. ax].iter().product(), None)?.iter_mut()
                            .flat_map(|arr| arr.elements.reverse_ext())
                            .collect::<Vec<N>>()
                    } else {
                        Array::flat(elements)
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
}

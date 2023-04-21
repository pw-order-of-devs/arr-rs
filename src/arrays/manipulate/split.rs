use crate::arrays::Array;
use crate::ext::vec_ext::VecInsertAt;
use crate::traits::{
    create::ArrayCreate,
    indexing::ArrayIndexing,
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

    fn array_split(&self, parts: usize, axis: Option<usize>) -> Vec<Self> {
        assert!(parts > 0, "number of sections must be larger than 0");
        if let Some(axis) = axis { assert!(axis < self.ndim(), "axis out of bounds"); }

        if self.is_empty() { return vec![self.clone()] }

        let axis = axis.unwrap_or(0);
        let n_total = self.get_shape()[axis];

        let (sections, extras) = (n_total / parts, n_total % parts);
        let section_sizes = std::iter::repeat(sections + 1)
            .take(extras)
            .chain(std::iter::repeat(sections).take(parts - extras))
            .collect::<Vec<usize>>()
            .insert_at(0, 0);
        let div_points = Array::flat(section_sizes).cumsum().get_elements();

        let arr = self.swapaxes(axis as isize, 0);
        div_points
            .windows(2)
            .map(|w| {
                let sub_shape = self.shape.iter().enumerate()
                    .map(|(i, &x)| if i == axis { w[1] - w[0] } else { x })
                    .filter(|&x| x > 0)
                    .collect();
                arr.slice(w[0] .. w[1]).reshape(sub_shape)
            })
            .collect()
    }

    fn split(&self, parts: usize, axis: Option<usize>) -> Vec<Self> {
        assert!(parts > 0, "number of sections must be larger than 0");
        if let Some(axis) = axis { assert!(axis < self.ndim(), "axis out of bounds"); }

        if self.is_empty() { return vec![self.clone()] }
        let n_total = self.shape[axis.unwrap_or(0)];

        assert_eq!(0, n_total % parts, "array split does not result in an equal division");
        self.array_split(parts, axis)
    }
}

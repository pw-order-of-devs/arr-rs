use crate::arrays::Array;
use crate::ext::vec_ext::{VecInsertAt, VecRemoveAt};
use crate::traits::{
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
        div_points
            .windows(2)
            .map(|w| arr.clone().into_iter()
                .skip(w[0]).take(w[1] - w[0])
                .collect::<Self>())
            .map(|m| {
                if self.ndim() == 1 { m }
                else {
                    let mut new_shape = self.get_shape();
                    new_shape[axis] /= self.get_shape().remove_at(axis).iter().product::<usize>();
                    if new_shape[axis] == 0 { new_shape[axis] = 1 }
                    m.reshape(new_shape)
                }
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

    fn hsplit(&self, parts: usize) -> Vec<Self> {
        assert!(self.ndim() > 0, "hsplit only works on arrays of 1 or more dimensions");
        assert!(parts > 0, "number of sections must be larger than 0");

        match self.ndim() {
            1 => self.split(parts, Some(0)),
            _ => self.split(parts, Some(1)),
        }
    }

    fn vsplit(&self, parts: usize) -> Vec<Self> {
        assert!(self.ndim() > 1, "vsplit only works on arrays of 2 or more dimensions");
        assert!(parts > 0, "number of sections must be larger than 0");

        self.split(parts, Some(0))
    }

    fn dsplit(&self, parts: usize) -> Vec<Self> {
        assert!(self.ndim() > 2, "dsplit only works on arrays of 3 or more dimensions");
        assert!(parts > 0, "number of sections must be larger than 0");

        self.split(parts, Some(2))
    }
}

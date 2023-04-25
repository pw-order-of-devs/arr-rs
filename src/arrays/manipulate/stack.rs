use crate::arrays::Array;
use crate::ext::vec_ext::{VecInsertAt, VecRemoveAt};
use crate::traits::{
    create::ArrayCreate,
    manipulate::{
        ArrayManipulate,
        stack::ArrayStack,
    },
    meta::ArrayMeta,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayStack<N> for Array<N> {

    fn concatenate(arrs: Vec<Self>, axis: Option<usize>) -> Self {
        if arrs.is_empty() { return Self::empty() }
        if let Some(axis) = axis {
            arrs.iter().for_each(|arr| assert!(axis < arr.ndim(), "axis out of bounds"));
            (0 .. arrs.len() - 1).for_each(|i| {
                let shape_1 = arrs[i].get_shape().remove_at(axis);
                let shape_2 = arrs[i + 1].get_shape().remove_at(axis);
                assert_eq!(shape_1, shape_2, "incompatible shapes for concatenate")
            });
        }

        let (mut arrs, initial) = (arrs.clone(), arrs[0].clone());
        arrs.remove_at(0).into_iter()
            .fold(initial, |a, b| a.append(&b, axis))
    }

    fn stack(arrs: Vec<Self>, axis: Option<usize>) -> Self {
        if arrs.is_empty() { return Self::empty() }
        if let Some(axis) = axis { arrs.iter().for_each(|arr| assert!(axis < arr.ndim(), "axis out of bounds")); }
        (0 .. arrs.len() - 1).for_each(|i| { assert_eq!(arrs[i].get_shape(), arrs[i + 1].get_shape(), "all input arrays must have the same shape") });

        let axis = axis.unwrap_or(0);
        let new_shape = arrs[0].get_shape().insert_at(axis, arrs.len());

        let (mut arrs, initial) = (arrs.clone(), arrs[0].clone());
        arrs.remove_at(0).into_iter()
            .fold(initial, |a, b| a.append(&b, Some(axis)))
            .reshape(new_shape)
    }
}

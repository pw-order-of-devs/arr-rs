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
        if arrs.is_empty() { Self::empty() }
        else {
            if let Some(axis) = axis { Self::validate_stack_shapes(&arrs, axis, axis) }

            let (mut arrs, initial) = (arrs.clone(), arrs[0].clone());
            arrs.remove_at(0).into_iter()
                .fold(initial, |a, b| a.append(&b, axis))
        }
    }

    fn stack(arrs: Vec<Self>, axis: Option<usize>) -> Self {
        if arrs.is_empty() { Self::empty() }
        else {
            if let Some(axis) = axis { arrs.iter().for_each(|arr| assert!(axis < arr.ndim(), "axis out of bounds")); }
            (0..arrs.len() - 1).for_each(|i| { assert_eq!(arrs[i].get_shape(), arrs[i + 1].get_shape(), "all input arrays must have the same shape") });

            let axis = axis.unwrap_or(0);
            let new_shape = arrs[0].get_shape().insert_at(axis, arrs.len());

            let (mut arrs, initial) = (arrs.clone(), arrs[0].clone());
            arrs.remove_at(0).into_iter()
                .fold(initial, |a, b| a.append(&b, Some(axis)))
                .reshape(new_shape)
        }
    }

    fn vstack(arrs: Vec<Self>) -> Self {
        if arrs.is_empty() { Self::empty() }
        else {
            Self::validate_stack_shapes(&arrs, 0, 0);
            let mut new_shape = arrs[0].get_shape();
            if new_shape.len() == 1 { new_shape.insert_at(0, arrs.len()); }
            else { new_shape[0] = arrs.iter().fold(0, |a, b| a + b.shape[0]); }

            Self::concatenate(arrs, Some(0)).reshape(new_shape)
        }
    }

    fn hstack(arrs: Vec<Self>) -> Self {
        if arrs.is_empty() { Self::empty() }
        else if arrs.iter().all(|arr| arr.ndim() == 1) {
            Self::concatenate(arrs, Some(0))
        } else {
            let arrs = arrs.iter().map(|arr| arr.atleast(2)).collect::<Vec<Self>>();
            Self::validate_stack_shapes(&arrs, 1, 0);
            let mut new_shape = arrs[0].get_shape();
            new_shape[1] = arrs.iter().fold(0, |a, b| a + b.shape[1]);

            Self::concatenate(arrs, Some(1)).reshape(new_shape)
        }
    }

    fn dstack(arrs: Vec<Self>) -> Self {
        if arrs.is_empty() { Self::empty() }
        else {
            let arrs = arrs.iter().map(|arr| arr.atleast(3)).collect::<Vec<Self>>();
            Self::validate_stack_shapes(&arrs, 2, 0);
            let mut new_shape = arrs[0].get_shape();
            new_shape[2] = arrs.iter().fold(0, |a, b| a + b.shape[2]);

            Self::concatenate(arrs, Some(2)).reshape(new_shape)
        }
    }
}

impl <N: Numeric> Array<N> {

    fn validate_stack_shapes(arrs: &Vec<Self>, axis: usize, remove_at: usize) {
        arrs.iter().for_each(|arr| assert!(axis < arr.ndim(), "axis out of bounds"));
        (0 .. arrs.len() - 1).for_each(|i| {
            let shape_1 = arrs[i].get_shape().remove_at(remove_at);
            let shape_2 = arrs[i + 1].get_shape().remove_at(remove_at);
            assert_eq!(shape_1, shape_2, "incompatible shapes for concatenate")
        });
    }
}

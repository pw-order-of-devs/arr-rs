use crate::arrays::Array;
use crate::ext::vec_ext::{VecRemoveAt, VecRevert};
use crate::prelude::ArrayAxis;
use crate::traits::{
    errors::ArrayError,
    binary::{
        ArrayBinary,
        ArrayBinaryBits,
        BitOrder,
    },
    create::ArrayCreate,
    indexing::ArrayIndexing,
    manipulate::{
        ArrayManipulate,
        broadcast::ArrayBroadcast,
        split::ArraySplit,
    },
    meta::ArrayMeta,
    types::numeric::Numeric,
    validators::{
        validate_has_error::ValidateHasError,
        validate_shape::ValidateShape,
    },
};

impl <N: Numeric> ArrayBinary<N> for Array<N> {
    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.bitwise_and(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.bitwise_or(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let elements = self.broadcast(other)?.into_iter()
            .map(|tuple| tuple.0.bitwise_xor(&tuple.1))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn bitwise_not(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.into_iter()
            .map(|&a| a.bitwise_not())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn invert(&self) -> Result<Array<N>, ArrayError> {
        self.bitwise_not()
    }

    fn left_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.left_shift(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn right_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.right_shift(&tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn binary_repr(num: N) -> String {
        num.binary_repr()
    }
}

impl ArrayBinaryBits for Array<u8> {

    fn unpack_bits(&self, axis: Option<isize>, count: Option<isize>, bit_order: Option<BitOrder>) -> Result<Array<u8>, ArrayError> {
        if self.is_empty()? { return Array::empty() }
        let bit_order = bit_order.unwrap_or(BitOrder::Big);
        match axis {
            None => {
                let result = self.ravel()?
                    .into_iter()
                    .flat_map(|a| {
                        let mut elems = (0..8).rev().map(move |idx| (a >> idx) & 1).collect::<Vec<u8>>();
                        if bit_order == BitOrder::Little { elems.reverse_ext() } else { elems }
                    })
                    .collect::<Array<u8>>();
                let count = count.unwrap_or(self.len()? as isize * 8);
                if count >= 0 { result.slice(0 .. count as usize) }
                else { result.slice(0 .. self.len()? - count as usize) }
            },
            Some(axis) => {
                let axis = Self::normalize_axis(axis, self.ndim()?);
                let mut new_shape = self.get_shape()?;
                new_shape[axis] *= 8;

                if axis < self.ndim()? - 1 {
                    let (parts, shape_last) = (self.get_shape()?.iter().product::<usize>(), self.ndim()? - 1);
                    let result = self.unpack_bits(None, count, Some(bit_order))?;
                    if self.shape[shape_last] == 1 { result }
                    else {
                        result.split(parts / shape_last, None)?
                            .into_iter()
                            .flat_map(|arr| (0..8)
                                .flat_map(move |i| (0..shape_last).map(move |ii| i + 8 * ii))
                                .map(move |index| arr[index]))
                            .collect::<Array<u8>>()
                    }
                } else {
                    let (parts, shape_first) = (self.get_shape()?.iter().product(), self.get_shape()?[0]);
                    let split = self.split(self.shape[axis], Some(axis))?.into_iter()
                        .map(|arr| arr.unpack_bits(None, count, Some(bit_order)))
                        .collect::<Vec<Result<Array<u8>, _>>>()
                        .has_error()?.into_iter()
                        .map(|arr| arr.unwrap())
                        .collect::<Vec<Array<u8>>>()
                        .into_iter().flatten().collect::<Array<u8>>()
                        .split(parts, None)?;
                    (0 .. shape_first)
                        .flat_map(|i| (0 .. parts / shape_first).map(move |ii| i + shape_first * ii))
                        .flat_map(|index| split[index].elements.clone())
                        .collect()
                }.reshape(new_shape)
            }
        }
    }

    fn pack_bits(&self, axis: Option<isize>, bit_order: Option<BitOrder>) -> Result<Array<u8>, ArrayError> {
        if self.is_empty()? { return Array::empty() }
        let bit_order = bit_order.unwrap_or(BitOrder::Big);
        match axis {
            None => {
                let mut elements = self.get_elements()?;
                if elements.len() % 8 != 0 { elements.extend_from_slice(&vec![0; 8 - elements.len() % 8]) }

                let parts = elements.len() / 8;
                let result = (0 .. parts).map(|p| {
                    let subarray = elements[p * 8 .. (p + 1) * 8].iter()
                        .map(|i| if i > &0 { "1" } else { "0" })
                        .collect::<Vec<&str>>()
                        .join("");
                    let subarray = if bit_order == BitOrder::Little { subarray.chars().rev().collect() } else { subarray };
                    u8::from_str_radix(&subarray, 2).unwrap()
                }).collect();
                Ok(result)
            },
            Some(axis) => {
                let mut new_shape = self.get_shape()?;
                let parts = self.get_shape()?.into_iter().rev().collect::<Vec<usize>>();
                let axis = Self::normalize_axis(axis, self.ndim()?);
                new_shape[axis] = 1;
                let split = if axis == self.ndim()? - 1 {
                    self.split(parts[axis], Some(self.ndim()? - 1 - axis))?
                } else {
                    let parts = self.get_shape()?.remove_at(axis).into_iter().product();
                    self.moveaxis(vec![axis as isize], vec![self.ndim()? as isize])
                        .ravel().split(parts, None)?
                };
                split.into_iter()
                    .map(|arr| arr.pack_bits(None, Some(bit_order)).get_elements())
                    .collect::<Vec<Result<Vec<u8>, _>>>()
                    .has_error()?.into_iter()
                    .flat_map(|arr| arr.unwrap())
                    .collect::<Array<u8>>()
                    .reshape(new_shape)
            },
        }
    }
}

impl <N: Numeric> ArrayBinary<N> for Result<Array<N>, ArrayError> {
    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_and(other)
    }

    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_or(other)
    }

    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_xor(other)
    }

    fn bitwise_not(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_not()
    }

    fn invert(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.invert()
    }

    fn left_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.left_shift(other)
    }

    fn right_shift(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.right_shift(other)
    }

    fn binary_repr(num: N) -> String {
        num.binary_repr()
    }
}

impl ArrayBinaryBits for Result<Array<u8>, ArrayError> {

    fn unpack_bits(&self, axis: Option<isize>, count: Option<isize>, bit_order: Option<BitOrder>) -> Result<Array<u8>, ArrayError> {
        self.clone()?.unpack_bits(axis, count, bit_order)
    }

    fn pack_bits(&self, axis: Option<isize>, bit_order: Option<BitOrder>) -> Result<Array<u8>, ArrayError> {
        self.clone()?.pack_bits(axis, bit_order)
    }
}

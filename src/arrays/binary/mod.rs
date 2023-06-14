use crate::arrays::Array;
use crate::ext::vec_ext::VecRevert;
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
        let array_u8 = self.into_iter().map(|i| i.to_usize() as u8).collect::<Array<u8>>();
        let bit_order = bit_order.unwrap_or(BitOrder::Big);
        match axis {
            None => {
                let result = array_u8.ravel()?
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
                    let split = self.split(self.shape[axis], Some(axis))?
                        .into_iter()
                        .map(|arr| arr.unpack_bits(None, count, Some(bit_order)))
                        .collect::<Vec<Result<Array<u8>, _>>>()
                        .has_error()?
                        .into_iter()
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
}

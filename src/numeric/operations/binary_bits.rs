use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::vec_ext::{VecRemoveAt, VecReverse},
    numeric::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Binary Array bits operations
pub trait ArrayBinaryBits where Self: Sized + Clone {

    /// Unpacks elements of a uint8 array into a binary-valued output array
    ///
    /// # Arguments
    ///
    /// * `axis` - the dimension over which bit-unpacking is done. if none, array is flattened
    /// * `count` - the number of elements to unpack along axis. if negative, array is trimmed
    /// * `bit_order` - the order of the returned bits. defaults to `Big`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Result<Array<u8>, _> = array!([[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 0, 1, 0, 1, 1, 1]]);
    /// let array: Result<Array<u8>, _> = array!([[2], [7], [23]]);
    /// assert_eq!(expected, array.unpack_bits(Some(1), None, None));
    /// ```
    fn unpack_bits(&self, axis: Option<isize>, count: Option<isize>, bit_order: Option<BitOrder>) -> Result<Array<u8>, ArrayError>;

    /// Packs the elements of a binary-valued array into bits in a uint8 array
    ///
    /// # Arguments
    ///
    /// * `axis` - the dimension over which bit-packing is done. if none, array is flattened
    /// * `bit_order` - the order of the returned bits. defaults to `Big`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Result<Array<u8>, _> = array!([[2], [7], [23]]);
    /// let array: Result<Array<u8>, _> = array!([[0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 0, 1, 0, 1, 1, 1]]);
    /// assert_eq!(expected, array.pack_bits(Some(1), None));
    /// ```
    fn pack_bits(&self, axis: Option<isize>, bit_order: Option<BitOrder>) -> Result<Array<u8>, ArrayError>;
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

impl ArrayBinaryBits for Result<Array<u8>, ArrayError> {

    fn unpack_bits(&self, axis: Option<isize>, count: Option<isize>, bit_order: Option<BitOrder>) -> Result<Array<u8>, ArrayError> {
        self.clone()?.unpack_bits(axis, count, bit_order)
    }

    fn pack_bits(&self, axis: Option<isize>, bit_order: Option<BitOrder>) -> Result<Array<u8>, ArrayError> {
        self.clone()?.pack_bits(axis, bit_order)
    }
}

use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    numeric::prelude::*,
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
                let axis = self.normalize_axis(axis);
                self.apply_along_axis(axis, |arr| arr.unpack_bits(None, count, Some(bit_order)))
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
                let axis = self.normalize_axis(axis);
                self.apply_along_axis(axis, |arr| arr.pack_bits(None, Some(bit_order)))
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

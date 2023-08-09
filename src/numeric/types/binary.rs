use crate::errors::prelude::*;

/// the order of the bits packed/unpacked
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum BitOrder {
    /// standard binary representation
    Big,
    /// reversed order
    Little,
}

/// BitOrder trait
pub trait BitOrderType {

    /// Parse input to BitOrder type
    fn to_bit_order(self) -> Result<BitOrder, ArrayError>;
}

impl BitOrderType for BitOrder {

    fn to_bit_order(self) -> Result<BitOrder, ArrayError> {
        Ok(self)
    }
}
impl BitOrderType for &str {

    fn to_bit_order(self) -> Result<BitOrder, ArrayError> {
        match self {
            "big" => Ok(BitOrder::Big),
            "little" => Ok(BitOrder::Little),
            _ => Err(ArrayError::ParameterError { param: "`bit_order`", message: "must be one of {`big`, `little`}" })
        }
    }
}
impl BitOrderType for String {

    fn to_bit_order(self) -> Result<BitOrder, ArrayError> {
        match self.as_str() {
            "big" => Ok(BitOrder::Big),
            "little" => Ok(BitOrder::Little),
            _ => Err(ArrayError::ParameterError { param: "`bit_order`", message: "must be one of {`big`, `little`}" })
        }
    }
}

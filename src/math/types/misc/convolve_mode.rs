use crate::errors::prelude::*;

/// the mode of convolve operation
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ConvolveMode {
    /// full
    Full,
    /// valid
    Valid,
    /// same
    Same,
}

/// ConvolveMode trait
pub trait ConvolveModeType {

    /// Parse input to ConvolveMode type
    fn to_mode(self) -> Result<ConvolveMode, ArrayError>;
}

impl ConvolveModeType for ConvolveMode {

    fn to_mode(self) -> Result<ConvolveMode, ArrayError> {
        Ok(self)
    }
}

impl ConvolveModeType for &str {

    fn to_mode(self) -> Result<ConvolveMode, ArrayError> {
        match self {
            "full" => Ok(ConvolveMode::Full),
            "valid" => Ok(ConvolveMode::Valid),
            "same" => Ok(ConvolveMode::Same),
            _ => Err(ArrayError::ParameterError { param: "`mode`", message: "must be one of {`full`, `valid`, `same`}" })
        }
    }
}

impl ConvolveModeType for String {

    fn to_mode(self) -> Result<ConvolveMode, ArrayError> {
        match self.as_str() {
            "full" => Ok(ConvolveMode::Full),
            "valid" => Ok(ConvolveMode::Valid),
            "same" => Ok(ConvolveMode::Same),
            _ => Err(ArrayError::ParameterError { param: "`mode`", message: "must be one of {`full`, `valid`, `same`}" })
        }
    }
}

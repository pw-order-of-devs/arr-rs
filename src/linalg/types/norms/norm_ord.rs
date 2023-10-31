use std::str::FromStr;

use crate::errors::prelude::*;

/// the ord of norm operation
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum NormOrd {
    /// integer
    Int(i32),
    /// infinite
    Inf,
    /// negative infinite
    NegInf,
    /// Frobenius norm
    Fro,
    /// Nuclear norm
    Nuc,
}

/// NormOrd trait
pub trait NormOrdType: Clone {

    /// Parse input to NormOrd type
    fn to_ord(self) -> Result<NormOrd, ArrayError>;
}

impl NormOrdType for NormOrd {
    fn to_ord(self) -> Result<NormOrd, ArrayError> {
        Ok(self)
    }
}

impl NormOrdType for &str {

    fn to_ord(self) -> Result<NormOrd, ArrayError> {
        parse_ord(self)
    }
}

impl NormOrdType for String {

    fn to_ord(self) -> Result<NormOrd, ArrayError> {
        parse_ord(&self)
    }
}

fn parse_ord(value: &str) -> Result<NormOrd, ArrayError> {
    match value.to_lowercase().as_str() {
        "inf" => Ok(NormOrd::Inf),
        "-inf" => Ok(NormOrd::NegInf),
        "fro" => Ok(NormOrd::Fro),
        "nuc" => Ok(NormOrd::Nuc),
        _ => match i32::from_str(value) {
            Ok(ord) => Ok(NormOrd::Int(ord)),
            Err(_) => Err(ArrayError::ParameterError { param: "`mode`", message: "must be one of {`inf`, `-inf`, `fro`, `nuc`} or an integer" })
        }
    }
}

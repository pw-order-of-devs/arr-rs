use crate::traits::types::ArrayElement;

/// Alphanumeric trait for array
pub trait Alphanumeric: ArrayElement {}

macro_rules! impl_alphanumeric {
    ($t:ty) => {
        impl ArrayElement for $t {}

        impl Alphanumeric for $t {}
    }
}

impl_alphanumeric!(String);
impl_alphanumeric!(&str);
impl_alphanumeric!(char);

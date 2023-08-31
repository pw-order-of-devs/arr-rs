use crate::prelude::Numeric;

/// Floating type for array
pub trait Floating: Numeric {}

impl Floating for f64 {}
impl Floating for f32 {}

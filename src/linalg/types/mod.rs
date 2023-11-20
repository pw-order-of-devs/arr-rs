use crate::prelude::{Array, ArrayError};

/// Norms parameters types definition
pub mod norms;

/// Tensor parameters types definition
pub mod tensor;

/// qr decomposition result type definition
pub type LinalgResult<N> = Result<Vec<(Array<N>, Array<N>)>, ArrayError>;

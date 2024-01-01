use crate::prelude::{Array, ArrayError};

/// Norms parameters types definition
pub mod norms;

/// Tensor parameters types definition
pub mod tensor;

/// decomposition result type definition
pub type DecompResult<N> = Result<(Array<N>, Array<N>), ArrayError>;
/// eigen result type definition
pub type EigenResult<N> = Result<Vec<(Array<N>, Array<N>)>, ArrayError>;

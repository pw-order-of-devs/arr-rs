use crate::prelude::{Array, ArrayError};

/// Norms parameters types definition
pub mod norms;

/// qr decomposition result type definition
pub type LinalgResult<N> = Result<Vec<(Array<N>, Array<N>)>, ArrayError>;

use crate::prelude::{Array, ArrayElement, ArrayError};

/// Norms parameters types definition
pub mod norms;

/// Tensor parameters types definition
pub mod tensor;

/// eigen result data definition
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct EigenData<N: ArrayElement> {
    /// eigenvalues
    pub values: Array<N>,
    /// eigenvectors
    pub vectors: Array<N>,
}
/// eigen result type definition
pub type EigenResult<N> = Result<EigenData<N>, ArrayError>;

/// decomposition - QR result data type definition
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct QrData<N: ArrayElement> {
    /// Q matrix
    pub q: Array<N>,
    /// R matrix
    pub r: Array<N>,
}
/// decomposition - QR result type definition
pub type QrResult<N> = Result<QrData<N>, ArrayError>;

/// decomposition - SVD result data type definition
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct SvdData<N: ArrayElement> {
    /// U matrix
    pub u: Array<N>,
    /// S matrix
    pub s: Array<N>,
    /// Vt matrix
    pub vt: Array<N>,
}
/// decomposition - SVD result type definition
pub type SvdResult<N> = Result<SvdData<N>, ArrayError>;

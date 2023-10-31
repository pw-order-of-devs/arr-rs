#[cfg(feature = "linalg")]
pub use crate::linalg::{
    operations::{
        decompositions::ArrayLinalgDecompositions,
        eigen::ArrayLinalgEigen,
        norms::ArrayLinalgNorms,
        products::ArrayLinalgProducts,
        solving_inverting::ArrayLinalgSolvingInvertingProducts,
    },
    types::{
        norms::norm_ord::{NormOrd, NormOrdType},
        LinalgResult,
    },
};

pub(crate) use crate::linalg::operations::common::LinalgHelper;

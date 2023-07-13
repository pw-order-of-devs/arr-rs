#[cfg(feature = "math")]
pub use crate::math::{
    operations::{
        sum_prod_diff::ArraySumProdDiff,
        hyperbolic::ArrayHyperbolic,
        misc::ArrayMathMisc,
        rounding::ArrayRounding,
        trigonometric::ArrayTrigonometric,
    },
};

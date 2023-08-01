#[cfg(feature = "math")]
pub use crate::math::{
    operations::{
        arithmetic::ArrayArithmetic,
        exp_log::ArrayExpLog,
        hyperbolic::ArrayHyperbolic,
        misc::ArrayMathMisc,
        rounding::ArrayRounding,
        special::ArrayMathSpecial,
        sum_prod_diff::ArraySumProdDiff,
        trigonometric::ArrayTrigonometric,
    },
};

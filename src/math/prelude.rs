#[cfg(feature = "math")]
pub use crate::math::{
    operations::{
        arithmetic::ArrayArithmetic,
        exp_log::ArrayExpLog,
        floating::ArrayFloating,
        hyperbolic::ArrayHyperbolic,
        misc::ArrayMathMisc,
        rational::ArrayRational,
        rounding::ArrayRounding,
        special::ArrayMathSpecial,
        sum_prod_diff::ArraySumProdDiff,
        trigonometric::ArrayTrigonometric,
    },
};

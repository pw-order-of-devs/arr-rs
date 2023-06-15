// traits traits
pub use crate::traits::{
    errors::ArrayError,
    binary::{
        ArrayBinary,
        ArrayBinaryBits,
        BitOrder,
    },
    create::{
        ArrayCreate,
        ArrayCreateFrom,
        ArrayCreateNumeric,
    },
    indexing::ArrayIndexing,
    manipulate::{
        ArrayManipulate,
        axis::ArrayAxis,
        broadcast::ArrayBroadcast,
        iter::ArrayIter,
        reorder::ArrayReorder,
        split::ArraySplit,
        stack::ArrayStack,
    },
    math::ArrayMath,
    meta::ArrayMeta,
    types::{
        ArrayElement,
        numeric::Numeric,
        numeric_ops::NumericOps,
        signed_numeric::SignedNumeric,
        bool_numeric::BoolNumeric,
        tuple_numeric::{Tuple2, TupleNumeric},
    },
};

// implementations
pub use crate::arrays::Array;

// macros
#[cfg(feature = "macros")]
pub use crate::{
    array,
    array_arange,
    array_rand,
    array_flat,
    array_eye,
    array_identity,
    array_zeros,
    array_ones,
    array_full,
};

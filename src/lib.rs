//! arr-rs - implementation of multidimensional array in Rust
//! # Examples
//!
//! ```ignore
//! // import the crate
//! use arr_rs::prelude::*;
//!
//! // create an array: (4 elements, 2 dimensions)
//! let arr = Array::<i32>::new(vec![1, 2, 3, 4], vec![2, 2]);
//!
//! // create same array using macro:
//! let arr: Array::<i32> = array!([[1, 2], [3, 4]]);
//!
//! // create random array with the same shape:
//! let arr = Array::<i32>::rand(vec![2, 2]);
//!
//! // array supports display and pretty display
//! let arr: Array<f64> = array!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
//! println!("{array}");
//! println!("{array:#}");
//!
//! // perform some chained operations on array:
//! let res = arr
//!     .map(|item| item * 2)
//!     .filter(|item| item % 3 == 0)
//!     .ravel()
//!     .slice(0..2);
//! ```
//!
//! # Crate Features
//! - `macros` - create array macro (enabled by default)
//!

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::redundant_pub_crate)]
#![allow(clippy::similar_names)]
#![cfg_attr(test, deny(rust_2018_idioms))]

/// core functionality - common for modules
pub mod core;

/// alphanumeric module implementation
#[cfg(feature = "alphanumeric")]
pub mod alphanumeric;

/// boolean module implementation
#[cfg(feature = "boolean")]
pub mod boolean;

/// linear algebra module implementation
#[cfg(feature = "linalg")]
pub mod linalg;

/// math module implementation
#[cfg(feature = "math")]
pub mod math;

/// numeric module implementation
#[cfg(feature = "numeric")]
pub mod numeric;

/// array error definitions
pub mod errors;

/// create array macro implementation
#[cfg(feature = "macros")]
pub mod macros;

/// prelude module - imports facade
pub mod prelude;

/// extension functions
pub(crate) mod extensions;

/// array validators definitions
pub(crate) mod validators;

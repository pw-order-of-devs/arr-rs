//! arr-rs - implementation of multidimensional array in Rust

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

/// prelude module - imports facade
pub mod prelude;

/// base modules - trait definitions
pub mod base;
/// implementation modules - array implementation
pub mod arrays;

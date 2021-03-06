#![deny(
    missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications, unused_variables, unreachable_code, unused_comparisons, unused_imports,
    unused_must_use
)]

//! This crate provides a suite of cryptographic services. It brings together numerious other quality crypto crates in addition to local implementations, unifying them all through a common interface.

#[macro_use]
extern crate error_chain;

pub mod asymmetric;
pub mod elliptic_curve;
pub mod errors;
pub mod pki;

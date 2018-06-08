#![deny(
    missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications, unused_variables, unreachable_code, unused_comparisons, unused_imports,
    unused_must_use
)]

//! This crate provides a suite of cryptographic services. It brings together numerious other quality crypto crates in addition to local implementations, unifying them all through a common interface.

#[macro_use]
extern crate error_chain;

/// Errors (uses error-chain)
pub mod errors;

/// Asymmetric crypto stubs for use within specific implementations, intended for internal usage but exposed to enable extension of this crate.
#[cfg(feature = "asymmetric")]
pub mod asymmetric;

/// Elliptic Curve Cryptography
#[cfg(feature = "elliptic_curve")]
pub mod elliptic_curve;

/// Public Key Infrastructure (certificates, et al.)
#[cfg(feature = "pki")]
pub mod pki;

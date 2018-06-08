//! Elliptic Curve Cryptography
//!
//! This module encapsulates the data structures and methods related to elliptic curve cryptography.
//!
//! Examples can be found within the implementation modules namespaced hereunder.

/// Elliptic curve private key
#[cfg(feature = "elliptic_curve")]
pub trait ECPrivateKey {}

/// Elliptic curve public key
#[cfg(feature = "elliptic_curve")]
pub trait ECPublicKey {}

pub mod curve_25519;

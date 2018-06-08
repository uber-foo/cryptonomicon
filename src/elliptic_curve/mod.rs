//! This module encapsulates the data structures and methods related to elliptic curve cryptography.
//!
//! Examples can be found within the implementation modules namespaced hereunder.

/// Elliptic curve private key
pub trait ECPrivateKey {}

/// Elliptic curve public key
pub trait ECPublicKey {}

/// Curve25519 implementation
#[cfg(feature = "elliptic_curve_25519")]
pub mod curve_25519;

//! Curve25519 implementation
//!
//! ## Examples
//!
//! ### Generating a Curve25519 Private Key
//! ```
//! extern crate cryptonomicon;
//!
//! use cryptonomicon::asymmetric::PrivateKey;
//! use cryptonomicon::elliptic_curve::curve_25519;
//!
//! pub fn main() {
//!     let ec_private_key = curve_25519::Curve25519PrivateKey::generate();
//!     println!("EC Private Key: {:?}", ec_private_key);
//! }

use super::super::asymmetric::{PrivateKey, PublicKey};
use super::super::errors::*;
use super::{ECPrivateKey, ECPublicKey};

/// Curve25519 public key
#[cfg(feature = "elliptic_curve_25519")]
#[derive(Debug, Copy, Clone)]
pub struct Curve25519PublicKey {}

#[cfg(feature = "elliptic_curve_25519")]
impl PublicKey for Curve25519PublicKey {}

#[cfg(feature = "elliptic_curve_25519")]
impl ECPublicKey for Curve25519PublicKey {}

/// Curve25519 private key
#[cfg(feature = "elliptic_curve_25519")]
#[derive(Debug, Copy, Clone)]
pub struct Curve25519PrivateKey {}

#[cfg(feature = "elliptic_curve_25519")]
impl PrivateKey for Curve25519PrivateKey {
    fn generate() -> Result<Vec<u8>> {
        // TODO implement this
        Err("unimplemented".into())
    }
}

#[cfg(feature = "elliptic_curve_25519")]
impl ECPrivateKey for Curve25519PrivateKey {}

//! This module encapsulates the data structures and methods related to elliptic curve cryptography.
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
#[derive(Debug, Copy, Clone)]
pub struct Curve25519PublicKey {}
impl PublicKey for Curve25519PublicKey {}
impl ECPublicKey for Curve25519PublicKey {}

/// Curve25519 private key
#[derive(Debug, Copy, Clone)]
pub struct Curve25519PrivateKey {}
impl PrivateKey for Curve25519PrivateKey {
    fn generate() -> Result<Vec<u8>> {
        // TODO implement this
        Err("unimplemented".into())
    }
}
impl ECPrivateKey for Curve25519PrivateKey {}

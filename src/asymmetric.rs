//! Asymmetric crypto stubs intended for internal usage but exposed to enable extension of this crate

use super::errors::*;

/// Common interface for all asymmetric crypto public keys
#[cfg(feature = "asymmetric")]
pub trait PublicKey {}

/// Common interface for all asymmetric crypto private keys
#[cfg(feature = "asymmetric")]
pub trait PrivateKey {
    /// Generate a new private key
    fn generate() -> Result<Vec<u8>>;
}

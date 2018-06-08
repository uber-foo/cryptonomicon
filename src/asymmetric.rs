use super::errors::*;

/// Common interface for all asymmetric crypto public keys
pub trait PublicKey {}
/// Common interface for all asymmetric crypto private keys
pub trait PrivateKey {
    /// Generate a new private key
    fn generate() -> Result<Vec<u8>>;
}

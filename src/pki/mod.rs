//! Public Key Infrastructure (certificates, et al.)

/// Certificate
#[cfg(feature = "pki")]
pub trait Certificate {}

/// Certificate Request
#[cfg(feature = "pki")]
pub trait CertificateRequest {}

/// Certificate Revocation List
#[cfg(feature = "pki")]
pub trait CertificateRevocationList {}

/// X509 Certificates
pub mod x509;

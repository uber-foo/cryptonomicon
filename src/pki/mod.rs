/// Certificate
pub trait Certificate {}
/// Certificate Request
pub trait CertificateRequest {}
/// Certificate Revocation List
pub trait CertificateRevocationList {}

/// X509 Certificates
#[cfg(feature = "x509")]
pub mod x509;

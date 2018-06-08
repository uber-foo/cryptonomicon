/// X509 Certificate
pub trait X509Certificate {}

/// X509v3 certificate implementation
#[cfg(feature = "x509_v3")]
pub mod v3;

//! X509 v3 Certificates

use super::super::Certificate;
use super::X509Certificate;

/// X509 v3 Certificate
#[cfg(feature = "x509_v3")]
#[derive(Debug, Copy, Clone)]
pub struct X509V3Certificate {}

#[cfg(feature = "x509_v3")]
impl Certificate for X509V3Certificate {}

#[cfg(feature = "x509_v3")]
impl X509Certificate for X509V3Certificate {}

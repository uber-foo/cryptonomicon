use super::super::Certificate;
use super::X509Certificate;

/// X509v3 Certificate
#[derive(Debug, Copy, Clone)]
pub struct X509V3Certificate {}
impl Certificate for X509V3Certificate {}
impl X509Certificate for X509V3Certificate {}

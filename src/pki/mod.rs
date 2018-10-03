//! Public Key Infrastructure (certificates, et al.)

use failure::Error;
use std::path::Path;

/// Certificate error
#[derive(Debug, Fail)]
pub enum CertificateError {
    /// Certificate could not be loaded from the specified file
    #[fail(display = "cannot load certificate from \"{}\"", file_name)]
    FailedToLoad {
        /// path to certificate file that could not be loaded
        file_name: String,
    },
}

/// Certificate
#[cfg(feature = "pki")]
pub trait Certificate {}
impl Certificate {
    /// load a certificate from a file
    pub fn from_file(path: &Path) -> Result<Box<Certificate>, Error> {
        Err(CertificateError::FailedToLoad {
            file_name: path.display().to_string(),
        }
        .into())
    }
}

/// Certificate Request
#[cfg(feature = "pki")]
pub trait CertificateRequest {}

/// Certificate Revocation List
#[cfg(feature = "pki")]
pub trait CertificateRevocationList {}

/// X509 Certificates
pub mod x509;

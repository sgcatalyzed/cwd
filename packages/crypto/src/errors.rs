use std::fmt::Debug;
use thiserror::Error;

pub type CryptoResult<T> = core::result::Result<T, CryptoError>;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Batch verify error: {msg}")]
    BatchErr {
        msg: String,
    },
    #[error("Crypto error: {msg}")]
    GenericErr {
        msg: String,
    },
    #[error("Invalid hash format")]
    InvalidHashFormat,
    #[error("Invalid public key format")]
    InvalidPubkeyFormat,
    #[error("Invalid signature format")]
    InvalidSignatureFormat,
    #[error("Invalid recovery parameter. Supported values: 0 and 1.")]
    InvalidRecoveryParam,
}

impl CryptoError {
    pub fn batch_err(msg: impl Into<String>) -> Self {
        CryptoError::BatchErr {
            msg: msg.into(),
        }
    }

    pub fn generic_err(msg: impl Into<String>) -> Self {
        CryptoError::GenericErr {
            msg: msg.into(),
        }
    }

    pub fn invalid_hash_format() -> Self {
        CryptoError::InvalidHashFormat
    }

    pub fn invalid_pubkey_format() -> Self {
        CryptoError::InvalidPubkeyFormat
    }

    pub fn invalid_signature_format() -> Self {
        CryptoError::InvalidSignatureFormat
    }

    pub fn invalid_recovery_param() -> Self {
        CryptoError::InvalidRecoveryParam
    }

    /// Numeric error code that can easily be passed over the
    /// contract VM boundary.
    pub fn code(&self) -> u32 {
        match self {
            CryptoError::InvalidHashFormat => 3,
            CryptoError::InvalidSignatureFormat => 4,
            CryptoError::InvalidPubkeyFormat => 5,
            CryptoError::InvalidRecoveryParam => 6,
            CryptoError::BatchErr { .. } => 7,
            CryptoError::GenericErr { .. } => 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // constructors
    #[test]
    fn batch_err_works() {
        let error = CryptoError::batch_err("something went wrong in a batch way");
        match error {
            CryptoError::BatchErr { msg, .. } => {
                assert_eq!(msg, "something went wrong in a batch way")
            }
            _ => panic!("wrong error type!"),
        }
    }

    #[test]
    fn generic_err_works() {
        let error = CryptoError::generic_err("something went wrong in a general way");
        match error {
            CryptoError::GenericErr { msg, .. } => {
                assert_eq!(msg, "something went wrong in a general way")
            }
            _ => panic!("wrong error type!"),
        }
    }

    #[test]
    fn invalid_hash_format_works() {
        let error = CryptoError::invalid_hash_format();
        match error {
            CryptoError::InvalidHashFormat => {}
            _ => panic!("wrong error type!"),
        }
    }

    #[test]
    fn invalid_signature_format_works() {
        let error = CryptoError::invalid_signature_format();
        match error {
            CryptoError::InvalidSignatureFormat => {}
            _ => panic!("wrong error type!"),
        }
    }

    #[test]
    fn invalid_pubkey_format_works() {
        let error = CryptoError::invalid_pubkey_format();
        match error {
            CryptoError::InvalidPubkeyFormat => {}
            _ => panic!("wrong error type!"),
        }
    }
}

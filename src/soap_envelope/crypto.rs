//! Basic utilities for Base64 encoding and cryptographic signing.

use super::binary_sec_token::BinarySecurityTokenBase64;
use crate::xml::canonicalization::XmlCanonicalizeError;
use base64::{DecodeError, Engine};
use ring::digest::SHA256;
use ring::error::KeyRejected;
use ring::rand::SystemRandom;
use ring::signature;

/// Encodes a byte slice into a Base64 string.
pub(crate) fn to_base64(bytes: &[u8]) -> String {
    base64::prelude::BASE64_STANDARD.encode(bytes)
}

/// Decodes a Base64 string into raw bytes
fn from_base64(b64: &str) -> Result<Vec<u8>, DecodeError> {
    base64::prelude::BASE64_STANDARD.decode(b64)
}

/// Returns a SHA-256 hash as base64
pub(crate) fn sha256_base64(input: &[u8]) -> String {
    to_base64(ring::digest::digest(&SHA256, input).as_ref())
}

/// Spočítá hash 256 z dat. Poté spočítá podpis pro ten hash.
pub(crate) fn sha256_and_sign_with_pfx(
    public_certif_base64: String,
    private_key_base64: &str,
    data: &[u8],
) -> Result<SignatureInfo, XmlSignError> {
    let private_key =
        from_base64(private_key_base64).map_err(|_| XmlSignError::ReadPrivateKeyFromBase64Err)?;

    let rng = SystemRandom::new();
    let private_key = signature::RsaKeyPair::from_pkcs8(&private_key)
        .map_err(|e| XmlSignError::InvalidPrivateKeyBytes { e })?;

    let mut signature = vec![0; private_key.public().modulus_len()];
    private_key
        .sign(&signature::RSA_PKCS1_SHA256, &rng, data, &mut signature)
        .map_err(|e| XmlSignError::SignError { e })?;

    Ok(SignatureInfo {
        signature,
        binary_security_token: BinarySecurityTokenBase64::new(public_certif_base64),
    })
}

#[derive(Debug, thiserror::Error)]
pub enum XmlSignError {
    #[error("Failed to decode private key from Base64")]
    ReadPrivateKeyFromBase64Err,

    #[error("Cannot read private key: {}", e.as_display())]
    InvalidPrivateKeyBytes { e: KeyRejected },

    #[error("Signing failed: {}", e.as_display())]
    SignError { e: ring::error::Unspecified },

    #[error("XML canonicalization failed: {0}")]
    CanoniError(#[from] XmlCanonicalizeError),
}

/// Represents a digital signature along with its public certificate information.
#[derive(Debug, derive_more::Constructor, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SignatureInfo {
    pub(crate) signature: Vec<u8>,
    pub(crate) binary_security_token: BinarySecurityTokenBase64,
}

/// Represents a public certificate as a base64 string
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub(crate) struct BinarySecurityTokenBase64(String);

impl BinarySecurityTokenBase64 {
    pub(crate) fn new(base64_public_certif: String) -> Self {
        Self(base64_public_certif)
    }
}

use chrono::{DateTime, Duration, Utc};
use std::borrow::Cow;

/// Defines the validity period of a SOAP message (creation and expiration times)
/// Timestamps use UTC.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, derive_more::Constructor, Hash)]
pub struct Timestamp {
    created: DateTime<Utc>,
    expires: DateTime<Utc>,
}

impl Timestamp {
    /// Creates a `Timestamp` that is valid from now for the given number of minutes
    pub fn new_valid_for_minutes(mins: i32) -> Self {
        let created = Utc::now();
        let expires = created + Duration::minutes(mins as i64);
        Self { created, expires }
    }

    /// Returns the creation time formatted for the SOAP envelope (`YYYY-MM-DDTHH:mm:ssZ`)
    pub fn created_str(&self) -> String {
        Timestamp::format_ymdhmsz(self.created)
    }

    /// Returns the expiration time formatted for the SOAP envelope (`YYYY-MM-DDTHH:mm:ssZ`)
    pub fn expires_str(&self) -> String {
        Timestamp::format_ymdhmsz(self.expires)
    }

    /// Returns the timestamp formatted for the SOAP envelope (`YYYY-MM-DDTHH:mm:ssZ`)
    fn format_ymdhmsz(d: DateTime<Utc>) -> String {
        d.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}

/// Defines the security variant used for a SOAP message.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SoapSecurity<'s> {
    /// Bez zabezpečení
    None,

    /// The message is signed using a client certificate (PFX format).
    /// Uses exclusive XML canonicalization, SHA-256, and RSA.
    ClientCertificate {
        public_base64: Cow<'s, str>,
        private_base64: Cow<'s, str>,
    },
}

use super::{
    binary_sec_token::BinarySecurityTokenBase64,
    crypto::{XmlSignError, sha256_and_sign_with_pfx, sha256_base64, to_base64},
    soap_envelope_parts::{SoapSecurity, Timestamp},
};
use crate::xml::canonicalization::{CanonizedXml, XmlCanonicalizeError};
use std::borrow::Cow;

/// The main structure of this library: a SOAP message envelope.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoapEnvelope<'s> {
    /// Optional validity period (from–to, in UTC).
    timestamp: Option<Timestamp>,

    /// The raw XML body of the message.
    body_xml: Cow<'s, str>,

    /// The chosen security variant for this message.
    security: SoapSecurity<'s>,
}

impl<'s> SoapEnvelope<'s> {
    /// Builds a signed SOAP envelope valid for the specified duration,
    /// using the provided Base64-encoded certificate and key.
    pub fn new_signed_with_timestamp(
        minutes_valid: i32,
        body_xml: impl Into<Cow<'s, str>>,
        public_certif_base64: impl Into<Cow<'s, str>>,
        private_key_base64: impl Into<Cow<'s, str>>,
    ) -> Self {
        Self {
            body_xml: body_xml.into(),
            security: SoapSecurity::ClientCertificate {
                public_base64: public_certif_base64.into(),
                private_base64: private_key_base64.into(),
            },
            timestamp: Some(Timestamp::new_valid_for_minutes(minutes_valid)),
        }
    }

    /// Builds a signed SOAP envelope,
    /// using the provided Base64-encoded certificate and key.
    pub fn new_signed(
        body_xml: impl Into<Cow<'s, str>>,
        public_certif_base64: impl Into<Cow<'s, str>>,
        private_key_base64: impl Into<Cow<'s, str>>,
    ) -> Self {
        Self {
            body_xml: body_xml.into(),
            security: SoapSecurity::ClientCertificate {
                public_base64: public_certif_base64.into(),
                private_base64: private_key_base64.into(),
            },
            timestamp: None,
        }
    }

    /// Builds an unsigned SOAP envelope
    pub fn new_no_security_header(body_xml: impl Into<Cow<'s, str>>) -> Self {
        Self {
            body_xml: body_xml.into(),
            security: SoapSecurity::None,
            timestamp: None,
        }
    }

    /// Builds an unsigned SOAP envelope valid for the specified duration
    pub fn new_no_security_header_with_timestamp(
        minutes_valid: i32,
        body_xml: impl Into<Cow<'s, str>>,
    ) -> Self {
        Self {
            body_xml: body_xml.into(),
            security: SoapSecurity::None,
            timestamp: Some(Timestamp::new_valid_for_minutes(minutes_valid)),
        }
    }

    /// Returns SOAP envelope XML as String
    /// This operation may fail if it includes XML canonicalization and signing
    pub fn get_final_xml(&self) -> Result<String, XmlSignError> {
        let timestamp = self.get_timestamp()?;
        let fullbody = self.get_fullbody()?;
        let signed_info_xml =
            self.get_ds_signed_info(timestamp.as_ref(), &sha256_base64(fullbody.as_bytes()))?;

        let timestamp_elem_xml = match timestamp {
            Some(timestamp) => format!("{timestamp}"),
            None => String::new(),
        };

        let wsse_security_elem_xml = match &self.security {
            SoapSecurity::None => format!(
                // V security bude jen Timestamp
                r#"<wsse:Security soapenv:mustUnderstand="1">{timestamp_elem_xml}</wsse:Security>"#
            ),
            SoapSecurity::ClientCertificate {
                public_base64,
                private_base64,
            } => {
                // V security bude Timestamp, hashe i podpis
                let (doc_sign, binary_sec_token_base64) = self.get_doc_signature(
                    &signed_info_xml,
                    public_base64.to_string(),
                    &private_base64,
                )?;

                format!(
                    r##"
                <wsse:Security soapenv:mustUnderstand="1">
            {timestamp_elem_xml}
            <wsse:BinarySecurityToken
                wsu:Id="{BIN_SEC_TOKEN_ID}"
                ValueType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-x509-token-profile-1.0#X509v3"
                EncodingType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary">{binary_sec_token_base64}</wsse:BinarySecurityToken>
            <ds:Signature>{signed_info_xml}<ds:SignatureValue>{doc_sign}</ds:SignatureValue>
                <ds:KeyInfo>
                    <wsse:SecurityTokenReference>
                        <wsse:Reference URI="#{BIN_SEC_TOKEN_ID}"
                                       ValueType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-x509-token-profile-1.0#X509v3"/>
                    </wsse:SecurityTokenReference>
                </ds:KeyInfo>
            </ds:Signature>
        </wsse:Security>
                "##
                )
            }
        };

        Ok(format!(
            r##"
    <soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"
                  xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd"
                  xmlns:ds="http://www.w3.org/2000/09/xmldsig#"
                  xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
    <soapenv:Header>{wsse_security_elem_xml}</soapenv:Header>{fullbody}</soapenv:Envelope>
    "##
        ))
    }

    /// Builds the complete `<soapenv:Body>` element of the SOAP message as canonical XML.
    fn get_fullbody(&self) -> Result<CanonizedXml, XmlCanonicalizeError> {
        let fullbody = format!(
            r#"
        <soapenv:Body xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" 
            xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" 
            wsu:Id="{BODY_ID}">{}</soapenv:Body>
        "#,
            self.body_xml
        );
        let body_canoni = CanonizedXml::new(&fullbody)?;
        Ok(body_canoni)
    }

    /// Builds the complete `<wsu:Timestamp>` element of the SOAP message as canonical XML.
    fn get_timestamp(&self) -> Result<Option<CanonizedXml>, XmlCanonicalizeError> {
        match &self.timestamp {
            Some(timestamp) => {
                let timestamp_xml = format!(
                    r#"
                        <wsu:Timestamp xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" wsu:Id="{TIMESTAMP_ID}">
                            <wsu:Created>{}</wsu:Created>
                            <wsu:Expires>{}</wsu:Expires>
                        </wsu:Timestamp>
                    "#,
                    timestamp.created_str(),
                    timestamp.expires_str(),
                );

                let timestamp_canoni = CanonizedXml::new(&timestamp_xml)?;
                Ok(Some(timestamp_canoni))
            }
            None => Ok(None),
        }
    }

    /// Builds the complete `<ds:SignedInfo>` element of the SOAP message as canonical XML.
    fn get_ds_signed_info(
        &self,
        timestamp: Option<&CanonizedXml>,
        body_hash: &str,
    ) -> Result<CanonizedXml, XmlCanonicalizeError> {
        let timestamp_reference_xml = match timestamp {
            Some(timestamp) => format!(
                r##"
             <ds:Reference URI="#{TIMESTAMP_ID}">
                        <ds:Transforms>
                            <ds:Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#" />
                        </ds:Transforms>
                        <ds:DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256" />
                        <ds:DigestValue>{timestamp_hash}</ds:DigestValue>
                    </ds:Reference>
            "##,
                timestamp_hash = sha256_base64(timestamp.as_bytes())
            ),
            None => String::new(),
        };

        CanonizedXml::new(&format!(
            r##"
        <ds:SignedInfo xmlns:ds="http://www.w3.org/2000/09/xmldsig#">
                    <ds:CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"/>
                    <ds:SignatureMethod Algorithm="http://www.w3.org/2001/04/xmldsig-more#rsa-sha256"/>
                   {timestamp_reference_xml}
        <ds:Reference URI="#{BODY_ID}">
               <ds:Transforms>
                   <ds:Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"/>
               </ds:Transforms>
               <ds:DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256"/>
               <ds:DigestValue>{body_hash}</ds:DigestValue>
           </ds:Reference>
        </ds:SignedInfo>
        "##
        ))
    }

    /// Generates the full digital signature and security token from the given `<SignedInfo>` XML element.
    /// Uses the provided Base64-encoded certificate and private key to create the signature.
    fn get_doc_signature(
        &self,
        signed_info_xml: &CanonizedXml,
        public_certif_base64: String,
        private_key_base64: &str,
    ) -> Result<(String, BinarySecurityTokenBase64), XmlSignError> {
        let signat = sha256_and_sign_with_pfx(
            public_certif_base64,
            private_key_base64,
            signed_info_xml.as_bytes(),
        )?;
        let signature_base64 = to_base64(&signat.signature);
        Ok((signature_base64, signat.binary_security_token))
    }
}

/// `wsu:Id` for the `<Timestamp>` XML element.
const TIMESTAMP_ID: &str = "Timsta";

/// `wsu:Id` for the `<Body>` XML element.
const BODY_ID: &str = "Msgbody";

/// `wsu:Id` for the `<BinarySecurityToken>` XML element.
const BIN_SEC_TOKEN_ID: &str = "X509Token1";

mod tests;
pub mod xml_canonized_doc;
use std::fmt::Display;

use xml_canonized_doc::XmlCanonizedDoc;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum XmlCanonicalizeError {
    #[error("Failed to read XML: {0}")]
    XmlReadError(#[from] xml::reader::Error),

    #[error("Failed to read a text value in the XML document")]
    ReadTextValueError,

    #[error("Empty string is not a valid XML document")]
    EmptyDoc,

    #[error(
        "Failed to parse XML namespace prefix: '{0}'. Must be ASCII and no longer than 16 characters."
    )]
    InvalidXmlnsPrefix(String),
}

/// A string that represents valid, canonicalized XML.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct CanonizedXml(String);

impl CanonizedXml {
    /// Reads a XML document and converts it to its canonical form.
    /// The XML reading or conversion may fail.
    pub fn new(xml: &str) -> Result<Self, XmlCanonicalizeError> {
        // stromová struktura kanonizovaného dokumentu. Při parsování může dojít k chybě
        let canonized_xml_tree = XmlCanonizedDoc::parse(xml)?;
        // Převod stromu XML dokumentu na string
        let canonized_xml = canonized_xml_tree.write_xml_as_string();
        Ok(Self(canonized_xml))
    }

    /// Returns its XML as UTF-8 bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns its XML as UTF-8 string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Moves its inner value to a regular string
    pub fn to_string(self) -> String {
        self.0
    }
}

/// Returns XML in its canonical form
impl Display for CanonizedXml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

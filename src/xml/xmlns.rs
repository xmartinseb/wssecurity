use crate::tools::StrCmpIgnoreCase;
use std::rc::Rc;

/// Represents a XML namespace (prefix and url)
#[derive(Debug, Eq, Clone, derive_more::Constructor)]
pub(crate) struct Xmlns {
    /// The URL is wrapped in `Rc` to avoid unnecessary allocations on clone.
    pub url: Rc<String>,

    /// XML namespace prefix assumes max. 16 ASCII chars
    pub prefix: tinystr::TinyStr16,
}

impl Xmlns {
    /// Returns `true` if this is the default namespace (i.e., no prefix, `xmlns="..."`)
    pub fn is_default(&self) -> bool {
        self.prefix.is_empty() && !self.url.is_empty()
    }
}

impl PartialEq for Xmlns {
    fn eq(&self, other: &Self) -> bool {
        self.prefix == other.prefix && self.url == other.url
    }
}

/// Xmlns are ordered by prefix, then by Uri
impl PartialOrd for Xmlns {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Xmlns are ordered by prefix, then by Uri
impl Ord for Xmlns {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.prefix
            .cmp_ignore_case(&other.prefix)
            .then(self.url.cmp_ignore_case(&other.url))
    }
}

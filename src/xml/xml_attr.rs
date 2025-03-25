use crate::tools::StrCmpIgnoreCase;
use std::borrow::Cow;
use std::cmp::Ordering;

use super::xmlns::Xmlns;

/// Represents a XML attribute by xml namespace, local name and string value
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct XmlAttr {
    pub ns: Option<Xmlns>,
    pub local_name: String,
    pub value: String,
}

impl XmlAttr {
    pub fn get_fullname(&self) -> Cow<str> {
        match &self.ns {
            Some(ns) => Cow::Owned(format!("{}:{}", ns.prefix, self.local_name)),
            None => Cow::Borrowed(&self.local_name),
        }
    }
}

/// Atributy se řadí podle abecedy podle celého názvu
impl PartialOrd for XmlAttr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Atributy se řadí podle abecedy podle celého názvu
impl Ord for XmlAttr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.ns.clone(), other.ns.clone()) {
            (None, None) => self.local_name.cmp(&other.local_name),
            (None, Some(_)) => Ordering::Less, // prvne se renderuji bezprefixove atributy
            (Some(_), None) => Ordering::Greater, // prvne se renderuji bezprefixove atributy
            (Some(self_ns), Some(other_ns)) => self_ns
                .cmp(&other_ns)
                .then(self.local_name.cmp_ignore_case(&other.local_name)),
        }
    }
}

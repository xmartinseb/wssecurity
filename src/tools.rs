use std::cmp::Ordering;

use tinystr::TinyStr16;

/// Compares two strs in case-insensitive mode.
/// Doesn't create any clones.
pub(crate) trait StrCmpIgnoreCase {
    fn cmp_ignore_case(&self, other: &Self) -> Ordering;
}

impl StrCmpIgnoreCase for str {
    fn cmp_ignore_case(&self, other: &Self) -> Ordering {
        // Iterativní porovnání znaků
        for (c1, c2) in self.chars().zip(other.chars()) {
            let cmp = c1.to_ascii_lowercase().cmp(&c2.to_ascii_lowercase());
            if cmp != Ordering::Equal {
                return cmp; // Pokud najdeme rozdíl, vrátíme výsledek
            }
        }

        // Porovnáme délky, pokud všechny znaky byly shodné
        self.len().cmp(&other.len())
    }
}

pub(crate) fn str_to_tinystr16(s: &str) -> Result<TinyStr16, tinystr::ParseError> {
    TinyStr16::try_from_utf8(s.as_bytes())
}

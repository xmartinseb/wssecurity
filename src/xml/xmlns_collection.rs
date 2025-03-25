use super::xmlns::Xmlns;
use std::collections::BTreeSet;

/// Kolekce, která udržuje seřazené XMLNS. Při insertu se podívá, zda už je daný prefix s url definován.
/// Pokud už je, nic se nevloží.
/// Takhle se zajišťuje, že v kanonizovaném dokumentu je každý xmlns definován jen jednou
#[derive(Debug, Clone, Default)]
pub(crate) struct XmlnsCollection(BTreeSet<Xmlns>);

impl XmlnsCollection {
    /// Inserts a new `xmlns` entry if needed, following these rules:
    /// 1. If (prefix, url) already exists — do nothing.
    /// 2. If the prefix exists with a different URL — replace it.
    /// 3. If (prefix, url) doesn't exists — insert the entry.
    pub fn insert_or_replace_if_not_found(&mut self, xmlns: Xmlns) {
        // prefix: TinyStr16, url: &str
        let n = self.0.iter().cloned().find(|n| n.prefix == xmlns.prefix);

        match n {
            Some(found) => {
                if *found.url == *xmlns.url {
                    // Je-li toto xmlns (prefix, url) už definováno, nesmí se duplikovat
                    return;
                } else {
                    // Je-li v seznamu xmlns s tímtéž prefixem, ale jiným url, přepíše se
                    self.0.remove(&found.clone());
                    self.0.insert(xmlns);
                };
            }
            None => {
                self.0.insert(xmlns);
            }
        };
    }

    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, Xmlns> {
        self.0.iter()
    }
}

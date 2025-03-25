use super::XmlCanonicalizeError;
use crate::{
    tools::str_to_tinystr16,
    xml::{
        xml_attr::XmlAttr,
        xml_elem::{ElemContent, XmlElem, XmlElemRc},
        xmlns::Xmlns,
        xmlns_collection::XmlnsCollection,
    },
};
use std::{
    collections::{BTreeSet, LinkedList},
    rc::Rc,
};
use tinystr::TinyStr16;
use xml::{EventReader, name::OwnedName, namespace::Namespace, reader::XmlEvent};

/// A XML document tree ready to be rendered as XML string in its canonical form.
#[derive(Debug)]
pub(crate) struct XmlCanonizedDoc {
    root: XmlElemRc,
}

impl XmlCanonizedDoc {
    /// Parses an XML string into a `XmlCanonizedDoc`.
    /// Returns an error if parsing fails.
    pub fn parse(xml: &str) -> Result<Self, XmlCanonicalizeError> {
        let parser = EventReader::from_str(xml);
        let root = parse_elem(parser)?;
        Ok(XmlCanonizedDoc { root })
    }

    /// Returns the canonicalized XML as a string.
    /// Canonicalization is performed recursively using the internal tree structure.
    pub fn write_xml_as_string(&self) -> String {
        self.root
            .borrow_elem_mut()
            .get_canonized_xml_recur(BTreeSet::default())
    }
}

/// Reads the next xml object and converts it to a XmlElemRc. The conversion may fail.
/// It only reads startElement, endElement and text values. Comments are ignored.
fn parse_elem(parser: EventReader<&[u8]>) -> Result<XmlElemRc, XmlCanonicalizeError> {
    let mut elems_stack = LinkedList::<XmlElemRc>::new();
    let mut root: Option<XmlElemRc> = None;
    for e in parser {
        let e = e?;
        match e {
            XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            } => {
                let parent = elems_stack.back();
                // Každý element zná namespacy, které zná jeho rodič. Může přidat vlastní, nebo redefinovat
                let mut known_nss = match parent {
                    Some(p) => p.borrow_elem().known_nss.clone(),
                    None => Default::default(),
                };
                insert_known_namespaces(namespace, &mut known_nss)?;
                let mut used_nss = BTreeSet::<Xmlns>::default();

                // Pokud je použit default namespace (tj. element je definován bez prefixu),
                // je potřeba navýšit počet použití u default xmlns
                if name.prefix.is_none() {
                    if let Some(defaultns) = known_nss.iter().find(|ns| ns.is_default()) {
                        used_nss
                            .insert(Xmlns::new(defaultns.url.clone(), defaultns.prefix.clone()));
                    }
                }

                let mut attrs = BTreeSet::<XmlAttr>::default();
                for at in attributes.iter() {
                    attrs.insert(XmlAttr {
                        local_name: at.name.local_name.clone(),
                        value: at.value.clone(),
                        ns: to_ns_insert_into_used(&at.name, &mut known_nss, &mut used_nss)?,
                    });
                }

                let elem = XmlElemRc::new(XmlElem {
                    ns: to_ns_insert_into_used(&name, &mut known_nss, &mut used_nss)?,
                    local_name: name.local_name,
                    used_nss,
                    content: ElemContent::Text("".to_owned()), // Může být doplněno později
                    known_nss,
                    attrs,
                });

                // elem muze byt neci potomek
                if let Some(parent_elem) = elems_stack.back() {
                    parent_elem.borrow_elem_mut().add_child(elem.clone());
                } else {
                    root = Some(elem.clone());
                }
                elems_stack.push_back(elem);
            }
            XmlEvent::EndElement { .. } => {
                elems_stack.pop_back();
            }
            XmlEvent::Characters(value) => {
                elems_stack
                    .back()
                    .ok_or(XmlCanonicalizeError::ReadTextValueError)?
                    .borrow_elem_mut()
                    .set_text_value(value);
            }
            _ => {}
        }
    }

    match root {
        Some(root) => Ok(root),
        None => Err(XmlCanonicalizeError::EmptyDoc),
    }
}

/// Iterates over a set of namespaces and inserts each into the known list,
/// following the inserting rules of `insert_or_replace_if_not_found`.
/// Returns an error if any prefix fails to parse.
fn insert_known_namespaces(
    namespaces: Namespace,
    known_nss: &mut XmlnsCollection,
) -> Result<(), XmlCanonicalizeError> {
    for (prefix, url) in namespaces.iter() {
        let prefix = parse_xmlns_prefix(prefix)?;
        let xmlns = Xmlns::new(Rc::new(url.to_owned()), prefix);
        known_nss.insert_or_replace_if_not_found(xmlns);
    }
    Ok(())
}

/// Converts an `OwnedName` into an `Xmlns`, inserting it into both the known and used namespace sets.
/// Returns the created `Xmlns`, or an error if the prefix cannot be parsed.
fn to_ns_insert_into_used(
    n: &OwnedName,
    known_nss: &mut XmlnsCollection,
    used_nss: &mut BTreeSet<Xmlns>,
) -> Result<Option<Xmlns>, XmlCanonicalizeError> {
    match &n.prefix {
        Some(prefix) => {
            let prefix = parse_xmlns_prefix(prefix)?;
            let url = n.namespace.clone().unwrap();
            let xmlns = Xmlns::new(Rc::new(url), prefix);
            known_nss.insert_or_replace_if_not_found(xmlns.clone());
            used_nss.insert(xmlns.clone());
            Ok(Some(xmlns))
        }
        None => Ok(None),
    }
}

/// Parses a `&str` into a `TinyStr16` used for XML namespace prefixes.
fn parse_xmlns_prefix(prefix: &str) -> Result<TinyStr16, XmlCanonicalizeError> {
    str_to_tinystr16(prefix)
        .map_err(|_| XmlCanonicalizeError::InvalidXmlnsPrefix(prefix.to_owned()))
}

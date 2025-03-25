use std::{
    borrow::Cow,
    cell::{Ref, RefCell, RefMut},
    collections::BTreeSet,
    rc::Rc,
};

use super::{xml_attr::XmlAttr, xmlns::Xmlns, xmlns_collection::XmlnsCollection};

#[derive(Debug, Clone)]
pub(crate) struct XmlElemRc(Rc<RefCell<XmlElem>>);

impl XmlElemRc {
    pub fn new(x: XmlElem) -> Self {
        Self(Rc::new(RefCell::new(x)))
    }

    pub fn borrow_elem(&self) -> Ref<XmlElem> {
        self.0.borrow()
    }

    pub fn borrow_elem_mut(&self) -> RefMut<XmlElem> {
        self.0.borrow_mut()
    }
}

#[derive(Debug)]
pub(crate) struct XmlElem {
    /// Zařazení do namespace (prefix a url)
    pub ns: Option<Xmlns>,

    pub local_name: String,

    /// Content je buď množina XML elementů, nebo text (může být prázdný)
    pub content: ElemContent,

    /// Namespacy, které element zná. Buďto je sám definuje, nebo je zná po předcích
    pub known_nss: XmlnsCollection,

    /// Namespacy, které tento element přímo používá ve svých atributech či názvu.
    pub used_nss: BTreeSet<Xmlns>,

    /// XML atributy elementu
    pub attrs: BTreeSet<XmlAttr>,
}

impl XmlElem {
    /// Přidá child element. Pokud byl dříve obsahem elementu prostý text, bude zahozen.
    pub fn add_child(&mut self, child: XmlElemRc) {
        match &mut self.content {
            ElemContent::Children(children) => children.push(child),
            _ => self.content = ElemContent::Children(vec![child]),
        }
    }

    /// Dá XML elementu textovou hodnotu. Pokud měl element dříve child elementy, budou zahozeny.
    pub fn set_text_value(&mut self, val: String) {
        match val.as_str() {
            _ => self.content = ElemContent::Text(val),
        };
    }

    pub fn get_fullname(&self) -> Cow<str> {
        match &self.ns {
            Some(ns) => Cow::Owned(format!("{}:{}", ns.prefix, self.local_name)),
            None => Cow::Borrowed(&self.local_name),
        }
    }

    /// Použito pro renderování finálního kanonizovaného XML.
    /// Vrátí atributy seřazené podle celého názvu
    pub fn get_ordered_attrs(&self) -> String {
        let mut attrs_str = String::new();
        for a in self.attrs.iter() {
            attrs_str += &format!(" {}=\"{}\"", a.get_fullname(), escape_xml(&a.value));
        }
        attrs_str
    }

    /// Použito pro renderování finálního kanonizovaného XML.
    /// Vrátí inner xml jako string
    pub fn content_as_string(&self, written_xmlns: BTreeSet<Xmlns>) -> Cow<str> {
        match &self.content {
            ElemContent::Text(txt) => Cow::Borrowed(txt),
            ElemContent::Children(elems) => {
                Cow::Owned(XmlElem::many_nodes_as_xml_string(&elems, written_xmlns))
            }
        }
    }

    /// Použito pro renderování finálního kanonizovaného XML.
    /// Proiteruje pole XML elementů, přičemž z nich vyrobí XML textovou reprezentaci
    fn many_nodes_as_xml_string(elems: &[XmlElemRc], written_xmlns: BTreeSet<Xmlns>) -> String {
        if elems.len() == 1 {
            elems[0]
                .borrow_elem()
                .get_canonized_xml_recur(written_xmlns)
        } else {
            let mut s = String::new();
            for e in elems {
                s += &e
                    .borrow_elem()
                    .get_canonized_xml_recur(written_xmlns.clone());
            }
            s
        }
    }

    pub(crate) fn get_canonized_xml_recur(&self, mut written_xmlns: BTreeSet<Xmlns>) -> String {
        let mut local_written_xmlns = BTreeSet::<Xmlns>::new();

        for used_ns in self.used_nss.iter() {
            if !written_xmlns.contains(used_ns) {
                local_written_xmlns.insert(used_ns.clone());
            }
        }

        let mut local_written_xmlns_str = String::with_capacity(256);
        for n in local_written_xmlns {
            local_written_xmlns_str += &format!(
                " {}=\"{}\"",
                if n.is_default() {
                    "xmlns".to_owned()
                } else {
                    format!("xmlns:{}", &n.prefix)
                },
                n.url
            );
            written_xmlns.insert(n);
        }

        format!(
            "<{fullname}{nss}{attrs}>{content}</{fullname}>",
            fullname = self.get_fullname(),
            nss = local_written_xmlns_str,
            attrs = self.get_ordered_attrs(),
            content = self.content_as_string(written_xmlns)
        )
    }
}

#[derive(Debug)]
pub(crate) enum ElemContent {
    Text(String),
    Children(Vec<XmlElemRc>),
}

fn escape_xml(inner_text: &str) -> String {
    let mut escaped_text = String::with_capacity(inner_text.len());

    for ch in inner_text.chars() {
        match ch {
            '&' => escaped_text.push_str("&amp;"),
            '<' => escaped_text.push_str("&lt;"),
            '>' => escaped_text.push_str("&gt;"),
            '"' => escaped_text.push_str("&quot;"),
            '\'' => escaped_text.push_str("&apos;"),
            _ => escaped_text.push(ch),
        }
    }

    escaped_text
}

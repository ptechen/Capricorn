use tendril::StrTendril;
use serde::Deserialize;
use crate::document_selection::DocumentSelection;

#[derive(Deserialize, Clone, Debug)]
pub struct Has {
    pub class: Option<String>,
    pub attr: Option<String>,
}

impl Has {
    pub fn class<'a>(&self, ds: DocumentSelection<'a>) -> (DocumentSelection<'a>, bool) {
        if self.class.is_none() {
            return (ds, true)
        }
        let class = self.class.as_ref().unwrap();
        if class == "" {
            return (ds, false);
        };
        return match ds {
            DocumentSelection::ParseDocument(d) => {
                (DocumentSelection::ParseDocument(d), d.root().has_class(class))
            }
            DocumentSelection::ParseSelection(d) => {
                (DocumentSelection::ParseSelection(d.to_owned()), d.to_owned().has_class(class))
            }
            DocumentSelection::ParseNode(d) => {
                (DocumentSelection::ParseNode(d.to_owned()), d.to_owned().has_class(class))
            }
        };
    }

    pub fn attr<'a>(&self, ds: DocumentSelection<'a>) -> (DocumentSelection<'a>, bool) {
        if self.attr.is_none() {
            return (ds, true);
        }
        let attr = self.attr.as_ref().unwrap();
        if attr == "" {
            return (ds, false);
        }
        return match ds {
            DocumentSelection::ParseDocument(d) => {
                let str_tendril = d.root().attr(attr).unwrap();
                let cur_str = str_tendril.trim();
                if cur_str == "" {
                    (DocumentSelection::ParseDocument(d), false)
                } else {
                    (DocumentSelection::ParseDocument(d), true)
                }
            }
            DocumentSelection::ParseSelection(d) => {
                let str_tendril = d.attr(attr).unwrap_or(StrTendril::default());
                let cur_str = str_tendril.trim();
                if cur_str == "" {
                    (DocumentSelection::ParseSelection(d), false)
                } else {
                    (DocumentSelection::ParseSelection(d), false)
                }
            }
            DocumentSelection::ParseNode(d) => {
                let str_tendril = d.attr(attr).unwrap_or(StrTendril::default());
                let cur_str = str_tendril.trim();
                if cur_str == "" {
                    (DocumentSelection::ParseNode(d), false)
                } else {
                    (DocumentSelection::ParseNode(d), true)
                }
            }
        };
    }
}
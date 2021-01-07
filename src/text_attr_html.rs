use crate::document_selection::DocumentSelection;
use serde::Deserialize;
use tendril::StrTendril;

#[derive(Default, Deserialize, Clone, Debug)]
pub struct TextAttrHtml {
    pub text: Option<bool>,
    pub attr: Option<String>,
    pub html: Option<bool>,
}

impl TextAttrHtml {
    pub fn run<'a>(&self, params: DocumentSelection<'a>) -> String {
        return if self.attr.is_some() && self.attr.as_ref().unwrap() != "" {
            self.attr(params)
        } else if self.html.is_some() && self.html.unwrap() {
            self.html(params)
        } else {
            self.text(params)
        }
    }

    pub fn text<'a>(&self, params: DocumentSelection<'a>) -> String {
        return match params {
            DocumentSelection::ParseSelection(d) => {
                let str_tendril = d.text();
                let cur_str = str_tendril.trim();
                String::from(cur_str)
            }

            DocumentSelection::ParseNode(d) => {
                let str_tendril = d.text();
                let cur_str = str_tendril.trim();
                String::from(cur_str)
            }

            DocumentSelection::ParseDocument(d) => {
                let str_tendril = d.text();
                let cur_str = str_tendril.trim();
                String::from(cur_str)
            }
        };
    }

    fn html<'a>(&self, params: DocumentSelection<'a>) -> String {
        return match params {
            DocumentSelection::ParseSelection(d) => {
                let str_tendril = d.html();
                let cur_str = str_tendril.trim();
                String::from(cur_str)
            }

            DocumentSelection::ParseNode(d) => {
                let str_tendril = d.html();
                let cur_str = str_tendril.trim();
                String::from(cur_str)
            }

            DocumentSelection::ParseDocument(d) => {
                let str_tendril = d.html();
                let cur_str = str_tendril.trim();
                String::from(cur_str)
            }
        };
    }

    pub fn attr<'a>(&self, params: DocumentSelection<'a>) -> String {
        let attr = self.attr.as_ref().unwrap();
        return match params {
            DocumentSelection::ParseSelection(d) => {
                let str_tendril = d.attr(attr).unwrap_or(StrTendril::default());
                let cur_str = str_tendril.trim();
                String::from(cur_str)
            }

            DocumentSelection::ParseNode(d) => {
                let str_tendril = d.attr(attr).unwrap_or(StrTendril::default());
                let cur_str = str_tendril.trim();
                String::from(cur_str)
            }

            DocumentSelection::ParseDocument(d) => {
                let str_tendril = d.root().attr(attr).unwrap();
                let cur_str = str_tendril.trim();
                String::from(cur_str)
            }
        };
    }
}
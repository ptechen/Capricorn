use crate::document_selection::DocumentSelection::ParseDocument;
use crate::document_selection::DocumentSelection::ParseNode;
use crate::document_selection::DocumentSelection::ParseSelection;
use crate::parse::SelectParams;
use crate::text_attr_html;
use nipper::{Document, Node, Selection};
use serde_json::Value;
use std::vec::Vec;

lazy_static! {
    static ref DEFAULT_EXEC_ORDER: Vec<String> = vec![
        String::from("selects"),
        String::from("each"),
        String::from("select_params"),
        String::from("nodes"),
        String::from("has"),
        String::from("contains")
    ];
}

pub enum DocumentSelection<'a> {
    ParseSelection(Selection<'a>),
    ParseDocument(&'a Document),
    ParseNode(Node<'a>),
}

impl<'a> Default for DocumentSelection<'a> {
    fn default() -> DocumentSelection<'a> {
        let ds = Selection::default();
        DocumentSelection::ParseSelection(ds)
    }
}

impl<'a> DocumentSelection<'a> {
    pub fn parse(self, params: &'a SelectParams) -> Value {
        self.parse_exec_order(params)
    }

    pub fn html(self) -> (DocumentSelection<'a>, String) {
        return match self {
            DocumentSelection::ParseSelection(d) => {
                let str_tendril = d.html();
                let cur_str = str_tendril.trim();
                (DocumentSelection::ParseSelection(d), String::from(cur_str))
            }

            DocumentSelection::ParseNode(d) => {
                let str_tendril = d.html();
                let cur_str = str_tendril.trim();
                (DocumentSelection::ParseNode(d), String::from(cur_str))
            }

            DocumentSelection::ParseDocument(d) => {
                let str_tendril = d.html();
                let cur_str = str_tendril.trim();
                (DocumentSelection::ParseDocument(d), String::from(cur_str))
            }
        };
    }

    pub fn text(self) -> (DocumentSelection<'a>, String) {
        return match self {
            DocumentSelection::ParseSelection(d) => {
                let str_tendril = d.text();
                let cur_str = str_tendril.trim();
                (DocumentSelection::ParseSelection(d), String::from(cur_str))
            }

            DocumentSelection::ParseNode(d) => {
                let str_tendril = d.text();
                let cur_str = str_tendril.trim();
                (DocumentSelection::ParseNode(d), String::from(cur_str))
            }

            DocumentSelection::ParseDocument(d) => {
                let str_tendril = d.text();
                let cur_str = str_tendril.trim();
                (DocumentSelection::ParseDocument(d), String::from(cur_str))
            }
        };
    }

    fn parse_exec_order(mut self, params: &SelectParams) -> Value {
        let exec_order;
        if params.exec_order.is_none() {
            exec_order = DEFAULT_EXEC_ORDER.to_owned();
        } else {
            let or = params.exec_order.as_ref().unwrap();
            exec_order = or.to_owned();
        }
        for val in exec_order.iter() {
            if val == "each" {
                if params.each.is_some() {
                    let each = params.each.as_ref().unwrap();
                    return each.each(self);
                }
            } else if val == "select_params" {
                if params.select_params.is_some() {
                    let cur_params = params.select_params.as_ref().as_ref().unwrap();
                    return self.parse(cur_params);
                }
            } else {
                self = self.parse_key(params, val);
            }
        }
        self.content(params)
    }

    fn parse_key(mut self, params: &SelectParams, exec: &str) -> DocumentSelection<'a> {
        return match exec {
            "selects" => {
                if params.selects.is_some() {
                    let s = params.selects.as_ref().unwrap();
                    self = self.selects(s);
                }
                self
            }

            "nodes" => {
                if params.nodes.is_some() {
                    let cur_node = params.nodes.as_ref().unwrap();
                    self = cur_node.run(self);
                }
                self
            }

            "has" => {
                if params.has.is_some() {
                    let has = params.has.as_ref().unwrap();
                    let (ds, b) = has.class(self);
                    if !b {
                        return DocumentSelection::default();
                    }
                    self = ds;

                    let (ds, b) = has.attr(self);
                    if !b {
                        return DocumentSelection::default();
                    }
                    ds
                } else {
                    self
                }
            }

            "contains" => {
                if params.contains.is_some() {
                    let contains = params.contains.as_ref().unwrap();
                    let (ds, b) = contains.call(self);
                    if !b {
                        DocumentSelection::default()
                    } else {
                        ds
                    }
                } else {
                    self
                }
            }
            _ => self,
        };
    }

    fn content(self, params: &SelectParams) -> Value {
        if params.text_attr_html.is_none() {
            let text_attr_html = text_attr_html::TextAttrHtml::default();
            let v = text_attr_html.text(self);
            if v == "" {
                return params.get_default_val();
            }
            if params.data_format.is_none() {
                return Value::String(v);
            }
            let data_format = params.data_format.as_ref().unwrap();
            data_format.data_format(v)
        } else {
            let text_attr_html = params.text_attr_html.as_ref().unwrap();
            let v = text_attr_html.run(self);
            if v == "" {
                return params.get_default_val();
            }
            if params.data_format.is_none() {
                return Value::String(v);
            }
            let data_format = params.data_format.as_ref().unwrap();
            data_format.data_format(v)
        }
    }

    fn selects(mut self, params: &Vec<String>) -> DocumentSelection<'a> {
        for i in params.iter() {
            self = self.select(i);
        }
        self
    }

    fn select(self, params: &str) -> DocumentSelection<'a> {
        return match self {
            self::ParseSelection(d) => {
                let s = d.select(params);
                DocumentSelection::ParseSelection(s)
            }
            self::ParseDocument(d) => {
                let s = d.select(params);
                DocumentSelection::ParseSelection(s)
            }
            self::ParseNode(d) => {
                let mut s = Selection::from(d.to_owned());
                s = s.select(params);
                DocumentSelection::ParseSelection(s)
            }
        };
    }

    pub fn nodes(self) -> Vec<Node<'a>> {
        return match self {
            self::ParseDocument(d) => {
                let root = d.root();
                let v = [root].to_vec();
                v
            }
            self::ParseSelection(d) => d.nodes().to_vec(),
            self::ParseNode(d) => {
                let v = [d.to_owned()].to_vec();
                v
            }
        };
    }
}

use std::vec::Vec;
use nipper::{Document, Selection, Node};
use serde_json::{Value, Map};
use tendril::StrTendril;
use crate::document_selection::DocumentSelection::ParseSelection;
use crate::document_selection::DocumentSelection::ParseDocument;
use crate::document_selection::DocumentSelection::ParseNode;
use crate::parse::SelectParams;
use crate::replace;
use crate::deletes;
use crate::splits;
use crate::text_attr_html;

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

    pub fn parse(self, params: &'a SelectParams) -> Value{
        self.parse_html(params)
    }

    fn parse_html(mut self, params: &SelectParams) -> Value {
        if params.selects.is_some() {
            let s = params.selects.as_ref().unwrap();
            self = self.selects(s);
        }

        return if params.each.is_some() {
            let each = params.each.as_ref().as_ref().unwrap();
            self.each(each)
        } else if params.each_keys.is_some() {
            self.each_keys(params)
        } else {
            self.content(params)
        };
    }

    fn content(mut self, params: &SelectParams) -> Value {
        if params.nodes.is_some() {
            let cur_node = params.nodes.as_ref().unwrap();
            self = cur_node.run(self);
        }
        // self = self.first_last_eq_parent_children(params);
        if params.select_params.is_some() {
            let cur_params = params.select_params.as_ref().as_ref().unwrap();
            let val = self.parse(cur_params);
            return val
        }
        if params.has_class.is_some() {
            let s = self.has_class(params);
            if !s.1 {
                return params.get_default_val();
            }
            self = s.0
        }
        if params.has_attr.is_some() {
            let s = self.has_attr(params);
            if !s.1 {
                return params.get_default_val();
            }
            self = s.0
        }

        if params.text_attr_html.is_none() {
            let text_attr_html = text_attr_html::TextAttrHtml::default();
            let v = text_attr_html.text(self);
            contains_replaces_deletes_splits(v, params)
        } else {
            let text_attr_html = params.text_attr_html.as_ref().unwrap();
            let v = text_attr_html.run(self);
            contains_replaces_deletes_splits(v, params)
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

    fn nodes(self) -> Vec<Node<'a>> {
        return match self {
            self::ParseDocument(d) => {
                let root = d.root();
                let v = [root].to_vec();
                v
            }
            self::ParseSelection(d) => {
                d.nodes().to_vec()
            }
            self::ParseNode(d) => {
                let v = [d.to_owned()].to_vec();
                v
            }
        };
    }

    fn each(mut self, params: &SelectParams) -> Value {
        let nodes = self.nodes();
        let mut array =Vec::new();
        for node in nodes.iter() {
            self = DocumentSelection::ParseNode(node.to_owned());
            let v = self.parse(params);
            array.push(v);
        }
        Value::Array(array)
    }

    pub fn has_attr(self, params: &SelectParams) -> (DocumentSelection<'a>, bool) {
        let attr = params.has_attr.as_ref().unwrap();
        if attr == "" {
            return (self, false);
        }
        return match self {
            self::ParseDocument(d) => {
                let str_tendril = d.root().attr(attr).unwrap();
                let cur_str = str_tendril.trim();
                if cur_str == "" {
                    (DocumentSelection::ParseDocument(d), false)
                } else {
                    (DocumentSelection::ParseDocument(d), true)
                }
            }
            self::ParseSelection(d) => {
                let str_tendril = d.attr(attr).unwrap_or(StrTendril::default());
                let cur_str = str_tendril.trim();
                if cur_str == "" {
                    (DocumentSelection::ParseSelection(d), false)
                } else {
                    (DocumentSelection::ParseSelection(d), false)
                }
            }
            self::ParseNode(d) => {
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

    pub fn has_class(self, params: &SelectParams) -> (DocumentSelection<'a>, bool) {
        let class = params.has_attr.as_ref().unwrap();
        if class == "" {
            return (self, false);
        }
        return match self {
            self::ParseDocument(d) => {
                (DocumentSelection::ParseDocument(d), d.root().has_class(class))
            }
            self::ParseSelection(d) => {
                (DocumentSelection::ParseSelection(d.to_owned()), d.to_owned().has_class(class))
            }
            self::ParseNode(d) => {
                (DocumentSelection::ParseNode(d.to_owned()), d.to_owned().has_class(class))
            }
        };
    }

    fn each_keys(self, params: &SelectParams) -> Value {
        let each_keys = params.each_keys.as_ref().unwrap();
        let nodes = self.nodes();
        let mut array = Vec::new();
        for node in nodes.iter() {
            let mut cur_map = Map::new();
            for (k, v) in each_keys.iter() {
                let ds = DocumentSelection::ParseNode(node.to_owned());
                let c_val = ds.parse(v);
                let val = c_val;
                cur_map.insert(k.to_string(), Value::from(val));
            }
            array.push(Value::Object(cur_map));
        }
        Value::Array(array)
    }
}

fn contains_replaces_deletes_splits(v: String, params: &SelectParams) -> Value {
    if !params.contains_text(&v) ||
        !params.not_contains_text(&v) {
        return params.get_default_val();
    }
    let val = replaces_deletes_splits(params, v);
    val
}

fn replaces_deletes_splits(params: &SelectParams, mut v: String) -> Value {
    if params.replaces.is_some() {
        let replaces = params.replaces.as_ref().unwrap();
        v = replace::replaces(replaces, v);
    }

    if params.deletes.is_some() {
        let del = params.deletes.as_ref().unwrap();
        v = deletes::deletes(del, v);

    }
    if params.splits.is_some() {
        let s = params.splits.as_ref().unwrap();
        let val = splits::splits(s, v);
        return val
    }
    Value::String(String::from(v))
}
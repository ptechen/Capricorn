use std::collections::HashMap;
use serde::Deserialize;
use serde_json::{Value, Number, Map};
use std::vec::Vec;
use nipper::Document;
use crate::{replace::Replaces, splits::Splits};
use crate::deletes::Deletes;
use crate::node;
use crate::text_attr_html;
use crate::document_selection::DocumentSelection;

/// Parse html params
#[derive(Default, Deserialize, Clone, Debug)]
pub struct SelectParams {
    /// css selection
    pub selects: Option<Vec<String>>,
    pub nodes: Option<node::Node>,
    pub text_attr_html: Option<text_attr_html::TextAttrHtml>,
    pub each: Box<Option<SelectParams>>,
    pub each_keys: Option<HashMapSelectParams>,
    pub select_params: Box<Option<SelectParams>>,
    pub has_attr: Option<String>,
    pub has_class: Option<String>,
    pub splits: Option<Splits>,
    pub contains_text: Option<Vec<String>>,
    pub not_contains_text: Option<Vec<String>>,
    pub deletes: Option<Deletes>,
    pub replaces: Option<Replaces>,
    pub default_val_type: Option<String>,
}

pub type HashMapSelectParams = HashMap<String, SelectParams>;

/// Parse html entry
pub fn parse_html(params: &HashMapSelectParams, html: &String) -> Map<String, Value> {
    let doc = Document::from(html);
    let mut hmp = Map::new();
    for (k, v) in params.iter() {
        let ds = DocumentSelection::ParseDocument(&doc);
        let cur_parse = ds.parse(v);
        let cur_val = cur_parse;
        hmp.insert(k.to_string(), cur_val);
    }
    hmp
}


impl SelectParams {
    fn default_val(&self, params: &String) -> Value {
        if params == "String" || params == "str" {
            return Value::String(String::new());
        } else if params == "Array" || params == "array" || params == "vec" {
            return Value::Array(Vec::new());
        } else if params == "int" || params == "Int" {
            return Value::Number(Number::from(0));
        } else if params == "map" {
            return Value::Object(Map::new());
        } else if params == "bool" {
            return Value::Bool(false);
        }
        Value::default()
    }

    pub fn get_default_val(&self) -> Value {
        if self.default_val_type.is_some() {
            let default_val_type = self.default_val_type.as_ref().unwrap();
            let default = self.default_val(default_val_type);
            return default;
        }
        return Value::default();
    }

    pub fn contains_text(&self, text: &str) -> bool {
        if self.contains_text.is_some() {
            let params = self.contains_text.as_ref().unwrap();
            for pat in params.iter() {
                let b = text.contains(pat);
                if !b {
                    return false;
                }
            }
        };
        true
    }

    pub fn not_contains_text(&self, text: &str) -> bool {
        if self.contains_text.is_some() {
            let params = self.contains_text.as_ref().unwrap();
            for pat in params.iter() {
                let b = text.contains(pat);
                if b {
                    return false;
                }
            }
        }
        true
    }
}



use nipper::Document;
use serde::Deserialize;
use serde_json::{Map, Number, Value};
use std::collections::HashMap;
use std::vec::Vec;

use crate::contains;
use crate::data_format;
use crate::document_selection::DocumentSelection;
use crate::each;
use crate::has;
use crate::node;
use crate::text_attr_html;

/// Parse html params
#[derive(Default, Deserialize, Clone, Debug)]
pub struct SelectParams {
    pub exec_order: Option<Vec<String>>,
    /// css selection
    pub selects: Option<Vec<String>>,
    /// each: all、fields、one
    pub each: Option<each::Each>,
    pub select_params: Box<Option<SelectParams>>,
    pub nodes: Option<node::Node>,
    /// has: has class and has attr
    pub has: Option<has::Has>,
    /// contains: contains and not contains
    pub contains: Option<contains::Contains>,
    pub text_attr_html: Option<text_attr_html::TextAttrHtml>,
    pub data_format: Option<data_format::data_format::DataFormat>,
    pub default_val_type: Option<String>,
}

pub type HashMapSelectParams = HashMap<String, SelectParams>;

/// Parse html entry
pub fn parse_html(params: &HashMapSelectParams, html: &str) -> Map<String, Value> {
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
}

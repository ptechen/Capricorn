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
use crate::has;
use crate::contains;
use crate::each;

/// Parse html params
#[derive(Default, Deserialize, Clone, Debug)]
pub struct SelectParams {
    pub exec_order: Option<Vec<String>>,
    /// css selection
    pub selects: Option<Vec<String>>,
    pub nodes: Option<node::Node>,
    pub text_attr_html: Option<text_attr_html::TextAttrHtml>,
    /// each: all、fields、one
    pub each: Option<each::Each>,
    pub select_params: Box<Option<SelectParams>>,
    /// has: has class and has attr
    pub has: Option<has::Has>,
    /// contains: contains and not contains
    pub contains: Option<contains::Contains>,
    pub splits: Option<Splits>,
    pub deletes: Option<Deletes>,
    pub replaces: Option<Replaces>,
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



use crate::document_selection::DocumentSelection;
use crate::parse::HashMapSelectParams;
use crate::parse::SelectParams;
use serde::Deserialize;
use serde_json::{Map, Value};

/// Each: Only one of all, fields and one can exist at the same time.
#[derive(Default, Deserialize, Clone, Debug)]
pub struct Each {
    pub all: Box<Option<SelectParams>>,
    pub one: Box<Option<SelectParams>>,
    pub fields: Option<HashMapSelectParams>,
}

impl Each {
    pub fn each(&self, ds: DocumentSelection) -> Value {
        return if self.all.is_some() {
            self.all(ds)
        } else if self.fields.is_some() {
            self.fields(ds)
        } else {
            self.one(ds)
        };
    }

    fn all(&self, mut ds: DocumentSelection) -> Value {
        let params = self.all.as_ref().as_ref().unwrap();
        let nodes = ds.nodes();
        let mut array = Vec::new();
        for node in nodes.iter() {
            ds = DocumentSelection::ParseNode(node.to_owned());
            let v = ds.parse(params);
            if v.is_null() {
                continue;
            }
            array.push(v);
        }
        Value::Array(array)
    }

    fn one(&self, mut ds: DocumentSelection) -> Value {
        let params = self.one.as_ref().as_ref().unwrap();
        let nodes = ds.nodes();
        for node in nodes.iter() {
            ds = DocumentSelection::ParseNode(node.to_owned());
            let v = ds.parse(params);
            if v.is_null() {
                continue;
            }
            return v;
        }
        params.get_default_val()
    }

    fn fields(&self, ds: DocumentSelection) -> Value {
        let each_keys = self.fields.as_ref().unwrap();
        let nodes = ds.nodes();
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

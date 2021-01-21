use crate::replace;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Debug)]
pub struct Split {
    pub key: Option<String>,
    pub index: Option<usize>,
}

impl Split {
    pub fn split(&self, params: Value) -> Value {
        if self.key.is_some() {
            let key = self.key.as_ref().unwrap();
            let key = replace::special_char(key);
            let val: Vec<&str> = params.as_str().unwrap().split(&key).collect();
            if self.index.is_some() {
                let index = self.index.unwrap();
                if index > val.len() - 1 {
                    return Value::default();
                }
                let v = val.get(index).unwrap();
                return Value::from(*v);
            }
            return Value::from(val);
        }
        Value::default()
    }
}

pub(crate) type Splits = Vec<Split>;

pub fn splits(sps: &Splits, params: String) -> Value {
    let mut val = Value::String(params);
    for split in sps.iter() {
        val = split.split(val);
    }
    if val.is_null() {
        val = Value::Array(Vec::new())
    }
    val
}

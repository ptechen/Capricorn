use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Clone, Debug)]
pub struct Split {
    pub key: Option<String>,
    pub index: Option<usize>,
    pub enable: Option<bool>,
}

impl Split {
    pub fn split(&self, params: Value) -> Value {
        if self.key.is_some() {
            let key = self.key.as_ref().unwrap();
            let val:Vec<&str> = params.as_str().unwrap().split(key).collect();
            if self.enable.is_some() && self.index.is_some(){
                let enable = self.enable.unwrap();
                if enable {
                    let index = self.index.unwrap();
                    if index > val.len() - 1 {
                        return Value::default()
                    }
                    let v = val.get(index).unwrap();
                    return Value::from(*v)
                }
            }
            return Value::from(val)
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
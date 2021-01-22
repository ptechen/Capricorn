use crate::data_format;
use serde::Deserialize;
use serde_json::Value;

/// Has: Class and attr can exist at the same time.
#[derive(Deserialize, Clone, Debug)]
pub struct DataFormat {
    pub splits: Option<data_format::splits::Splits>,
    pub deletes: Option<data_format::deletes::Deletes>,
    pub replaces: Option<data_format::replaces::Replaces>,
    pub find: Option<String>,
    pub find_iter: Option<String>,
}

impl DataFormat {
    pub fn data_format(&self, mut v: String) -> Value {
        if self.replaces.is_some() {
            let replaces = self.replaces.as_ref().unwrap();
            v = data_format::replaces::replaces(replaces, v);
        }

        if self.deletes.is_some() {
            let del = self.deletes.as_ref().unwrap();
            v = data_format::deletes::deletes(del, v);
        }

        if self.splits.is_some() {
            let s = self.splits.as_ref().unwrap();
            let val = data_format::splits::splits(s, v);
            return val;
        }

        if self.find.is_some() {
            return self.find(&v);
        }

        if self.find_iter.is_some() {
            return self.find_iter(&v);
        }
        Value::String(String::from(v))
    }

    fn find(&self, v: &str) -> Value {
        let re = self.find.as_ref().unwrap();
        let r = regex::Regex::new(re);
        if r.is_ok() {
            let r = r.unwrap();
            if r.is_match(v) {
                let v = r.find(v).unwrap().as_str();
                return Value::String(v.to_string());
            }
        }
        Value::String(v.to_string())
    }

    fn find_iter(&self, v: &str) -> Value {
        let re = self.find_iter.as_ref().unwrap();
        let r = regex::Regex::new(re);
        if r.is_ok() {
            let r = r.unwrap();
            if r.is_match(v) {
                let v: Vec<Value> = r
                    .find_iter(v)
                    .map(|mat| Value::String(String::from(mat.as_str())))
                    .collect();
                return Value::Array(v);
            }
        }
        let array = vec![Value::String(v.to_string())];
        Value::Array(array)
    }
}

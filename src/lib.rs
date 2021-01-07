pub mod parse;
mod replace;
mod splits;
mod deletes;
mod document_selection;
mod node;
mod text_attr_html;


#[cfg(test)]
mod tests {
    use std::collections::hash_map::HashMap;
    use quicli::prelude::*;
    use serde_yaml;
    use serde_json::Value;
    use crate::parse_html;
    use crate::parse::SelectParams;

    #[test]
    fn test_run() -> Result<(), Box<dyn std::error::Error>> {
        let yml = read_file("./test_html/test.yml").unwrap();
        let v: HashMap<String, SelectParams> = serde_yaml::from_str(&yml).unwrap();
        let html = read_file("./test_html/test.html").unwrap();
        let r = parse_html(html, v);
        assert_eq!(r.get("splits").unwrap(), "ff");
        assert_eq!(r.get("last").unwrap(), "last");
        assert_eq!(r.get("last1").unwrap(), "");
        assert_eq!(r.get("text").unwrap(), "bbbbbbbbb");
        assert_eq!(r.get("first").unwrap(), "first");
        assert_eq!(r.get("eq").unwrap(), "first");
        assert_eq!(r.get("eeq").unwrap(), "");
        assert_eq!(r.get("replace").unwrap(), "!!!!!!");
        assert_eq!(r.get("attr").unwrap(), "/te\n    st");
        assert_eq!(r.get("deletes").unwrap(), "/test");
        assert_eq!(r.get("deletes").unwrap(), "/test");
        assert_eq!(r.get("has_attr_splits").unwrap(), "ff");
        assert_eq!(r.get("default_val_type").unwrap().as_str().unwrap(), "");
        assert_eq!(r.get("html").unwrap(), "<a class=\"a\" href=\"/te\n    st\">111111</a>");
        let v = vec![Value::String(String::from("last")), Value::String(String::from("last"))];
        assert_eq!(r.get("each").unwrap().as_array().unwrap(), &v);
        assert_eq!(r.get("prev_sibling").unwrap(), "prev");
        assert_eq!(r.get("next_sibling").unwrap(), "next");
        assert_eq!(r.get("parent").unwrap(), "parent");
        assert_eq!(r.get("children").unwrap(), "children1children2");
        println!("{:?}", r);
        Ok(())
    }
}

use std::collections::HashMap;
use serde_json::{Value, Map};
use nipper::Document;
use crate::parse::SelectParams;
use crate::document_selection::DocumentSelection;

/// Parse html entry
pub fn parse_html(html: String, select_params: HashMap<String, SelectParams>) -> Map<String, Value> {
    let doc = Document::from(&html);
    let mut hmp = Map::new();
    for (k, v) in select_params.iter() {
        let ds = DocumentSelection::ParseDocument(&doc);
        let cur_parse = ds.parse(v);
        let cur_val = cur_parse;
        hmp.insert(k.to_string(), cur_val);
    }
    hmp
}




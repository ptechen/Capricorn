#[macro_use]
extern crate lazy_static;
mod contains;
mod data_format;
mod document_selection;
mod each;
mod has;
pub mod match_html;
mod node;
pub mod parse;
mod text_attr_html;

#[cfg(test)]
mod tests {
    use crate::match_html;
    use crate::parse;
    use quicli::prelude::*;
    use serde_json::Value;
    use serde_yaml;
    /// test parse html function
    #[test]
    fn test_parse_html() -> Result<(), Box<dyn std::error::Error>> {
        let yml = read_file("./test_pages/test.yml").unwrap();
        let params: parse::HashMapSelectParams = serde_yaml::from_str(&yml).unwrap();
        let html = read_file("./test_pages/test.html").unwrap();
        let r = parse::parse_html(&params, &html);
        assert_eq!(r.get("splits").unwrap(), "ff");
        assert_eq!(r.get("last").unwrap(), "last");
        assert_eq!(r.get("text").unwrap(), "bbbbbbbbb");
        assert_eq!(r.get("first").unwrap(), "first");
        assert_eq!(r.get("eq").unwrap(), "first");
        assert_eq!(r.get("replace").unwrap(), "!!!!!!");
        assert_eq!(r.get("attr").unwrap(), "/te!    st");
        assert_eq!(r.get("deletes").unwrap(), "/test");
        assert_eq!(r.get("has_attr_splits").unwrap(), "ff");
        assert_eq!(r.get("default_val_type").unwrap().as_str().unwrap(), "");
        assert_eq!(
            r.get("html").unwrap(),
            "<a class=\"a\" href=\"/te\n    st\">111111</a>"
        );
        let v = vec![
            Value::String(String::from("last")),
            Value::String(String::from("last")),
        ];
        assert_eq!(r.get("each").unwrap().as_array().unwrap(), &v);
        assert_eq!(r.get("prev_sibling").unwrap(), "prev");
        assert_eq!(r.get("next_sibling").unwrap(), "next");
        assert_eq!(r.get("parent").unwrap(), "parent");
        assert_eq!(r.get("children").unwrap(), "children1children2");
        println!("{:?}", r);
        Ok(())
    }

    /// test regexes match parse html function
    #[test]
    fn test_regexes_match_parse_html() -> Result<(), Box<dyn std::error::Error>> {
        let yml = read_file("./test_pages/regexes_match_parse_html.yml").unwrap();
        let v: match_html::MatchHtmlVec = serde_yaml::from_str(&yml).unwrap();
        let html = read_file("./test_pages/test.html").unwrap();
        let r = v.regexes_match_parse_html(&html)?;
        assert_eq!(r.get("splits").unwrap(), "ff");
        assert_eq!(r.get("last").unwrap(), "last");
        assert_eq!(r.get("text").unwrap(), "bbbbbbbbb");
        assert_eq!(r.get("first").unwrap(), "first");
        assert_eq!(r.get("eq").unwrap(), "first");
        assert_eq!(r.get("replace").unwrap(), "!!!!!!");
        assert_eq!(r.get("attr").unwrap(), "/te!    st");
        assert_eq!(r.get("deletes").unwrap(), "/test");
        assert_eq!(r.get("deletes").unwrap(), "/test");
        assert_eq!(r.get("has_attr_splits").unwrap(), "ff");
        assert_eq!(r.get("default_val_type").unwrap().as_str().unwrap(), "");
        assert_eq!(
            r.get("html").unwrap(),
            "<a class=\"a\" href=\"/te\n    st\">111111</a>"
        );
        let v = vec![
            Value::String(String::from("last")),
            Value::String(String::from("last")),
        ];
        assert_eq!(r.get("each").unwrap().as_array().unwrap(), &v);
        assert_eq!(r.get("prev_sibling").unwrap(), "prev");
        assert_eq!(r.get("next_sibling").unwrap(), "next");
        assert_eq!(r.get("parent").unwrap(), "parent");
        assert_eq!(r.get("children").unwrap(), "children1children2");
        println!("{:?}", r);
        Ok(())
    }
}

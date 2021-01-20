use capricorn::parse;
use quicli::prelude::*;
#[test]
fn test_text() -> Result<(), Box<dyn std::error::Error>> {
    let yml = read_file("./test_pages/text_attr_html.yml").unwrap();
    let params: parse::HashMapSelectParams = serde_yaml::from_str(&yml).unwrap();
    let html = read_file("./test_pages/text_attr_html.html").unwrap();
    let r = parse::parse_html(&params, &html);
    println!("{:?}", r);
    assert_eq!(r.get("text").unwrap(), "text");
    assert_eq!(r.get("attr").unwrap(), "/attr");
    assert_eq!(r.get("html").unwrap(), "<a href=\"/attr\">test</a>");
    Ok(())
}

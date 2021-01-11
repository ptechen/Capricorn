use capricorn::parse;
use quicli::prelude::*;
#[test]
fn test_attr() -> Result<(), Box<dyn std::error::Error>> {
    let yml = read_file("./test_pages/attr.yml").unwrap();
    let params: parse::HashMapSelectParams = serde_yaml::from_str(&yml).unwrap();
    let html = read_file("./test_pages/attr.html").unwrap();
    let r = parse::parse_html(&params, &html);
    println!("{:?}", r);
    Ok(())
}
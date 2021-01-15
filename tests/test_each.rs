use capricorn::parse;
use quicli::prelude::*;
#[test]
fn test_each() -> Result<(), Box<dyn std::error::Error>> {
    let yml = read_file("./test_pages/each.yml").unwrap();
    let params: parse::HashMapSelectParams = serde_yaml::from_str(&yml).unwrap();
    let html = read_file("./test_pages/each.html").unwrap();
    let r = parse::parse_html(&params, &html);
    println!("{:?}", r);
    Ok(())
}
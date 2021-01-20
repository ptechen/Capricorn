use capricorn::parse;
use quicli::prelude::*;
#[test]
fn test_last() -> Result<(), Box<dyn std::error::Error>> {
    let yml = read_file("./test_pages/replace.yml").unwrap();
    let params: parse::HashMapSelectParams = serde_yaml::from_str(&yml).unwrap();
    let html = read_file("./test_pages/replace.html").unwrap();
    let r = parse::parse_html(&params, &html);
    println!("{:?}", r);
    assert_eq!(r.get("replace").unwrap(), "replace_success");
    Ok(())
}

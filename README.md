# Capricorn

### Parse html according to configuration.
### Capricorn is a html parsing library that supports recursion and custom execution order.
[![Version info](https://img.shields.io/crates/v/capricorn.svg)](https://crates.io/crates/capricorn)
[![Downloads](https://img.shields.io/crates/d/capricorn.svg?style=flat-square)](https://crates.io/crates/capricorn)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/capricorn)
[![example branch parameter](https://github.com/ptechen/capricorn/workflows/CI/badge.svg?branch=main)]()
[![dependency status](https://deps.rs/crate/capricorn/0.1.91/status.svg)](https://deps.rs/crate/capricorn)

### Default execution order
    vec![String::from("selects"),
        String::from("each"),
        String::from("select_params"),
        String::from("nodes"),
        String::from("has"),
        String::from("contains")];
        
    selects > each > (one or all or fields) > ... text_attr_html > (text or attr or html);
    selects > select_params > selects > ... text_attr_html > (text or attr or html);
    selects > nodes > has > contains > text_attr_html > (text or attr or html);
                                                      
    
### Support:
| Capricorn | support | example |val type|
| :----: | :----: | :----- |:----:|
| selects element | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name | String |
| selects class | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - .class_name | String | 
| selects class element | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - .class_name <br> &nbsp; &nbsp; &nbsp; - element_name | String | 
| first | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; nodes: <br> &nbsp; &nbsp; &nbsp; first: true | String | 
| last | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; nodes: <br> &nbsp; &nbsp; &nbsp; last: true | String | 
| eq | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; nodes: <br> &nbsp; &nbsp; &nbsp; eq: 0 | String | 
| parent | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; nodes: <br> &nbsp; &nbsp; &nbsp; parent: true | String | 
| children | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; nodes: <br> &nbsp; &nbsp; &nbsp; children: true | String | 
| prev_sibling | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; nodes: <br> &nbsp; &nbsp; &nbsp; prev_sibling: true | String | 
| next_sibling | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; nodes: <br> &nbsp; &nbsp; &nbsp; next_sibling: true | String | 
| has_class | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; has: <br> &nbsp; &nbsp; &nbsp; class: class_name | String | 
| has_attr | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; has: <br> &nbsp; &nbsp; &nbsp; attr: attr_name | String | 
| each one | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; each: <br> &nbsp; &nbsp; &nbsp; one: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; selects:<br>&nbsp; &nbsp; &nbsp;  &nbsp; &nbsp; &nbsp; &nbsp; - .class_name<br>&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;  ... | String | 
| each all | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; each: <br> &nbsp; &nbsp; &nbsp; all: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; selects:<br>&nbsp; &nbsp; &nbsp;  &nbsp; &nbsp; &nbsp; &nbsp; - .class_name<br>&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;  ... | Array | 
| each fields | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; each: <br> &nbsp; &nbsp; &nbsp; fields: <br> &nbsp; &nbsp; &nbsp; &nbsp; field_name: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; selects:<br>&nbsp; &nbsp; &nbsp;  &nbsp; &nbsp; &nbsp; &nbsp; - .class_name<br>&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;  ... <br> &nbsp; &nbsp; &nbsp; &nbsp; field_name1: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; selects:<br>&nbsp; &nbsp; &nbsp;  &nbsp; &nbsp; &nbsp; &nbsp; - .class_name<br>&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;  ...  | Map | 
| select_params | ✔ | field_name: <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; select_params: <br> &nbsp; &nbsp; &nbsp; selects:<br>&nbsp; &nbsp; &nbsp;  &nbsp; &nbsp; - .class_name<br>&nbsp; &nbsp; &nbsp;  ... | ... | 
| text | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; text_attr_html: <br> &nbsp; &nbsp; &nbsp; text: true | String |
| attr | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; text_attr_html: <br> &nbsp; &nbsp; &nbsp; attr: true | String |
| html | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; text_attr_html: <br> &nbsp; &nbsp; &nbsp; html: true | String |
| text contains | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; contains: <br> &nbsp; &nbsp; &nbsp; contains: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; text: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; - test | String |
| text not contains | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; contains: <br> &nbsp; &nbsp; &nbsp; not_contains: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; text: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; - test | String |
| html contains | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; contains: <br> &nbsp; &nbsp; &nbsp; contains: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; html: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; - test | String |
| html not contains | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; contains: <br> &nbsp; &nbsp; &nbsp; not_contains: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; html: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; - test | String |
| exec order | ✔ | field_name:<br> &nbsp; exec_order: <br> &nbsp; &nbsp; &nbsp; - selects <br> &nbsp; &nbsp; &nbsp; - has <br> &nbsp; &nbsp; &nbsp; - nodes <br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; has: <br> &nbsp; &nbsp; &nbsp; class: class_name <br> &nbsp; nodes: <br> &nbsp; &nbsp; &nbsp; first: true | String |
| data format splits | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; data_format: <br> &nbsp; &nbsp; &nbsp; splits: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; - { key: str } | Array |
| data format splits | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; data_format: <br> &nbsp; &nbsp; &nbsp; splits: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; - { key: str, index: 0 } | String |
| data format replaces | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; data_format: <br> &nbsp; &nbsp; &nbsp; replaces: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; - str | String |
| data format deletes | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; data_format: <br> &nbsp; &nbsp; &nbsp; deletes: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; - str | String |
| data format find | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; data_format: <br> &nbsp; &nbsp; &nbsp; find: <br> &nbsp; &nbsp; &nbsp; &nbsp; - regex | String |
| data format find_iter | ✔ | field_name:<br> &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; - element_name <br> &nbsp; data_format: <br> &nbsp; &nbsp; &nbsp; find_iter: <br> &nbsp; &nbsp; &nbsp; &nbsp; - regex | Array |
| Multi-version regular matching err | ✔ |regexes_match_parse_html: <br>  &nbsp; &nbsp; - regex: regex <br> &nbsp; &nbsp; &nbsp; version: 1 <br> &nbsp; &nbsp; &nbsp; err: err_msg | Err |
| Multi-version regular matching fields | ✔ |regexes_match_parse_html: <br>  &nbsp; &nbsp; - regex: regex <br> &nbsp; &nbsp; &nbsp; version: 1 <br> &nbsp; &nbsp; &nbsp; fields: <br> &nbsp; &nbsp; &nbsp; &nbsp; field_name: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; ... <br> &nbsp; &nbsp; &nbsp; &nbsp; field_name: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; selects: <br> &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; ... | Map |

    
#### [Parse html code, more...](https://github.com/ptechen/Capricorn/blob/main/src/lib.rs)
    let yml = read_file("./test_html/test.yml").unwrap();
    let params: parse::HashMapSelectParams = serde_yaml::from_str(&yml).unwrap();
    let html = read_file("./test_html/test.html").unwrap();
    let r = parse::parse_html(&params, &html);
    
#### [Multi-version regular matching parsing html code, more...](https://github.com/ptechen/Capricorn/blob/main/src/lib.rs)
    let yml = read_file("./test_html/regexes_match_parse_html.yml").unwrap();
    let v:  match_html::MatchHtmlVec = serde_yaml::from_str(&yml).unwrap();
    let html = read_file("./test_html/test.html").unwrap();
    let r =  v.regexes_match_parse_html(html)?;
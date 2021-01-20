# Capricorn

### Parse html according to configuration.
### Capricorn is a html parsing library that supports recursion and custom execution order.
[![Version info](https://img.shields.io/crates/v/capricorn.svg)](https://crates.io/crates/capricorn)
[![Downloads](https://img.shields.io/crates/d/capricorn.svg?style=flat-square)](https://crates.io/crates/capricorn)
[![Build Status](https://www.travis-ci.org/ptechen/Capricorn.svg?branch=main)](https://www.travis-ci.org/ptechen/Capricorn)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/capricorn)

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
                                                      
    
### Example:
#### [Parse html, test.yml, more...](https://github.com/ptechen/Capricorn/blob/main/test_html/test.yml)
    last:
      selects:
        - '.a'
      nodes:
        last: true
    #  text_attr_html: # Default text, can be omitted
    #    text: true
    
    last1:
      selects:
        - '.aa'
      nodes:
        last: true

#### [Multi-version regular matching parsing html, regexes_match_parse_html.yml, more...](https://github.com/ptechen/Capricorn/blob/main/test_html/regexes_match_parse_html.yml)
    regexes_match_parse_html:
      - regex: 'error'
        version: 1
        err: "" # Custom error message, return error message directly if the regular expression matches successfully
        fields:
          last:
            selects:
              - '.a'
            nodes:
              last: true
          #  text_attr_html: # Default text, can be omitted
          #    text: true
    
      - regex: '.*?'
        version: 1
        fields:
          last:
            selects:
              - '.a'
            nodes:
              last: true
          #  text_attr_html: # Default text, can be omitted
          #    text: true
    
          last1:
            selects:
              - '.aa'
            nodes:
              last: true
    
          text:
            selects:
              - '.b'
    
          first:
            selects:
              - '.a'
            nodes:
              first: true
     
#### test.html:
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Title</title>
    </head>
    <body>
    <a class="a">first</a>
    <a class="a" href="/te
        st">111111</a>
    <a class="a">last</a>
    <a class="b">bbb</a>
    <a class="f" href="fff">fffddffeddggdd</a>
    <a class="f" href="http://www.test.com">fffddffeddggdd</a>
    <ul>
        <li>
            <a class="a">last</a>
            <a class="b">bbb</a>
        </li>
        <li>
            <a class="a">last</a>
            <a class="b">bbb</a>
        </li>
    </ul>
    
    <div>
        <a class="parent">parent</a>
    </div>
    <div>
        <a>prev</a>
    </div>
    <div class="children">
        <a>children1</a>
        <a>children2</a>
    </div>
    <div>
        <a>next</a>
    </div>
    </body>
    </html>
    
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
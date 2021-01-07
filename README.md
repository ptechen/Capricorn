# Capricorn

### Parse html according to configuration
[![Version info](https://img.shields.io/crates/v/capricorn.svg)](https://crates.io/crates/capricorn)

### Example:
#### test.yml    
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
    
    eq:
      selects:
        - '.a'
      nodes:
        eq: 0
    
    eeq:
      selects:
        - '.aa'
      nodes:
        eq: 0
    
    html:
      selects:
        - '.a'
      nodes:
        eq: 1
      text_attr_html:
        html: true
    
    replace:
      selects:
        - '.a'
      nodes:
        eq: 1
      replaces: [{from: "1", to: "!"}]
    
    attr:
      selects:
        - '.a'
      nodes:
        eq: 1
      text_attr_html:
        attr: 'href'
    
    deletes:
      selects:
        - '.a'
      nodes:
        eq: 1
      text_attr_html:
        attr: 'href'
      deletes:
        - ' '
        - '\n'
    
    splits:
      selects:
        - '.f'
      splits:
        - {key: 'e', index: 0, enable: true}
        - {key: 'dd', index: 1, enable: true}
    
    default_val_type:
      selects:
        - '.f'
      has_attr: 'href1dd'
      splits:
        - {key: 'e', index: 0, enable: true}
        - {key: 'dd', index: 1, enable: true}
      default_val_type: "str"
    
    has_attr_splits:
      selects:
        - '.f'
      nodes:
        eq: 1
      has_attr: 'href'
      splits:
        - {key: 'e', index: 0, enable: true}
        - {key: 'dd', index: 1, enable: true}
      default_val_type: "vec"
    
    each_keys:
      selects:
        - 'ul'
        - 'li'
      each_keys:
        aaaa:
          selects:
            - '.a'
        cccc:
          selects:
            - '.b'
    
    each:
      selects:
        - 'ul'
        - 'li'
      each:
        selects:
          - '.a'
    
    #oooo:
    #  html: true
    
    
    eq_selects:
      selects:
        - 'ul'
        - 'li'
      nodes:
        eq: 0
      select_params:
        selects:
          - '.a'
    
    parent:
      selects:
        - '.parent'
      nodes:
        parent: true
    
    children:
      selects:
        - '.children'
      nodes:
        children: true
    
    prev_sibling:
      selects:
        - '.children'
      nodes:
        prev_sibling: true
    
    next_sibling:
      selects:
        - '.children'
      nodes:
        next_sibling: true
        
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
    
#### Code(https://github.com/ptechen/Capricorn/blob/main/src/lib.rs)
    let yml = read_file("./test_html/test.yml").unwrap();
    let v: HashMap<String, SelectParams> = serde_yaml::from_str(&yml).unwrap();
    let html = read_file("./test_html/test.html").unwrap();
    let r = parse_html(html, v);
    

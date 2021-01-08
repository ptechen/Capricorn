use std::vec::Vec;

pub(crate) type Deletes = Vec<String>;

pub fn deletes(del: &Deletes, mut params: String) -> String {
    for d in del.iter(){
        if d == "\\n" {
            params = params.replace("\n", "")
        } else if d == "\\t" {
            params = params.replace("\t", "")
        } else if d == "\\r" {
            params = params.replace("\r", "")
        } else {
            params = params.replace(d, "")
        }
    }
    params
}


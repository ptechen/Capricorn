use crate::replace;
use std::vec::Vec;

pub(crate) type Deletes = Vec<String>;

pub fn deletes(del: &Deletes, mut params: String) -> String {
    for d in del.iter() {
        let d = replace::special_char(d);
        params = params.replace(&d, "")
    }
    params
}

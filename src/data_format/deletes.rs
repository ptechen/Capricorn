use crate::data_format;
use std::vec::Vec;

pub(crate) type Deletes = Vec<String>;

pub fn deletes(del: &Deletes, mut params: String) -> String {
    for d in del.iter() {
        let d = data_format::replaces::special_char(d);
        params = params.replace(&d, "")
    }
    params
}

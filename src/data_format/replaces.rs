use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Replace {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl Replace {
    fn replace(&self, params: &String) -> String {
        if self.from.is_none() || self.to.is_none() {
            return String::from(params);
        }
        let from = self.from.as_ref().unwrap();
        let from = &special_char(from);
        let to = self.to.as_ref().unwrap();
        let to = &special_char(to);
        params.replace(from, to)
    }
}

pub(crate) type Replaces = Vec<Replace>;

pub fn replaces(reps: &Replaces, mut params: String) -> String {
    for cur in reps.iter() {
        params = cur.replace(&params);
    }
    String::from(params)
}

pub fn special_char(params: &str) -> String {
    if params.contains("\\n") {
        let cur = params.replace("\\n", "\n");
        return cur;
    } else if params.contains("\\t") {
        let cur = params.replace("\\t", "\t");
        return cur;
    } else if params.contains("\\r") {
        let cur = params.replace("\\r", "\r");
        return cur;
    }
    String::from(params)
}

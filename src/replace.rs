use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Replace {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl Replace {
    fn replace(&self, params: &String) -> String {
        if self.from.is_none() || self.to.is_none(){
            return String::from(params)
        }
        let from = self.from.as_ref().unwrap();
        let from = &special_char(from.to_string());
        let to = self.to.as_ref().unwrap();
        let to = &special_char(to.to_string());
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

fn special_char(params: String) -> String {
    if "\\n" == params {
        let cur = String::from("\n");
        return cur
    } else if params == "\\t" {
        let cur = String::from("\t");
        return cur
    }
    params
}
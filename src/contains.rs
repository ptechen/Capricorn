use serde::Deserialize;
use crate::replace;
#[derive(Deserialize, Clone, Debug)]
pub struct Contains {
    pub contains: Option<Vec<String>>,
    pub not_contains: Option<Vec<String>>,
}

impl Contains {
    pub fn contains(&self, text: &str) -> bool {
        if self.contains.is_none() {
            return true;
        }

        let params = self.contains.as_ref().unwrap();
        for pat in params.iter() {
            let pat  = replace::special_char(pat);
            let b = text.contains(&pat);
            if !b {
                return false;
            }
        }

        true
    }

    pub fn not_contains(&self, text: &str) -> bool {
        if self.not_contains.is_none() {
            return true;
        }

        let params = self.not_contains.as_ref().unwrap();
        for pat in params.iter() {
            let pat  = replace::special_char(pat);
            let b = text.contains(&pat);
            if b {
                return false;
            }
        }

        true
    }
}
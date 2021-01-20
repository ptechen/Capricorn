use crate::parse;
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use serde_json::{Map, Value};
use std::error::Error;
use std::vec::Vec;

/// Choose different parsing configurations based on regular matching
#[derive(Deserialize, Clone)]
pub struct MatchHtmlVec {
    pub regexes_match_parse_html: Option<Vec<MatchHtml>>,
}

#[derive(Deserialize, Clone)]
pub struct MatchHtml {
    /// Regex match html
    pub regex: Option<String>,
    /// Custom error message, return error message directly if the regular expression matches successfully
    pub err: Option<String>,
    /// Parse the configuration of html
    pub fields: Option<parse::HashMapSelectParams>,
    /// Add version, you can not add
    pub version: Option<String>,
}

impl MatchHtmlVec {
    pub fn regexes_match_parse_html(
        &self,
        html: &str,
    ) -> Result<Map<String, Value>, Box<dyn Error>> {
        if self.regexes_match_parse_html.is_none() {
            return Err(Box::from("regexes does not exist"));
        }
        for match_html in self.regexes_match_parse_html.as_ref().unwrap().iter() {
            if match_html.regex.is_some() && match_html.fields.is_some() {
                let regular = match_html.regex.as_ref().unwrap();
                let r = Regex::new(regular)?;
                if r.is_match(html) {
                    if match_html.err.is_some() {
                        let err = match_html.err.as_ref().unwrap();
                        return Err(Box::from(err.to_string()));
                    }
                    let select_params = match_html.fields.as_ref().unwrap();
                    let mut val = parse::parse_html(select_params, html);
                    if match_html.version.is_some() {
                        let version = match_html.version.as_ref().unwrap();
                        val.insert(String::from("version"), json!(version));
                    }
                    return Ok(val);
                } else {
                    continue;
                }
            }
        }
        return Err(Box::from("All regexes do not match"));
    }
}

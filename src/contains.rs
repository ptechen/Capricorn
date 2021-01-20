use crate::document_selection::DocumentSelection;
use crate::replace;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Contains {
    pub contains: Option<TextHtml>,
    pub not_contains: Option<TextHtml>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TextHtml {
    pub html: Option<Vec<String>>,
    pub text: Option<Vec<String>>,
}

impl TextHtml {
    fn text<'a>(&self, ds: DocumentSelection<'a>) -> (DocumentSelection<'a>, bool) {
        if self.text.is_none() {
            return (ds, true);
        }

        let params = self.text.as_ref().unwrap();
        let (ds, html) = ds.text();
        let b = self.contains(params, html);
        return (ds, b);
    }

    fn contains(&self, params: &Vec<String>, content: String) -> bool {
        for pat in params.iter() {
            let pat = replace::special_char(pat);
            let b = content.contains(&pat);
            if !b {
                return b;
            }
        }
        true
    }

    fn html<'a>(&self, ds: DocumentSelection<'a>) -> (DocumentSelection<'a>, bool) {
        if self.html.is_none() {
            return (ds, true);
        }

        let params = self.html.as_ref().unwrap();
        let (ds, html) = ds.html();
        let b = self.contains(params, html);
        return (ds, b);
    }
}

impl Contains {
    pub fn call<'a>(&self, mut ds: DocumentSelection<'a>) -> (DocumentSelection<'a>, bool) {
        if self.contains.is_some() {
            let (d, b) = self.contains(ds);
            if !b {
                return (d, b);
            }
            ds = d
        }
        if self.not_contains.is_some() {
            return self.not_contains(ds);
        }
        (ds, true)
    }

    fn contains<'a>(&self, ds: DocumentSelection<'a>) -> (DocumentSelection<'a>, bool) {
        let contains = self.contains.as_ref().unwrap();
        let (ds, b) = contains.text(ds);
        if !b {
            return (ds, b);
        }
        return contains.html(ds);
    }

    fn not_contains<'a>(&self, ds: DocumentSelection<'a>) -> (DocumentSelection<'a>, bool) {
        let not_contains = self.not_contains.as_ref().unwrap();
        let (ds, b) = not_contains.text(ds);
        if !b {
            return (ds, true);
        }
        let (ds, b) = not_contains.html(ds);
        if !b {
            return (ds, true);
        }
        (ds, b)
    }
}

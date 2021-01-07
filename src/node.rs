use crate::document_selection::DocumentSelection;
use nipper::Selection;
use serde::Deserialize;

#[derive(Default,Deserialize, Clone, Debug)]
pub struct Node {
    pub first: Option<bool>,
    pub last: Option<bool>,
    pub eq: Option<usize>,
    pub parent: Option<bool>,
    pub children: Option<bool>,
    pub prev_sibling: Option<bool>,
    pub next_sibling: Option<bool>,
}

impl Node {
    pub fn run<'a>(&self, params: DocumentSelection<'a>) -> DocumentSelection<'a> {
        if self.first.is_some() {
            return self.first(params);
        } else if self.last.is_some() {
            return self.last(params);
        } else if self.eq.is_some() {
            return self.eq(params);
        } else if self.parent.is_some() {
            return self.parent(params);
        } else if self.children.is_some() {
            return self.children(params);
        } else if self.prev_sibling.is_some() {
            return self.prev_sibling(params);
        } else if self.next_sibling.is_some() {
            return self.next_sibling(params);
        }
        params
    }

    fn last<'a>(&self, params: DocumentSelection<'a>) -> DocumentSelection<'a> {
        if self.last.unwrap() {
            return match params {
                DocumentSelection::ParseSelection(d) => {
                    let nodes = d.nodes().to_vec();
                    if nodes.len() > 0 {
                        let node = nodes.last().unwrap();
                        DocumentSelection::ParseNode(node.to_owned())
                    } else {
                        DocumentSelection::default()
                    }
                }
                _ => {
                    params
                }
            };
        }
        params
    }

    fn first<'a>(&self, params: DocumentSelection<'a>) -> DocumentSelection<'a> {
        if self.first.unwrap() {
            return match params {
                DocumentSelection::ParseSelection(d) => {
                    let nodes = d.nodes().to_vec();
                    if nodes.len() > 0 {
                        let node = nodes.first().unwrap();
                        DocumentSelection::ParseNode(node.to_owned())
                    } else {
                        DocumentSelection::default()
                    }
                }
                _ => {
                    params
                }
            };
        }
        params
    }

    fn eq<'a>(&self, params: DocumentSelection<'a>) -> DocumentSelection<'a> {
        let b = self.eq.unwrap();
        return match params {
            DocumentSelection::ParseSelection(d) => {
                let nodes = d.nodes().to_vec();
                if nodes.len() > 0 {
                    let node = nodes.get(b).unwrap();
                    DocumentSelection::ParseNode(node.to_owned())
                } else {
                    DocumentSelection::default()
                }
            }
            _ => {
                params
            }
        }
    }

    fn parent<'a>(&self, params: DocumentSelection<'a>) -> DocumentSelection<'a> {
        if self.parent.unwrap() {
            return match params {
                DocumentSelection::ParseSelection(d) => {
                    let s = d.parent();
                    DocumentSelection::ParseSelection(s)
                }
                DocumentSelection::ParseDocument(d) => {
                    let node = d.root().parent().unwrap();
                    let s = Selection::from(node);
                    DocumentSelection::ParseSelection(s)
                }
                DocumentSelection::ParseNode(d) => {
                    let mut s = Selection::from(d.to_owned());
                    s = s.parent();
                    DocumentSelection::ParseSelection(s)
                }
            };
        }
        params
    }

    fn children<'a>(&self, params: DocumentSelection<'a>) -> DocumentSelection<'a> {
        if self.children.unwrap() {
            return match params {
                DocumentSelection::ParseSelection(d) => {
                    let s = d.children();
                    DocumentSelection::ParseSelection(s)
                }
                DocumentSelection::ParseDocument(d) => {
                    let node = d.root();
                    let mut s = Selection::from(node);
                    s = s.children();
                    DocumentSelection::ParseSelection(s)
                }
                DocumentSelection::ParseNode(d) => {
                    let mut s = Selection::from(d.to_owned());
                    s = s.children();
                    DocumentSelection::ParseSelection(s)
                }
            };
        }
        params
    }

    fn prev_sibling<'a>(&self, params: DocumentSelection<'a>) -> DocumentSelection<'a> {
        if self.prev_sibling.unwrap() {
            return match params {
                DocumentSelection::ParseSelection(d) => {
                    let s = d.prev_sibling();
                    DocumentSelection::ParseSelection(s)
                }
                DocumentSelection::ParseDocument(d) => {
                    let node = d.root();
                    let mut s = Selection::from(node);
                    s = s.prev_sibling();
                    DocumentSelection::ParseSelection(s)
                }
                DocumentSelection::ParseNode(d) => {
                    let mut s = Selection::from(d.to_owned());
                    s = s.prev_sibling();
                    DocumentSelection::ParseSelection(s)
                }
            };
        }
        params
    }

    fn next_sibling<'a>(&self, params: DocumentSelection<'a>) -> DocumentSelection<'a> {
        if self.next_sibling.unwrap() {
            return match params {
                DocumentSelection::ParseSelection(d) => {
                    let s = d.next_sibling();
                    DocumentSelection::ParseSelection(s)
                }
                DocumentSelection::ParseDocument(d) => {
                    let node = d.root();
                    let mut s = Selection::from(node);
                    s = s.next_sibling();
                    DocumentSelection::ParseSelection(s)
                }
                DocumentSelection::ParseNode(d) => {
                    let mut s = Selection::from(d.to_owned());
                    s = s.next_sibling();
                    DocumentSelection::ParseSelection(s)
                }
            };
        }
        params
    }
}
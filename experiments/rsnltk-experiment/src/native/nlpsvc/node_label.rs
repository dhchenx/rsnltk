//! # node_label.rs
//!
//! Representation for data stored in tree nodes (and maybe elsewhere)

use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NodeLabel {
    pub span: Option<(usize, usize)>,
    pub attributes: HashMap<String, String>,
}

impl NodeLabel {
    pub fn new() -> NodeLabel {
        NodeLabel { span: None, attributes: HashMap::new(), }
    }

    pub fn set_span(&mut self, begin: usize, end: usize) -> &mut Self {
        // TODO: Check for end < begin, etc.
        self.span = Some((begin, end));
        self
    }

    pub fn get_span(&self) -> Option<(usize, usize)> {
        self.span
    }

    pub fn set_sym_val(&mut self, attr: &str, val: &str) -> &mut Self {
        self.attributes.insert(attr.to_string(), val.to_string());
        self
    }

    pub fn get_sym_val(&self, attr: &str) -> &str {
        &self.attributes[attr]
    }
}

impl fmt::Display for NodeLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.span {
            None => write!(f, "_ _"),
            Some((b, e)) => write!(f, "{} {} ", 
                                      self.span.unwrap().0, 
                                      self.span.unwrap().1)
        }
    }
}

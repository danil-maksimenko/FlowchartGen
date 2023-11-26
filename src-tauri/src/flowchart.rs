use std::{ops::{Deref, DerefMut}, str::FromStr, collections::HashMap};

use serde::{Serialize, Deserialize};

use crate::ast::Ast;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlowchartStr(String);

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Flowchart<'a> {
    string: FlowchartStr,
    #[serde(borrow)]
    lookup: HashMap<Ast, &'a str>
}

impl Default for FlowchartStr {
    fn default() -> Self {
        Self(String::from_str("start=>start: Start").unwrap())
    }
}
impl Deref for FlowchartStr {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0 
    }
}
impl DerefMut for FlowchartStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

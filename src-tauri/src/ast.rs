use std::ops::{Deref, DerefMut};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
/// Represents the code in the text format
pub struct Code(String);

impl From<String> for Code {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for Code {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl Deref for Code {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Code {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Serialize, Clone, Hash, Deserialize, PartialEq, Eq)]
pub enum Ast {
    Action(Code),
    Output(Code),
    Input(Code),
    Conditional {
        condition: Box<Ast>,
        body: Vec<Ast>,
    },
}

impl Ast {
    // todo: naive as fuck way to do this
    pub fn identify_in_out_or_action<C: Into<Code>>(code: C) -> Self {
        let code = code.into();
        if code.contains("printf") || code.contains("cout") {
            return Self::output(code);
        } else if code.contains("scanf") || code.contains("cin") {
            return Self::input(code)
        }
        Self::action(code)
    }
    #[inline]
    pub fn action<C: Into<Code>>(code: C) -> Self {
        Self::Action(code.into())
    }
    #[inline]
    pub fn output<C: Into<Code>>(code: C) -> Self {
        Self::Output(code.into())
    }
    #[inline]
    pub fn input<C: Into<Code>>(code: C) -> Self {
        Self::Input(code.into())
    }
    #[inline]
    pub fn condition<B: Into<Box<Ast>>, V: Into<Vec<Ast>>>(condition: B, body: V) -> Self {
        Self::Conditional { condition: condition.into(), body: body.into() }
    }
}
use clang::{diagnostic::Diagnostic, diagnostic::Severity};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CodeError {
    text: String,
    line: u32,
    character: u32,
}
impl TryFrom<Diagnostic<'_>> for CodeError {
    type Error = (); // todo: proper error type
    fn try_from(diagnostic: Diagnostic<'_>) -> Result<Self, Self::Error> {
        if matches!(
            diagnostic.get_severity(),
            Severity::Ignored | Severity::Note | Severity::Warning
        ) {
            return Err(());
        }
        let text = diagnostic.get_text();
        let location = diagnostic.get_location().get_file_location();
        let line = location.line;
        let character = location.offset;
        Ok(Self {
            text,
            line,
            character,
        })
    }
}
impl From<Vec<CodeError>> for ParseError {
    fn from(val: Vec<CodeError>) -> Self {
        ParseError::CodeErrors { errors: val }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize)]
pub enum ParseError {
    NoMain,
    CodeErrors { errors: Vec<CodeError> },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NoMain => "NoMain",
            Self::CodeErrors { .. } => "CodeError",
        };
        writeln!(f, "ParseError: {s}")
    }
}
impl std::error::Error for ParseError {}
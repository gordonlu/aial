// diagnostic.rs — Structured compiler diagnostics

use crate::token::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity { Error, Warning, Note }

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self { Severity::Error => "error", Severity::Warning => "warning", Severity::Note => "note" }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Option<&'static str>,
    pub message: String,
    pub span: Option<Span>,
    pub help: Option<String>,
}

#[derive(Debug, Default)]
pub struct Diagnostics {
    pub items: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self { Self { items: Vec::new() } }

    pub fn error(&mut self, code: &'static str, msg: String) -> &mut Self {
        self.items.push(Diagnostic { severity: Severity::Error, code: Some(code), message: msg, span: None, help: None });
        self
    }
    pub fn error_at(&mut self, code: &'static str, span: Span, msg: String) -> &mut Self {
        self.items.push(Diagnostic { severity: Severity::Error, code: Some(code), message: msg, span: Some(span), help: None });
        self
    }
    pub fn warn(&mut self, code: &'static str, msg: String) -> &mut Self {
        self.items.push(Diagnostic { severity: Severity::Warning, code: Some(code), message: msg, span: None, help: None });
        self
    }
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Error)
    }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }

    /// Adapter: convert old-style Vec<String> errors into diagnostics
    pub fn from_strings(code: &'static str, errors: Vec<String>) -> Self {
        let mut diags = Self::new();
        for e in errors { diags.error(code, e); }
        diags
    }

    /// Adapter: collect diagnostics and old-style errors together
    pub fn merge_strings(&mut self, code: &'static str, errors: Vec<String>) {
        for e in errors { self.error(code, e); }
    }

    /// Emit all diagnostics as a formatted string
    pub fn emit(&self) -> String {
        let mut out = String::new();
        for d in &self.items {
            let sev = d.severity.as_str();
            let code_str = d.code.map(|c| format!("[{}]", c)).unwrap_or_default();
            if let Some(span) = d.span {
                out.push_str(&format!("{}: [line {}:{}] {}{}\n",
                    sev, span.line, span.col, code_str, if code_str.is_empty() { "" } else { " " }));
                out.push_str(&format!("  {}\n", d.message));
            } else {
                out.push_str(&format!("{}: {}{} {}\n", sev, code_str, if code_str.is_empty() { "" } else { ":" }, d.message));
            }
            if let Some(ref help) = d.help {
                out.push_str(&format!("  = help: {}\n", help));
            }
        }
        out
    }
}

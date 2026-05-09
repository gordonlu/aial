// philosophy.rs — AIAL diagnostic style engine
// --philosophy tao | legalist | medical
// Error-specific quotations and diagnoses, in English.

use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Default = 0,
    Tao = 1,
    Legalist = 2,
    Medical = 3,
}

static CURRENT: AtomicU8 = AtomicU8::new(0);

pub fn set_from_flag(flag: &str) {
    let mode = match flag {
        "tao" => Mode::Tao,
        "legalist" => Mode::Legalist,
        "medical" => Mode::Medical,
        _ => Mode::Default,
    };
    CURRENT.store(mode as u8, Ordering::Relaxed);
}

pub fn wrap(severity: &str, msg: &str) -> String {
    match CURRENT.load(Ordering::Relaxed) {
        1 => tao_style(severity, msg),
        2 => legalist_style(severity, msg),
        3 => medical_style(severity, msg),
        _ => format!("{}: {}", severity, msg),
    }
}

// ──── Tao (道家): gentle, paradoxical wisdom ────

fn tao_style(severity: &str, msg: &str) -> String {
    let (quote, layout) = match () {
        _ if msg.contains("undefined name") || msg.contains("undefined variable") => (
            "\"The nameless is the beginning of heaven and earth.\" — Laozi",
            "suggestion: perhaps it needs to be named first?",
        ),
        _ if msg.contains("type error") || msg.contains("cannot unify") => (
            "\"The five colors make one blind.\" — Laozi",
            "suggestion: too many shapes trying to fit — simplify the type.",
        ),
        _ if msg.contains("syntax error") || msg.contains("parse error") || msg.contains("expected") => (
            "\"A journey of a thousand miles begins with a single step.\" — Laozi",
            "suggestion: start from a simpler form and build upward.",
        ),
        _ if msg.contains("parameter count") || msg.contains("mismatch") => (
            "\"Clay is shaped into vessels, but the emptiness is what makes them useful.\" — Laozi",
            "suggestion: less is sometimes more — check what you're passing.",
        ),
        _ if msg.contains("not found") || msg.contains("no API key") => (
            "\"He who knows does not speak; he who speaks does not know.\" — Laozi",
            "suggestion: the thing you seek has not been declared yet.",
        ),
        _ => (
            "\"True words are not beautiful; beautiful words are not true.\" — Laozi",
            "suggestion: strip away the excess and the root cause will appear.",
        ),
    };
    format!("{}\n  {}: {}\n  {}", quote, severity, msg, layout)
}

// ──── Legalist (法家): strict, no-compromise law ────

fn legalist_style(severity: &str, msg: &str) -> String {
    let (quote, verdict) = match () {
        _ if msg.contains("undefined name") || msg.contains("undefined variable") => (
            "\"If names are not correct, speech does not follow reason.\" — Han Feizi",
            "ruling: this identifier has no standing. Declare it before use.",
        ),
        _ if msg.contains("type error") || msg.contains("cannot unify") => (
            "\"The law is the measure of all things. Deviation from it is not permitted.\" — Han Feizi",
            "ruling: these types do not conform. Reconcile them immediately.",
        ),
        _ if msg.contains("syntax error") || msg.contains("parse error") => (
            "\"Eliminate the weeds, and the seedlings will thrive by themselves.\" — Han Feizi",
            "ruling: remove the malformed syntax. The correct form must prevail.",
        ),
        _ if msg.contains("capability") || msg.contains("not declared") => (
            "\"The wise ruler makes the law clear; the foolish ruler multiplies prohibitions.\" — Han Feizi",
            "ruling: the capability was not declared. The law cannot be bypassed.",
        ),
        _ if msg.contains("parameter count") || msg.contains("mismatch") => (
            "\"Punishment should fit the crime, reward should match the merit.\" — Han Feizi",
            "ruling: every function expects exact tribute. Count your arguments.",
        ),
        _ if msg.contains("budget") || msg.contains("exhausted") => (
            "\"Order comes from law, not from hope.\" — Han Feizi",
            "ruling: the budget limit is absolute. No further calls are permitted.",
        ),
        _ => (
            "\"The law is no respecter of persons; the measuring-line does not bend.\" — Han Feizi",
            "ruling: this code must be corrected before it may proceed.",
        ),
    };
    format!("{}\n  {}: {}\n  {}", quote, severity, msg, verdict)
}

// ──── Medical (医家): diagnostic, symptom → prescription ────

fn medical_style(severity: &str, msg: &str) -> String {
    let (symptom, rx) = match () {
        _ if msg.contains("undefined name") || msg.contains("undefined variable") => (
            "Symptom: unknown entity at this location.",
            "Prescription: declare the identifier before referencing it.",
        ),
        _ if msg.contains("type error") || msg.contains("cannot unify") => (
            "Symptom: incompatible blood-types in the expression.",
            "Prescription: ensure both sides of the operation share the same type.",
        ),
        _ if msg.contains("syntax error") || msg.contains("parse error") => (
            "Symptom: structural malformation in the source.",
            "Prescription: verify the grammar at this position. A missing token? A stray delimiter?",
        ),
        _ if msg.contains("parameter count") || msg.contains("mismatch") => (
            "Symptom: function receiving wrong number of arguments.",
            "Prescription: compare the call site against the function signature.",
        ),
        _ if msg.contains("capability") || msg.contains("not declared") => (
            "Symptom: insufficient permissions for this operation.",
            "Prescription: add the required provider to [capabilities] in aial.toml.",
        ),
        _ if msg.contains("not found") || msg.contains("no API key") => (
            "Symptom: missing authentication material.",
            "Prescription: run `aial key add` or set AIAL_KEY_<PROVIDER>.",
        ),
        _ if msg.contains("budget") || msg.contains("exhausted") => (
            "Symptom: resource depletion — token budget exhausted.",
            "Prescription: increase token_budget in context::new or reduce max_tokens per ask.",
        ),
        _ => (
            "Symptom: compilation irregularity detected at this site.",
            "Prescription: examine the surrounding context and consult the language reference.",
        ),
    };
    format!("{}: {}\n  {} — {}", severity, msg, symptom, rx)
}

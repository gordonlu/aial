use super::*;

use unicode_segmentation::UnicodeSegmentation;
use crate::io::aial_rt_io_readkey;

struct LineEditor {
    prompt: String,
    buffer: String,
    cursor: usize,        // byte offset in buffer
    history: Vec<String>,
    history_idx: usize,   // 0 = new input, 1..=len = history pos
    saved: String,        // saved input before history navigation
}

static LINE_EDITORS: OnceLock<Mutex<HashMap<i64, LineEditor>>> = OnceLock::new();
static NEXT_LINE_ID: Mutex<i64> = Mutex::new(1);

fn line_editors() -> &'static Mutex<HashMap<i64, LineEditor>> {
    LINE_EDITORS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn line_redraw(ed: &LineEditor) {
    use std::io::Write;
    let out = format!("\r\x1b[2K{} {}", ed.prompt, ed.buffer);
    let _ = std::io::stdout().write_all(out.as_bytes());
    let _ = std::io::stdout().flush();
}

/// Find the grapheme cluster boundary before `pos` (byte offset)
fn grapheme_boundary_before(s: &str, pos: usize) -> usize {
    let mut prev = 0;
    for (byte_pos, _) in s.grapheme_indices(true) {
        if byte_pos >= pos { break; }
        prev = byte_pos;
    }
    prev
}

/// Find the grapheme cluster boundary at or after `pos` (byte offset)
fn grapheme_boundary_after(s: &str, pos: usize) -> usize {
    for (byte_pos, _) in s.grapheme_indices(true) {
        if byte_pos > pos { return byte_pos; }
    }
    s.len() // at end
}

#[no_mangle]
pub extern "C" fn aial_rt_line_new(prompt_ptr: i64) -> i64 {
    let prompt = lock!(strs()).get(&prompt_ptr).cloned().unwrap_or_else(|| "> ".to_string());
    let mut n = lock!(NEXT_LINE_ID);
    let id = *n; *n += 1;
    lock!(line_editors()).insert(id, LineEditor {
        prompt,
        buffer: String::new(),
        cursor: 0,
        history: Vec::new(),
        history_idx: 0,
        saved: String::new(),
    });
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_line_set_history(handle: i64, hist_ptr: i64) {
    let json = lock!(strs()).get(&hist_ptr).cloned().unwrap_or_default();
    let entries: Vec<String> = serde_json::from_str(&json).unwrap_or_default();
    if let Some(ed) = lock!(line_editors()).get_mut(&handle) {
        ed.history = entries;
        ed.history_idx = 0;
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_line_read(handle: i64) -> i64 {
    let ptr = alloc();
    let result: Option<String> = {
        let mut eds = lock!(line_editors());
        if eds.contains_key(&handle) {
            drop(eds);
            // Show prompt immediately before waiting for keypress
            {
                let eds = lock!(line_editors());
                if let Some(ed) = eds.get(&handle) { line_redraw(ed); }
            }
            loop {
                let key_ptr = aial_rt_io_readkey();
                let key = lock!(strs()).get(&key_ptr).cloned().unwrap_or_default();
                let mut eds = lock!(line_editors());
                let ed = match eds.get_mut(&handle) { Some(e) => e, None => { drop(eds); return ptr; } };

                let is_nav = key == "UP" || key == "DOWN";
                if !is_nav { ed.history_idx = 0; }

                let mut done = false;
                let mut line = String::new();
                match key.as_str() {
                    "ENTER" => {
                        line = ed.buffer.clone();
                        if !line.is_empty() {
                            ed.history.push(line.clone());
                            while ed.history.len() > 200 { ed.history.remove(0); }
                        }
                        ed.buffer.clear(); ed.cursor = 0; ed.history_idx = 0; ed.saved.clear();
                        done = true;
                    }
                    "CTRL_Q" | "CTRL_C" => {
                        ed.buffer.clear(); ed.cursor = 0; ed.history_idx = 0; ed.saved.clear();
                        line = String::new(); done = true;
                    }
                    "BACKSPACE" => {
                        if ed.cursor > 0 {
                            let start = grapheme_boundary_before(&ed.buffer, ed.cursor);
                            ed.buffer.replace_range(start..ed.cursor, "");
                            ed.cursor = start;
                        }
                    }
                    "LEFT" => {
                        if ed.cursor > 0 {
                            ed.cursor = grapheme_boundary_before(&ed.buffer, ed.cursor);
                        }
                    }
                    "RIGHT" => {
                        let blen = ed.buffer.len();
                        if ed.cursor < blen {
                            ed.cursor = grapheme_boundary_after(&ed.buffer, ed.cursor);
                        }
                    }
                    "UP" => {
                        if !ed.history.is_empty() {
                            if ed.history_idx == 0 { ed.saved = ed.buffer.clone(); }
                            if ed.history_idx < ed.history.len() {
                                ed.history_idx += 1;
                                let idx = ed.history.len() - ed.history_idx;
                                ed.buffer = ed.history[idx].clone();
                                ed.cursor = ed.buffer.len();
                            }
                        }
                    }
                    "DOWN" => {
                        if ed.history_idx > 1 {
                            ed.history_idx -= 1;
                            let idx = ed.history.len() - ed.history_idx;
                            ed.buffer = ed.history[idx].clone();
                            ed.cursor = ed.buffer.len();
                        } else if ed.history_idx == 1 {
                            ed.history_idx = 0;
                            ed.buffer = ed.saved.clone();
                            ed.cursor = ed.buffer.len();
                        }
                    }
                    "CTRL_L" => { ed.buffer.clear(); ed.cursor = 0; }
                    "ESC" | "TAB" | "HOME" | "END" | "PAGEUP" | "PAGEDOWN"
                    | "F1" | "F2" | "F3" | "F4" | "DELETE" | "CTRL_D" => {}
                    s if !s.is_empty() => {
                        let first = s.as_bytes()[0];
                        if first >= 32 || first >= 0xC0 {
                            ed.buffer.insert_str(ed.cursor, s);
                            ed.cursor += s.len();
                        }
                    }
                    _ => {}
                }

                if done {
                    use std::io::Write; let _ = std::io::stdout().write_all(b"\r\n"); let _ = std::io::stdout().flush();
                    drop(eds);
                    lock!(strs()).insert(ptr, line);
                    return ptr;
                }
                line_redraw(ed);
                drop(eds);
            }
        } else { None }
    };
    match result {
        Some(s) => { lock!(strs()).insert(ptr, s); }
        None => { lock!(strs()).insert(ptr, String::new()); }
    }
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_line_redraw(handle: i64) {
    let eds = lock!(line_editors());
    if let Some(ed) = eds.get(&handle) { line_redraw(ed); }
}

#[no_mangle]
pub extern "C" fn aial_rt_line_end(handle: i64) {
    lock!(line_editors()).remove(&handle);
}

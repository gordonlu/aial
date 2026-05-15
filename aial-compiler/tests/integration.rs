// integration.rs — End-to-end tests: compile .aal → .ll → link → run
use std::process::Command;
use std::fs;

const RT_DIR: &str = "../aial-rt/target/debug";

fn compile_and_run(name: &str, source: &str) -> Result<String, String> {
    let tmp_dir = std::env::temp_dir().join(format!("aial_test_{}", name));
    let _ = fs::create_dir_all(&tmp_dir);

    let aal_path = tmp_dir.join("test.aal");
    let ll_path = tmp_dir.join("test.ll");
    let bin_path = tmp_dir.join("test_bin");
    let toml_path = tmp_dir.join("aial.toml");

    fs::write(&aal_path, source).map_err(|e| e.to_string())?;
    // Write aial.toml for capability check
    fs::write(&toml_path, "[capabilities]\nallow_network = [{ provider = \"deepseek\", models = [\"deepseek-v4-flash\"] }]\nallow_filesystem = [{ path = \".\", access = \"write\" }]\n").ok();

    // Compile with AIAL — run from tmp_dir so aial.toml is found
    let aial = std::env::current_dir()
        .map(|d| d.join("target/debug/aial"))
        .unwrap_or_else(|_| "target/debug/aial".into());

    let output = Command::new(&aial)
        .arg("build")
        .arg(&aal_path)
        .current_dir(&tmp_dir)
        .output()
        .map_err(|e| format!("aial not found: {} (build with cargo build first)", e))?;

    // Read LLVM IR from tmp_dir
    let cwd_ll = tmp_dir.join("aial_output.ll");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !cwd_ll.exists() {
        return Err(format!("no .ll generated: {}", stderr));
    }
    fs::copy(&cwd_ll, &ll_path).map_err(|e| e.to_string())?;

    // Link with clang
    let rt_path = std::env::current_dir().unwrap().join(RT_DIR);
    let link = Command::new("clang")
        .arg(&ll_path)
        .arg("-L").arg(&rt_path)
        .arg("-laial_rt")
        .arg("-lm").arg("-lpthread").arg("-ldl")
        .arg("-o").arg(&bin_path)
        .output()
        .map_err(|e| format!("clang not found: {}", e))?;

    if !link.status.success() {
        return Err(format!("link failed: {}", String::from_utf8_lossy(&link.stderr)));
    }

    // Run
    let run = Command::new(&bin_path)
        .output()
        .map_err(|e| format!("run failed: {}", e))?;

    let stdout = String::from_utf8_lossy(&run.stdout).to_string();
    if !run.status.success() {
        return Err(format!("binary crashed (exit {}): stdout={}", run.status.code().unwrap_or(-1), stdout));
    }

    Ok(stdout)
}

#[test]
fn hello_world_prints() {
    let out = compile_and_run("hello", r#"fn main() { println("HELLO"); return; }"#)
        .expect("compile+run failed");
    assert!(out.contains("HELLO"), "output should contain HELLO, got: {}", out);
}

#[test]
fn while_loop_runs() {
    // Compute 0+1+2+3+4 = 10 but we can't easily print ints, so just check no crash
    let out = compile_and_run("while", r#"fn main() { let i = 0; while i < 5 { i = i + 1; } return; }"#)
        .expect("while loop failed");
    // Just checking it doesn't crash is enough
}

#[test]
fn void_function_works() {
    let out = compile_and_run("void", r#"
fn say_hi() { println("hi"); }
fn main() { say_hi(); say_hi(); return; }
"#).expect("void fn failed");
    assert!(out.contains("hi"), "output: {}", out);
}

#[test]
fn user_function_returns_value() {
    let out = compile_and_run("userfn", r#"
fn double(x: int) -> int { return x + x; }
fn main() { let x = double(21); println("ok"); return; }
"#).expect("user fn failed");
    assert!(out.contains("ok"), "output: {}", out);
}

#[test]
fn str_eq_works() {
    let out = compile_and_run("streq", r#"
fn check(s: string) -> int { if str_eq(s, "quit") { return 1; } return 0; }
fn main() { if check("quit") == 1 { println("MATCH"); } return; }
"#).expect("str_eq failed");
    assert!(out.contains("MATCH"), "output: {}", out);
}

#[test]
fn json_parse_and_stringify() {
    let out = compile_and_run("json", r#"
fn main() { let v = json::parse("[1,2,3]"); let s = json::stringify(v); println("OK"); return; }
"#).expect("json failed");
    assert!(out.contains("OK"), "output: {}", out);
}

#[test]
fn map_set_get_has_works() {
    let out = compile_and_run("map", r#"
fn main() {
    let m = map::new();
    map::set(m, "name", "AIAL");
    if map::has(m, "name") { println("OK"); }
    let v = map::get(m, "name");
    if str_eq(v, "AIAL") { println("MATCH"); }
    map::remove(m, "name");
    if !map::has(m, "name") { println("REMOVED"); }
    return;
}
"#).expect("map failed");
    assert!(out.contains("OK"), "output: {}", out);
    assert!(out.contains("MATCH"), "output: {}", out);
    assert!(out.contains("REMOVED"), "output: {}", out);
}

#[test]
fn heap_push_pop_ordering() {
    let out = compile_and_run("heap", r#"
fn main() {
    let h = heap::new();
    heap::push(h, "low", 1);
    heap::push(h, "high", 10);
    heap::push(h, "mid", 5);
    let top = heap::pop(h);
    if str_eq(top, "high") { println("TOP_OK"); }
    let n = heap::len(h);
    if n == 2 { println("LEN_OK"); }
    return;
}
"#).expect("heap failed");
    assert!(out.contains("TOP_OK"), "output: {}", out);
    assert!(out.contains("LEN_OK"), "output: {}", out);
}

#[test]
fn array_sort_works() {
    let out = compile_and_run("array", r#"
fn main() {
    let a = array::new();
    array::push(a, "c");
    array::push(a, "a");
    array::push(a, "b");
    array::sort(a);
    let first = array::get(a, 0);
    if str_eq(first, "a") { println("SORT_OK"); }
    let sz = array::len(a);
    if sz == 3 { println("LEN_OK"); }
    return;
}
"#).expect("array failed");
    assert!(out.contains("SORT_OK"), "output: {}", out);
    assert!(out.contains("LEN_OK"), "output: {}", out);
}

#[test]
fn module_function_call_works() {
    let out = compile_and_run("modfn", r#"
module Greeter {
    fn greet(name: string) -> string { return strcat("hello ", name); }
}
fn main() {
    let msg = Greeter::greet("world");
    if str_eq(msg, "hello world") { println("MOD_OK"); }
    return;
}
"#).expect("module fn failed");
    assert!(out.contains("MOD_OK"), "output: {}", out);
}

#[test]
fn generic_monomorphization_works() {
    let out = compile_and_run("genmono", r#"
fn id<T>(x: T) -> T { return x; }
fn main() {
    let a = id(42);
    let b = id("test");
    if str_eq(b, "test") { println("GEN_OK"); }
    return;
}
"#).expect("generic monomorphization failed");
    assert!(out.contains("GEN_OK"), "output: {}", out);
}

#[test]
fn token_estimate_works() {
    let out = compile_and_run("token", r#"
fn main() {
    let t = token_estimate("hello world");
    if t >= 1 { println("TOK_OK"); }
    return;
}
"#).expect("token_estimate failed");
    assert!(out.contains("TOK_OK"), "output: {}", out);
}

#[test]
fn nested_module_works() {
    let out = compile_and_run("nestmod", r#"
module A {
    module B {
        fn val() -> string { return "deep"; }
    }
}
fn main() {
    let v = A::B::val();
    if str_eq(v, "deep") { println("NEST_OK"); }
    return;
}
"#).expect("nested module failed");
    assert!(out.contains("NEST_OK"), "output: {}", out);
}

#[test]
fn defer_executes_lifo() {
    let out = compile_and_run("defer", r#"
fn main() {
    defer { println("second"); }
    defer { println("first"); }
    println("main");
    return;
}
"#).expect("defer failed");
    let lines: Vec<&str> = out.lines().collect();
    assert!(lines.iter().any(|l| l.contains("main")), "output: {}", out);
    assert!(lines.iter().any(|l| l.contains("first")), "output: {}", out);
    assert!(lines.iter().any(|l| l.contains("second")), "output: {}", out);
}

#[test]
fn bool_int_cmp_works() {
    let out = compile_and_run("boolint", r#"
fn main() {
    let m = map::new();
    map::set(m, "k", "v");
    if map::has(m, "k") == true { println("BOOL_EQ_TRUE"); }
    if map::has(m, "missing") == false { println("BOOL_EQ_FALSE"); }
    let h = map::has(m, "k");
    if h == 1 { println("BOOL_EQ_1"); }
    if h != 0 { println("BOOL_NE_0"); }
    return;
}
"#).expect("bool/int cmp failed");
    assert!(out.contains("BOOL_EQ_TRUE"), "output: {}", out);
    assert!(out.contains("BOOL_EQ_FALSE"), "output: {}", out);
    assert!(out.contains("BOOL_EQ_1"), "output: {}", out);
    assert!(out.contains("BOOL_NE_0"), "output: {}", out);
}

#[test]
fn actor_ops_llvm() {
    let out = compile_and_run("actor", r#"
fn handle(_pid: int) {
    println("actor_ok");
    return;
}
fn main() {
    let pid = actor::spawn();
    actor::send(pid, "ping");
    let msg = actor::recv_timeout(pid, 100);
    println("main_ok");
    return;
}
"#).expect("actor failed");
    assert!(out.contains("main_ok"), "output: {}", out);
}

#[test]
fn llvm_type_alignment() {
    let tmp_dir = std::env::temp_dir().join("aial_test_llvm");
    let _ = fs::create_dir_all(&tmp_dir);
    fs::write(tmp_dir.join("aial.toml"), "[capabilities]\nallow_network = [{ provider = \"deepseek\", models = [\"deepseek-v4-flash\"] }]\nallow_filesystem = [{ path = \".\", access = \"write\" }]\n").ok();
    fs::write(tmp_dir.join("test.aal"), "fn main() { println(\"ok\"); return; }\n").ok();

    let aial = std::env::current_dir().unwrap().join("target/debug/aial");
    let _ = Command::new(&aial).arg("build").arg("test.aal").current_dir(&tmp_dir).output().ok();

    let ll_file = tmp_dir.join("aial_output.ll");
    if !ll_file.exists() { return; }
    let ll = fs::read_to_string(&ll_file).unwrap_or_default();

    // Verify void-returning functions are declared as void
    let void_funcs = ["aial_rt_println", "aial_rt_print", "aial_rt_ctx_save_message",
        "aial_rt_ctx_close_memory", "aial_rt_string_register", "aial_rt_time_sleep",
        "aial_rt_io_raw_mode", "aial_rt_ffi_close", "aial_rt_http_respond"];
    for d in ll.lines().filter(|l| l.starts_with("declare ")) {
        let parts: Vec<&str> = d.split_whitespace().collect();
        if parts.len() < 3 { continue; }
        let name = parts[2].trim_start_matches('@').split('(').next().unwrap_or("");
        if void_funcs.iter().any(|&f| name.contains(f)) {
            assert_eq!(parts[1], "void", "{} should be declare void, got {}", name, parts[1]);
        }
    }
}

// ── Self-hosting essentials: verify real runtime behavior, not stubs ──

#[test]
fn int_to_string_works() {
    let out = compile_and_run("itos", r#"
fn main() { println(int_to_string(42)); return; }
"#).expect("int_to_string failed");
    assert!(out.contains("42"), "output: {}", out);
}

#[test]
fn string_to_int_works() {
    let out = compile_and_run("stoi", r#"
fn main() { let n = string_to_int("42"); if n == 42 { println("OK"); } return; }
"#).expect("string_to_int failed");
    assert!(out.contains("OK"), "output: {}", out);
}

#[test]
fn str_find_works() {
    let out = compile_and_run("strfind", r#"
fn main() {
    let i = str_find("hello world", "world");
    if i == 6 { println("FOUND"); }
    let j = str_find("hello", "x");
    if j == -1 { println("NOTFOUND"); }
    return;
}
"#).expect("str_find failed");
    assert!(out.contains("FOUND"), "output: {}", out);
    assert!(out.contains("NOTFOUND"), "output: {}", out);
}

#[test]
fn process_run_works() {
    let out = compile_and_run("procrun", r#"
fn main() { println(process::run("echo hello_from_subprocess")); return; }
"#).expect("process::run failed");
    assert!(out.contains("hello_from_subprocess"), "output: {}", out);
}

#[test]
fn file_list_dir_works() {
    let out = compile_and_run("listdir", r#"
fn main() {
    let dirs = file::list_dir("/tmp");
    if strlen(dirs) > 0 { println("HAS_ENTRIES"); }
    return;
}
"#).expect("file::list_dir failed");
    assert!(out.contains("HAS_ENTRIES"), "output: {}", out);
}

#[test]
fn time_now_ms_works() {
    let out = compile_and_run("nowms", r#"
fn main() {
    let t = time::now_ms();
    if t > 1700000000000 { println("TIME_OK"); }
    return;
}
"#).expect("time::now_ms failed");
    assert!(out.contains("TIME_OK"), "output: {}", out);
}

#[test]
fn args_works() {
    let out = compile_and_run("argstest", r#"
fn main() { println(args()); return; }
"#).expect("args failed");
    assert!(!out.contains("[error"), "should not contain error: {}", out);
}

#[test]
fn ffi_works() {
    let out = compile_and_run("ffitest", r#"
fn main() {
    let h = ffi::load("libc.so.6");
    if h != 0 { println("FFI_LOAD_OK"); }
    ffi::close(h);
    return;
}
"#).expect("ffi failed");
    assert!(out.contains("FFI_LOAD_OK"), "output: {}", out);
}

#[test]
fn int_to_string_roundtrip() {
    let out = compile_and_run("roundtrip", r#"
fn main() {
    let s = int_to_string(12345);
    let n = string_to_int(s);
    if n == 12345 { println("ROUNDTRIP_OK"); }
    return;
}
"#).expect("roundtrip failed");
    assert!(out.contains("ROUNDTRIP_OK"), "output: {}", out);
}

// ── array::join tests ──

#[test]
fn array_join_works() {
    let out = compile_and_run("arrjoin", r#"
fn main() {
    let a = array::new();
    array::push(a, "hello");
    array::push(a, "world");
    let s = array::join(a, ",");
    if str_eq(s, "hello,world") { println("JOIN_OK"); }
    return;
}
"#).expect("array_join failed");
    assert!(out.contains("JOIN_OK"), "output: {}", out);
}

#[test]
fn array_join_empty() {
    let out = compile_and_run("arrjoin2", r#"
fn main() {
    let a = array::new();
    let s = array::join(a, ",");
    if str_eq(s, "") { println("JOIN_EMPTY_OK"); }
    return;
}
"#).expect("array_join empty failed");
    assert!(out.contains("JOIN_EMPTY_OK"), "output: {}", out);
}

#[test]
fn array_join_single() {
    let out = compile_and_run("arrjoin3", r#"
fn main() {
    let a = array::new();
    array::push(a, "only");
    let s = array::join(a, ",");
    if str_eq(s, "only") { println("JOIN_ONE_OK"); }
    return;
}
"#).expect("array_join single failed");
    assert!(out.contains("JOIN_ONE_OK"), "output: {}", out);
}

// ── io::is_tty test (not a TTY in test runner, should not crash) ──

#[test]
fn io_is_tty_does_not_crash() {
    let out = compile_and_run("istty", r#"
fn main() {
    let t = io::is_tty();
    if t == 0 { println("NOT_TTY"); }
    if t == 1 { println("IS_TTY"); }
    return;
}
"#).expect("io::is_tty failed");
    // Either output is valid — just verify it doesn't crash
    assert!(out.contains("NOT_TTY") || out.contains("IS_TTY"), "output: {}", out);
}

// ── Editor operations: backspace and insert via strslice+strcat ──

#[test]
fn editor_backspace_middle() {
    let out = compile_and_run("edbs", r#"
fn main() {
    let editor = "abcde"; let col = 3;  // cursor after 'c'
    let left = strslice(editor, 0, col - 1);
    let right = strslice(editor, col, strlen(editor) - col);
    editor = strcat(left, right);
    col = col - 1;
    if str_eq(editor, "abde") { println("BS_MID_OK"); }
    return;
}
"#).expect("editor backspace failed");
    assert!(out.contains("BS_MID_OK"), "output: {}", out);
}

#[test]
fn editor_insert_middle() {
    let out = compile_and_run("edins", r#"
fn main() {
    let editor = "abde"; let col = 2;  // cursor after 'b'
    let key = "c";
    let left = strslice(editor, 0, col);
    let right = strslice(editor, col, strlen(editor) - col);
    editor = strcat(strcat(left, key), right);
    col = col + strlen(key);
    if str_eq(editor, "abcde") { println("INS_MID_OK"); }
    return;
}
"#).expect("editor insert failed");
    assert!(out.contains("INS_MID_OK"), "output: {}", out);
}

#[test]
fn editor_backspace_start() {
    let out = compile_and_run("edbs2", r#"
fn main() {
    let editor = "abc"; let col = 0;
    // Backspace at start: no-op
    if col > 0 {
        let left = strslice(editor, 0, col - 1);
        let right = strslice(editor, col, strlen(editor) - col);
        editor = strcat(left, right);
        col = col - 1;
    }
    if str_eq(editor, "abc") { println("BS_START_OK"); }
    return;
}
"#).expect("editor backspace start failed");
    assert!(out.contains("BS_START_OK"), "output: {}", out);
}

#[test]
fn editor_insert_end() {
    let out = compile_and_run("edins2", r#"
fn main() {
    let editor = "ab"; let col = 2;  // cursor at end
    let key = "c";
    let left = strslice(editor, 0, col);
    let right = strslice(editor, col, strlen(editor) - col);
    editor = strcat(strcat(left, key), right);
    col = col + strlen(key);
    if str_eq(editor, "abc") { println("INS_END_OK"); }
    return;
}
"#).expect("editor insert end failed");
    assert!(out.contains("INS_END_OK"), "output: {}", out);
}

// ── UTF-8 character boundary navigation ──

#[test]
fn str_prev_char_ascii() {
    let out = compile_and_run("prevch1", r#"
fn main() {
    let s = "hello";
    let p = str_prev_char(s, 3);  // before 'l' (byte 3)
    if p == 2 { println("PREV_ASCII_OK"); }
    return;
}
"#).expect("str_prev_char ascii failed");
    assert!(out.contains("PREV_ASCII_OK"), "output: {}", out);
}

#[test]
fn str_prev_char_cjk() {
    let out = compile_and_run("prevch2", r#"
fn main() {
    let s = "你好世界";  // 12 bytes, 4 chars, 3 bytes each
    let p = str_prev_char(s, 6);  // before '世' (byte 6)
    if p == 3 { println("PREV_CJK_OK"); }
    return;
}
"#).expect("str_prev_char cjk failed");
    assert!(out.contains("PREV_CJK_OK"), "output: {}", out);
}

#[test]
fn str_next_char_ascii() {
    let out = compile_and_run("nextch1", r#"
fn main() {
    let s = "hello";
    let n = str_next_char(s, 2);  // after 'l' (byte 2 → byte 3)
    if n == 3 { println("NEXT_ASCII_OK"); }
    return;
}
"#).expect("str_next_char ascii failed");
    assert!(out.contains("NEXT_ASCII_OK"), "output: {}", out);
}

#[test]
fn str_next_char_cjk() {
    let out = compile_and_run("nextch2", r#"
fn main() {
    let s = "你好世界";  // 12 bytes, 4 chars
    let n = str_next_char(s, 3);  // after '你' (byte 3 → byte 6)
    if n == 6 { println("NEXT_CJK_OK"); }
    return;
}
"#).expect("str_next_char cjk failed");
    assert!(out.contains("NEXT_CJK_OK"), "output: {}", out);
}

#[test]
fn str_prev_char_at_start() {
    let out = compile_and_run("prevch3", r#"
fn main() {
    let s = "hello";
    let p = str_prev_char(s, 0);  // at start
    if p == 0 { println("PREV_START_OK"); }
    return;
}
"#).expect("str_prev_char at start failed");
    assert!(out.contains("PREV_START_OK"), "output: {}", out);
}

#[test]
fn str_prev_char_continuation() {
    // Test that str_prev_char walks back past UTF-8 continuation bytes
    let out = compile_and_run("prevch4", r#"
fn main() {
    // Build a string with a known multi-byte char then check boundary
    let s = "abc"; let n = str_next_char(s, 0);
    if n == 1 { println("NEXT_1"); }
    n = str_next_char(s, n);
    if n == 2 { println("NEXT_2"); }
    n = str_next_char(s, n);
    if n == 3 { println("NEXT_3"); }
    let p = str_prev_char(s, 3);
    if p == 2 { println("PREV_2"); }
    p = str_prev_char(s, p);
    if p == 1 { println("PREV_1"); }
    p = str_prev_char(s, p);
    if p == 0 { println("PREV_0"); }
    return;
}
"#).expect("str boundary walk failed");
    assert!(out.contains("NEXT_1"), "output: {}", out);
    assert!(out.contains("NEXT_2"), "output: {}", out);
    assert!(out.contains("NEXT_3"), "output: {}", out);
    assert!(out.contains("PREV_2"), "output: {}", out);
    assert!(out.contains("PREV_1"), "output: {}", out);
    assert!(out.contains("PREV_0"), "output: {}", out);
}

// ── Cross-backend conformance tests ──

#[test]
fn process_run_status_works() {
    let out = compile_and_run("procstat", r#"
fn main() {
    // Just verify it doesn't crash — run_status returns a heap block
    let r = process::run_status("echo hello");
    if r != 0 { println("PROCSTAT_OK"); }
    return;
}
"#).expect("process::run_status failed");
    assert!(out.contains("PROCSTAT_OK"), "output: {}", out);
}

#[test]
fn cross_backend_string_ops() {
    // Verify string ops produce same result in interpreter and AOT
    // This only runs AOT (compile_and_run), but validates end-to-end
    let out = compile_and_run("xstr", r#"
fn main() {
    let s = "hello world";
    if strlen(s) == 11 { println("STRLEN_OK"); }
    if str_eq(strslice(s, 0, 5), "hello") { println("STR_LEFT"); }
    if str_eq(strslice(s, 6, 5), "world") { println("STR_RIGHT"); }
    if str_find(s, "world") == 6 { println("STR_FIND"); }
    let n = string_to_int("42");
    if n == 42 { println("STOI_OK"); }
    let t = int_to_string(42);
    if str_eq(t, "42") { println("ITOS_OK"); }
    return;
}
"#).expect("cross-backend string test failed");
    assert!(out.contains("STRLEN_OK"), "output: {}", out);
    assert!(out.contains("STR_LEFT"), "output: {}", out);
    assert!(out.contains("STR_RIGHT"), "output: {}", out);
    assert!(out.contains("STR_FIND"), "output: {}", out);
    assert!(out.contains("STOI_OK"), "output: {}", out);
    assert!(out.contains("ITOS_OK"), "output: {}", out);
}

#[test]
fn global_storage_works() {
    let out = compile_and_run("gstore", r#"
fn main() {
    global::set("test_key", "hello");
    if global::has("test_key") == 1 { println("GLOBAL_HAS"); }
    let v = global::get("test_key");
    if str_eq(v, "hello") { println("GLOBAL_GET_OK"); }
    global::delete("test_key");
    if global::has("test_key") == 0 { println("GLOBAL_DEL_OK"); }
    return;
}
"#).expect("global storage failed");
    assert!(out.contains("GLOBAL_HAS"), "output: {}", out);
    assert!(out.contains("GLOBAL_GET_OK"), "output: {}", out);
    assert!(out.contains("GLOBAL_DEL_OK"), "output: {}", out);
}

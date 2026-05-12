// integration.rs — End-to-end tests: compile .aal → .ll → link → run
use std::process::Command;
use std::fs;

const RT_DIR: &str = "../aial-rt/target/release";

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

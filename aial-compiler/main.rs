// main.rs — AAL compiler driver
#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process;

mod token;
mod lexer;
mod ast;
mod parser;
mod symbol;
mod types;
mod type_checker;
mod ir;
mod ir_builder;
mod ir_lower;
mod interpreter;
mod capability;
mod key_manager;
mod philosophy;
mod jit_backend;
mod aot_backend;
mod llvm_backend;

use lexer::Lexer;
use parser::Parser;
use symbol::NameResolver;
use ir_builder::IRBuilder;
use ir_lower::lower_module;
use interpreter::interpret;
use jit_backend::jit_run;
use type_checker::TypeChecker;

/// Preprocess `include` directives. Supports nested includes with cycle detection.
/// Lines matching `include "path"` (with optional leading whitespace) are replaced
/// with the referenced file's content. Paths are relative to the including file.
fn preprocess(source: &str, base_path: &PathBuf, visited: &mut HashSet<PathBuf>) -> Result<String, String> {
    let dot = PathBuf::from(".");
    let base_dir = base_path.parent().unwrap_or(dot.as_path());
    let mut result = String::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("include \"") && trimmed.ends_with('"') {
            let path_str = &trimmed[9..trimmed.len()-1]; // strip include "..."
            let include_path = base_dir.join(path_str);
            let canonical = include_path.canonicalize().map_err(|e| format!("include not found: {} ({})", path_str, e))?;
            if !visited.insert(canonical.clone()) {
                return Err(format!("circular include: {}", path_str));
            }
            let included_source = fs::read_to_string(&include_path)
                .map_err(|e| format!("include read error: {} ({})", path_str, e))?;
            let expanded = preprocess(&included_source, &include_path, visited)?;
            result.push_str(&expanded);
            result.push('\n');
            visited.remove(&canonical);
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    Ok(result)
}

fn compile_and_run(source: &str, backend: &str) -> Result<(), Vec<String>> {
    let lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();
    if !lex_errors.is_empty() { return Err(lex_errors); }

    let parser = Parser::new(tokens);
    let program = parser.parse().map_err(|errors| {
        errors.into_iter().map(|e| format!("syntax error: {}", e)).collect::<Vec<_>>()
    })?;

    let symbols = NameResolver::new().resolve(&program).map_err(|errors| {
        errors.into_iter().map(|e| format!("name error: {}", e)).collect::<Vec<_>>()
    })?;

    let config = capability::load_config().map_err(|e| vec![e])?;
    let (specializations, call_specializations) = TypeChecker::with_config(symbols, config).check(&program).map_err(|errors| {
        errors.into_iter().map(|e| format!("type error: {}", e)).collect::<Vec<_>>()
    })?;

    let mut ir_builder = IRBuilder::new();
    ir_builder.set_specializations(specializations);
    ir_builder.set_call_specializations(call_specializations);
    let ir_module = ir_builder.build(&program, &types::TypeEnv::new());

    let (lowered_module, reg) = lower_module(&ir_module);
    match backend {
        "jit" => jit_run(&lowered_module, &reg).map_err(|e| vec![format!("jit error: {}", e)])?,
        "aot" => {
            aot_backend::aot_compile(&lowered_module, &reg, "aial_output.o")
                .map_err(|e| vec![format!("aot error: {}", e)])?;
            println!("AOT compilation complete -> aial_output.o");
            return Ok(());
        }
        "llvm" => {
            llvm_backend::llvm_compile(&lowered_module, &reg, "aial_output.ll")
                .map_err(|e| vec![format!("llvm error: {}", e)])?;
            println!("LLVM IR generated -> aial_output.ll");
            return Ok(());
        }
        _ => interpret(&lowered_module).map_err(|e| vec![format!("runtime error: {}", e)])?,
    }
    Ok(())
}

fn cli() -> &'static str {
    let arg0 = std::env::args().next().unwrap_or_default();
    if arg0.contains("cargo") || arg0.contains("rustc") { "cargo run --" } else { "aial" }
}

fn die(msg: &str) -> ! {
    eprintln!("{}", philosophy::wrap("error", msg));
    process::exit(1);
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let c = cli();
    let mut backend = "interpret";

    // Parse global flags
    if let Some(pos) = args.iter().position(|a| a == "--philosophy") {
        if let Some(mode) = args.get(pos + 1) {
            philosophy::set_from_flag(mode);
            args.remove(pos); args.remove(pos);
        }
    }
    if let Some(pos) = args.iter().position(|a| a == "--backend") {
        if let Some(val) = args.get(pos + 1) {
            backend = Box::leak(val.clone().into_boxed_str());
            args.remove(pos); args.remove(pos);
        }
    }

    match args.get(1).map(|s| s.as_str()) {
        Some("key") => match args.get(2).map(|s| s.as_str()) {
            Some("add") => {
                let provider = get_flag(&args, "--provider").unwrap_or_else(|| {
                    // 自动选择已有的 provider
                    let p = key_manager::first_provider();
                    Box::leak(p.into_boxed_str())
                });
                let key = get_flag(&args, "--key").unwrap_or_else(|| {
                    die(&format!("usage: {} key add --provider <name> --key <key>", c));
                });
                match key_manager::set_key(provider, key) {
                    Ok(_) => println!("saved API key for provider `{}`", provider),
                    Err(e) => eprintln!("error: {}", e),
                }
            }
            Some("list") => match key_manager::list_keys() {
                Ok(keys) => {
                    if keys.is_empty() {
                        println!("No API keys stored.");
                        println!("Run `{} key add --provider openai --key YOUR_KEY` to add one.", c);
                    } else {
                        println!("Registered API keys:");
                        for (p, m) in &keys { println!("  {}: {}", p, m); }
                    }
                }
                Err(e) => eprintln!("error: {}", e),
            }
            Some("remove") => {
                let provider = get_flag(&args, "--provider").unwrap_or("openai");
                match key_manager::remove_key(provider) {
                    Ok(_) => println!("removed API key for provider `{}`", provider),
                    Err(e) => eprintln!("error: {}", e),
                }
            }
            _ => die(&format!("usage: {} key <add|list|remove> [--provider <name>] [--key <key>]", c)),
        },
        Some("build") => {
            let path = args.get(2).unwrap_or_else(|| die(&format!("usage: {} build <file.aal>", c)));
            let raw = fs::read_to_string(PathBuf::from(path)).expect("failed to read file");
            let source = preprocess(&raw, &PathBuf::from(path).canonicalize().unwrap_or_else(|_| PathBuf::from(path)), &mut HashSet::new());
            let source = source.unwrap_or_else(|e| die(&e));
            if let Err(errors) = compile_and_run(&source, "llvm") {
                for e in errors { eprintln!("{}", philosophy::wrap("error", &e)); }
                process::exit(1);
            }
            // Validate LLVM IR with clang -c before suggesting link
            let status = process::Command::new("clang")
                .args(["-c", "-o", "/dev/null", "aial_output.ll"])
                .stderr(process::Stdio::piped())
                .status();
            if let Ok(s) = status {
                if !s.success() {
                    let output = process::Command::new("clang")
                        .args(["-c", "-o", "/dev/null", "aial_output.ll"])
                        .output().ok();
                    if let Some(o) = output {
                        let err = String::from_utf8_lossy(&o.stderr);
                        eprintln!("{}", err.lines().take(5).collect::<Vec<_>>().join("\n"));
                    }
                    eprintln!("{}", philosophy::wrap("error", "LLVM IR validation failed — type mismatch or malformed IR (caught at compile time, before linking)"));
                    process::exit(1);
                }
            }
            // Link: clang aial_output.ll -L aial-rt/target/release -laial_rt -lm -lpthread -ldl -o aial_bin
            eprintln!("To link: clang aial_output.ll -L ../aial-rt/target/release -laial_rt -lm -lpthread -ldl -rdynamic -o aial_bin");
        }
        Some("run") => {
            let path = args.get(2).unwrap_or_else(|| die(&format!("usage: {} run <file.aal>", c)));
            let raw = fs::read_to_string(PathBuf::from(path)).expect("failed to read file");
            let source = preprocess(&raw, &PathBuf::from(path).canonicalize().unwrap_or_else(|_| PathBuf::from(path)), &mut HashSet::new());
            let source = source.unwrap_or_else(|e| die(&e));
            if let Err(errors) = compile_and_run(&source, backend) {
                for e in errors { eprintln!("{}", philosophy::wrap("error", &e)); }
                process::exit(1);
            }
        }
        Some("check") => {
            let path = args.get(2).unwrap_or_else(|| die(&format!("usage: {} check <file.aal>", c)));
            let raw = fs::read_to_string(PathBuf::from(path)).expect("failed to read file");
            let source = preprocess(&raw, &PathBuf::from(path).canonicalize().unwrap_or_else(|_| PathBuf::from(path)), &mut HashSet::new());
            let source = source.unwrap_or_else(|e| die(&e));
            match Parser::new(Lexer::new(&source).tokenize().0).parse() {
                Ok(_) => println!("syntax OK"),
                Err(e) => { for e in e { eprintln!("{}", e); } process::exit(1); }
            }
        }
        _ => {
            let source = if let Some(path) = args.get(1) {
                let raw = fs::read_to_string(PathBuf::from(path)).expect("failed to read file");
                preprocess(&raw, &PathBuf::from(path).canonicalize().unwrap_or_else(|_| PathBuf::from(path)), &mut HashSet::new())
                    .unwrap_or_else(|e| die(&e))
            } else {
                r#"
fn main() {
    let ctx = context::new(system_prompt = "you are a helpful assistant", token_budget = 4096);
    let response = ask(model = 0, context = ctx, prompt = "Hello! What time is it?", temperature = 0.7, max_tokens = 1024);
    println(response.text);
    return;
}
"#.to_string()
            };
            if let Err(errors) = compile_and_run(&source, backend) {
                for e in errors { eprintln!("{}", philosophy::wrap("error", &e)); }
                process::exit(1);
            }
        }
    }
}

fn get_flag<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).map(|s| s.as_str())
}

// main.rs — AAL compiler driver
#![allow(dead_code)]

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
    TypeChecker::with_config(symbols, config).check(&program).map_err(|errors| {
        errors.into_iter().map(|e| format!("type error: {}", e)).collect::<Vec<_>>()
    })?;

    let ir_builder = IRBuilder::new();
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
            let source = fs::read_to_string(PathBuf::from(path)).expect("failed to read file");
            if let Err(errors) = compile_and_run(&source, "llvm") {
                for e in errors { eprintln!("{}", philosophy::wrap("error", &e)); }
                process::exit(1);
            }
            // Link: clang aial_output.ll -L aial-rt/target/release -laial_rt -lm -lpthread -ldl -o aial_bin
            eprintln!("To link: clang aial_output.ll -L ../aial-rt/target/release -laial_rt -lm -lpthread -ldl -o aial_bin");
        }
        Some("run") => {
            let path = args.get(2).unwrap_or_else(|| die(&format!("usage: {} run <file.aal>", c)));
            let source = fs::read_to_string(PathBuf::from(path)).expect("failed to read file");
            if let Err(errors) = compile_and_run(&source, backend) {
                for e in errors { eprintln!("{}", philosophy::wrap("error", &e)); }
                process::exit(1);
            }
        }
        Some("check") => {
            let path = args.get(2).unwrap_or_else(|| die(&format!("usage: {} check <file.aal>", c)));
            let source = fs::read_to_string(PathBuf::from(path)).expect("failed to read file");
            match Parser::new(Lexer::new(&source).tokenize().0).parse() {
                Ok(_) => println!("syntax OK"),
                Err(e) => { for e in e { eprintln!("{}", e); } process::exit(1); }
            }
        }
        _ => {
            let source = if let Some(path) = args.get(1) {
                fs::read_to_string(PathBuf::from(path)).expect("failed to read file")
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

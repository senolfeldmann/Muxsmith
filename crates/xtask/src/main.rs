use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("gen-capability") if args.len() == 4 => {
            let schema = fs::read_to_string(&args[2]).unwrap_or_else(|e| {
                eprintln!("cannot read {}: {e}", args[2]);
                process::exit(1);
            });
            let code = xtask::codegen::generate(&schema).unwrap_or_else(|e| {
                eprintln!("generation failed: {e}");
                process::exit(1);
            });
            fs::write(&args[3], code).unwrap_or_else(|e| {
                eprintln!("cannot write {}: {e}", args[3]);
                process::exit(1);
            });
            eprintln!("wrote {}", args[3]);
        }
        _ => {
            eprintln!("usage: cargo run -p xtask -- gen-capability <schema.json> <out.rs>");
            process::exit(2);
        }
    }
}

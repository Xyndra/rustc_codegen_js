//! CLI: split a complete Tailwind build into critical/lazy CSS for one or
//! more prerendered pages.
//!
//! Usage: tailwind-split <full.css> <out-dir> [page.html ...]

use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage: tailwind-split <full.css> <out-dir> [page.html ...]");
        return ExitCode::from(2);
    }
    let full_path = PathBuf::from(&args[0]);
    let out_dir = PathBuf::from(&args[1]);
    let full = match std::fs::read_to_string(&full_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("read {}: {e}", full_path.display());
            return ExitCode::FAILURE;
        }
    };
    let mut pages = Vec::new();
    for page in &args[2..] {
        match std::fs::read_to_string(page) {
            Ok(html) => pages.push(html),
            Err(e) => {
                eprintln!("read {page}: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
    let refs: Vec<&str> = pages.iter().map(String::as_str).collect();
    let split = match fw_tailwind_build::split_for_pages(&full, &refs) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("create {}: {e}", out_dir.display());
        return ExitCode::FAILURE;
    }
    let critical_path = out_dir.join("critical.css");
    let lazy_path = out_dir.join("lazy.css");
    if let Err(e) = std::fs::write(&critical_path, &split.critical) {
        eprintln!("write {}: {e}", critical_path.display());
        return ExitCode::FAILURE;
    }
    if let Err(e) = std::fs::write(&lazy_path, &split.lazy) {
        eprintln!("write {}: {e}", lazy_path.display());
        return ExitCode::FAILURE;
    }
    println!(
        "critical {} bytes ({}), lazy {} bytes ({})",
        split.critical.len(),
        critical_path.display(),
        split.lazy.len(),
        lazy_path.display()
    );
    ExitCode::SUCCESS
}

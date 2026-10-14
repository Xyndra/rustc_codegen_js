use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

fn root() -> PathBuf {
    let mut p = env::current_dir().unwrap();
    loop {
        if p.join("rust-toolchain.toml").exists() && p.join("Cargo.toml").exists() {
            return p;
        }
        if !p.pop() {
            break;
        }
    }
    panic!("repository root not found");
}

fn prepare(root: &PathBuf) -> Result<(), String> {
    let cfg = root.join(".cargo").join("config.toml");
    let body = format!(
        r#"cargo-features = ["profile-rustflags"]

[build]
[unstable]
codegen-backend = true

[profile.dev.package.dgs]
codegen-backend = "{root}/target/release/libcodegen_js.so"
rustflags = ["-Zalways-encode-mir=false"]

[profile.dev.package.dom]
codegen-backend = "{root}/target/release/libcodegen_js.so"
rustflags = ["-Zalways-encode-mir=false"]

[profile.dev.package.dgs-browser]
codegen-backend = "{root}/target/release/libcodegen_js.so"
rustflags = ["-Zalways-encode-mir=false"]

[profile.dev.package.typst-browser]
codegen-backend = "{root}/target/release/libcodegen_js.so"
rustflags = ["-Zalways-encode-mir=false"]

[profile.dev.package.dgs-playground]
codegen-backend = "{root}/target/release/libcodegen_js.so"
rustflags = ["-Zalways-encode-mir=false"]

[profile.dev.package.color-test]
codegen-backend = "{root}/target/release/libcodegen_js.so"
rustflags = ["-Zalways-encode-mir=false"]

[profile.dev.package.dgs-quicktest]
codegen-backend = "{root}/target/release/libcodegen_js.so"
rustflags = ["-Zalways-encode-mir=false"]

[profile.dev.package.dgs-site]
codegen-backend = "{root}/target/release/libcodegen_js.so"
rustflags = ["-Zalways-encode-mir=false"]
"#,
        root = root.display()
    );
    fs::create_dir_all(root.join(".cargo")).unwrap();
    fs::write(cfg, body).map_err(|e| e.to_string())?;
    println!("prepared .cargo/config.toml");
    Ok(())
}

fn run(cmd: &mut Command, root: &PathBuf) -> Result<(), String> {
    println!("running: {:?}", cmd);
    cmd.current_dir(root);
    let status: ExitStatus = cmd.status().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command failed: {:?}", status))
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let root = root();
    let cmd = args.first().cloned().unwrap_or_else(|| "help".into());
    match cmd.as_str() {
        "prepare" => {
            if let Err(e) = prepare(&root) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        "backend" => {
            let mut c = Command::new("cargo");
            c.args(["build", "--release"]);
            if let Err(e) = run(&mut c, &root) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        "all" | "site" => {
            // backend
            let mut c = Command::new("cargo");
            c.args(["build", "--release"]);
            if let Err(e) = run(&mut c, &root) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
            if let Err(e) = prepare(&root) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
            let mut c = Command::new("cargo");
            c.args([
                "build",
                "--manifest-path",
                "ports/Cargo.toml",
                "-p",
                "dgs",
                "-p",
                "dgs-playground",
                "-p",
                "dgs-site",
            ]);
            if let Err(e) = run(&mut c, &root) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        "help" | "-h" | "--help" => {
            println!("cargo-codegen-js: prepare | backend | site | all");
        }
        other => {
            eprintln!("unknown command: {}", other);
            std::process::exit(1);
        }
    }
}

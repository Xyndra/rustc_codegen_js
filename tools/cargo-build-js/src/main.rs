use std::env;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    let mut p = env::current_dir().unwrap();
    loop {
        if p.join("rust-toolchain.toml").exists() && p.join("Cargo.toml").exists() {
            return p;
        }
        if !p.pop() { break; }
    }
    env::current_dir().unwrap()
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("build-js") {
        args.remove(0);
    }
    let help = args.iter().any(|a| a == "--help" || a == "-h");
    if help {
        println!("cargo build-js: wrapper around cargo with rustc_codegen_js enabled");
        return;
    }

    let root = root();
    let backend_status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&root)
        .status()
        .expect("build backend");
    if !backend_status.success() {
        std::process::exit(backend_status.code().unwrap_or(1));
    }

    let backend = format!("{}/target/release/libcodegen_js.so", root.display());
    let packages = [
        "dgs",
        "dom",
        "dgs-browser",
        "typst-browser",
        "dgs-playground",
        "color-test",
        "dgs-quicktest",
    ];

    let mut cargo_args = if args.is_empty() {
        vec!["build".to_string()]
    } else {
        args.clone()
    };
    if let Some(first) = cargo_args.first().cloned() {
        if first.starts_with('-') {
            cargo_args.insert(0, "build".to_string());
        }
    }

    let mut cmd = Command::new("cargo");
    cmd.arg("--config").arg("unstable.codegen-backend=true");
    cmd.arg("--config").arg("cargo-features=[\"profile-rustflags\"]");
    for pkg in &packages {
        cmd.arg("--config")
            .arg(format!("profile.dev.package.{pkg}.codegen-backend=\"{backend}\""));
        cmd.arg("--config")
            .arg(format!(
                "profile.dev.package.{pkg}.rustflags=[\"-Zalways-encode-mir=false\"]"
            ));
    }
    cmd.args(&cargo_args);
    cmd.current_dir(&root);
    let status = cmd.status().expect("run cargo");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

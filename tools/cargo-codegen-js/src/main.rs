use std::env;
use std::fs;
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
    panic!("repository root not found");
}

fn prepare(root: &PathBuf) {
    let cfg = root.join(".cargo").join("config.toml");
    let backend = format!("{}/target/release/libcodegen_js.so", root.display());
    let body = format!(
        "cargo-features = [\"profile-rustflags\"]\n\n[build]\n[unstable]\ncodegen-backend = true\n\n[profile.dev.package.dgs]\ncodegen-backend = \"{b}\"\nrustflags = [\"-Zalways-encode-mir=false\"]\n\n[profile.dev.package.dom]\ncodegen-backend = \"{b}\"\nrustflags = [\"-Zalways-encode-mir=false\"]\n\n[profile.dev.package.dgs-browser]\ncodegen-backend = \"{b}\"\nrustflags = [\"-Zalways-encode-mir=false\"]\n\n[profile.dev.package.typst-browser]\ncodegen-backend = \"{b}\"\nrustflags = [\"-Zalways-encode-mir=false\"]\n\n[profile.dev.package.dgs-playground]\ncodegen-backend = \"{b}\"\nrustflags = [\"-Zalways-encode-mir=false\"]\n\n[profile.dev.package.color-test]\ncodegen-backend = \"{b}\"\nrustflags = [\"-Zalways-encode-mir=false\"]\n\n[profile.dev.package.dgs-quicktest]\ncodegen-backend = \"{b}\"\nrustflags = [\"-Zalways-encode-mir=false\"]\n\n[profile.dev.package.dgs-site]\ncodegen-backend = \"{b}\"\nrustflags = [\"-Zalways-encode-mir=false\"]\n",
        b = backend
    );
    fs::create_dir_all(root.join(".cargo")).unwrap();
    fs::write(cfg, body).unwrap();
    println!("prepared generated .cargo/config.toml");
}

fn exec(cmd: &mut Command, root: &PathBuf) {
    println!("running: {:?}", cmd);
    cmd.current_dir(root);
    let status = cmd.status().expect("failed to spawn command");
    if !status.success() {
        eprintln!("command failed: {}", status);
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let root = root();
    match args.first().map(String::as_str) {
        Some("backend") => {
            let mut c = Command::new("cargo"); c.args(["build", "--release"]); exec(&mut c, &root);
        }
        Some("all") => {
            let mut c = Command::new("cargo"); c.args(["build", "--release"]); exec(&mut c, &root);
            prepare(&root);
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
            exec(&mut c, &root);
        }
        Some("prepare") => prepare(&root),
        Some("clean") => {
            let _ = fs::remove_file(root.join(".cargo").join("config.toml"));
        }
        Some("help") | Some("-h") | Some("--help") | None => {
            println!("usage: cargo-codegen-js [prepare|backend|all|clean]");
        }
        Some(other) => {
            // passthrough to cargo with generated config if exists
            let mut c = Command::new("cargo");
            c.args(args);
            exec(&mut c, &root);
        }
    }
}

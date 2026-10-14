use std::env;
use std::path::PathBuf;
use std::process::Command;

fn target_root() -> PathBuf {
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    for anc in out.ancestors() {
        if anc.join("CACHEDIR.TAG").exists() {
            return anc.to_path_buf();
        }
    }
    out.join("../../..").canonicalize().unwrap_or(out)
}

fn backend_path() -> PathBuf {
    let root = target_root();
    let name = if cfg!(target_os = "windows") {
        "codegen_js.dll"
    } else if cfg!(target_os = "macos") {
        "libcodegen_js.dylib"
    } else {
        "libcodegen_js.so"
    };
    root.join("release").join(name)
}

fn main() {
    let src = backend_path();
    if !src.exists() {
        let status = Command::new("cargo")
            .args(["build", "--release", "-p", "rustc_codegen_js"])
            .status()
            .expect("build backend");
        if !status.success() {
            panic!("could not build backend");
        }
    }
    let src = backend_path();
    let data = std::fs::read(&src).expect("read backend");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("backend");
    std::fs::write(&out, data).expect("write backend bytes");
    println!("cargo:rerun-if-changed={}", src.display());
}

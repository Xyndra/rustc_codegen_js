use std::process::Command;

fn main() {
    // The `dylib` link needs libLLVM-*-rust-*.so, which lives in $SYSROOT/lib.
    // rustc does not pass that -L automatically for out-of-tree backends,
    // so add it here (portable: resolved from the active toolchain at build time).
    if let Ok(sysroot) = std::env::var("SYSROOT").or_else(|_| {
        Command::new("rustc")
            .arg("--print")
            .arg("sysroot")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .map_err(|_| ())
    }) {
        println!("cargo:rustc-link-search=native={sysroot}/lib");
        println!("cargo:rerun-if-env-changed=SYSROOT");
    }
}

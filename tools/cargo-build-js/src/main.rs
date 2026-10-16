use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

static BACKEND: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/backend"));

fn allowed_targets(manifest: Option<&str>) -> Vec<String> {
    let mut cmd = Command::new("cargo");
    cmd.args(["metadata", "--format-version=1", "--no-deps"]);
    if let Some(m) = manifest {
        cmd.args(["--manifest-path", m]);
    }
    let Ok(out) = cmd.output() else {
        return Vec::new();
    };
    if !out.status.success() {
        return Vec::new();
    }
    let Ok(v): Result<serde_json::Value, _> = serde_json::from_slice(&out.stdout) else {
        return Vec::new();
    };
    let packages = match v.get("packages").and_then(|x| x.as_array()) {
        Some(x) => x,
        None => return Vec::new(),
    };
    // Find root member ids.
    let members: Vec<String> = v
        .get("workspace_members")
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    // If no workspace members, use root package metadata.
    let mut member_ids = members.clone();
    if member_ids.is_empty() {
        if let Some(rp) = v.get("root_package").and_then(|x| x.as_str()) {
            member_ids.push(rp.to_string());
        }
    }
    let mut result = Vec::new();
    for pkg in packages {
        let id = pkg.get("id").and_then(|x| x.as_str()).unwrap_or("");
        if !member_ids.iter().any(|m| m == id) {
            continue;
        }
        if let Some(name) = pkg.get("name").and_then(|x| x.as_str()) {
            result.push(name.to_string());
        }
        if let Some(deps) = pkg.get("dependencies").and_then(|x| x.as_array()) {
            for dep in deps {
                if let Some(name) = dep.get("name").and_then(|x| x.as_str()) {
                    let kind = dep.get("kind").and_then(|x| x.as_str()).unwrap_or("normal");
                    if kind == "normal" || kind == "null" {
                        result.push(name.to_string());
                    }
                }
            }
        }
    }
    result.sort();
    result.dedup();
    result
}

fn extracted_backend_path() -> PathBuf {
    let ext = if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };
    let dir = env::temp_dir().join("rustc_codegen_js");
    let _ = fs::create_dir_all(&dir);
    // FNV-1a over the embedded bytes: any backend change re-extracts. The
    // old bare length check missed same-size rebuilds and pinned stale
    // codegen across reinstalls.
    let mut h: u64 = 0xcbf29ce484222325;
    for b in BACKEND {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let path = dir.join(format!("rustc_codegen_js_backend_{h:016x}.{ext}"));
    if !path.exists() {
        let _ = fs::write(&path, BACKEND);
        // Drop superseded extractions so the temp dir does not grow one
        // multi-MB backend per toolchain build.
        if let Ok(rd) = fs::read_dir(&dir) {
            for ent in rd.flatten() {
                let p = ent.path();
                if p != path
                    && p.file_name()
                        .and_then(|n| n.to_str())
                        .map_or(false, |n| {
                            n.starts_with("rustc_codegen_js_backend_")
                        })
                {
                    let _ = fs::remove_file(&p);
                }
            }
        }
    }
    path
}

fn main() {
    // Wrapper mode
    let raw: Vec<String> = env::args().collect();
    let rustc_mode = raw
        .get(1)
        .map(|s| {
            let p = std::path::Path::new(s);
            p.file_name().and_then(|n| n.to_str()) == Some("rustc") || s == "rustc"
        })
        .unwrap_or(false);
    if rustc_mode {
        let args = &raw[2..];
        let mut cmd = Command::new("rustc");
        let mut skip = false;
        let mut crate_name = env::var("CARGO_CRATE_NAME").ok();
        // Crate types on this rustc command line. The JS backend only
        // supports rlib output, so anything else (bin, proc-macro, cdylib,
        // ...) must always compile natively.
        let mut crate_types: Vec<String> = Vec::new();
        let mut i = 0usize;
        while i < args.len() {
            match args[i].as_str() {
                "--crate-name" => {
                    if i + 1 < args.len() {
                        crate_name = Some(args[i + 1].clone());
                    }
                    i += 2;
                    continue;
                }
                "--crate-type" => {
                    if i + 1 < args.len() {
                        crate_types.push(args[i + 1].clone());
                    }
                    i += 2;
                    continue;
                }
                s if s.starts_with("--crate-type=") => {
                    crate_types.push(s.trim_start_matches("--crate-type=").to_string());
                }
                s if s.starts_with("--crate-name=") => {
                    crate_name = Some(s.trim_start_matches("--crate-name=").to_string());
                }
                _ => {}
            }
            i += 1;
        }
        if crate_types
            .iter()
            .any(|t| t != "lib" && t != "rlib")
        {
            skip = true;
        }
        if crate_name
            .as_deref()
            .map_or(false, |n| n.starts_with("build_script_"))
        {
            skip = true;
        }
        let joined = args.join(" ");
        if joined.contains("build_script_")
            || joined.contains("build-script")
            || joined.contains("build_script_build")
        {
            skip = true;
        }
        // cargo names build scripts in command metadata through env.
        if env::var("CARGO_CRATE_NAME").map_or(false, |n| n.starts_with("build_script_")) {
            skip = true;
        }
        if let Ok(allowed) = env::var("RUSTC_CODEGEN_JS_TARGETS") {
            if !allowed.trim().is_empty() {
                let allowed: Vec<_> = allowed.split(',').map(str::trim).collect();
                let cn = crate_name.as_deref().unwrap_or("");
                // Package names use dashes (`dgs-playground`) while
                // `CARGO_CRATE_NAME` uses underscores (`dgs_playground`):
                // normalize both sides or dashed crates never match.
                let cn_norm = cn.replace('-', "_");
                if !allowed
                    .iter()
                    .any(|x| x.replace('-', "_") == cn_norm)
                {
                    skip = true;
                }
            }
        }
        if !skip {
            eprintln!(
                "CG {} {:?}",
                crate_name.as_deref().unwrap_or("?"),
                args.iter().find(|a| a.starts_with("--crate-name"))
            );
        }
        if !skip {
            let backend = extracted_backend_path();
            cmd.arg(format!("-Zcodegen-backend={}", backend.display()));
            cmd.arg("-Zalways-encode-mir=false");
            cmd.arg("-Awarnings");
        }
        cmd.args(args);
        cmd.env_remove("RUSTC_WRAPPER");
        cmd.env_remove("RUSTC_CODEGEN_JS_BACKEND");
        let status = cmd.status().expect("run rustc");
        std::process::exit(status.code().unwrap_or(1));
    }

    // Cargo subcommand
    let mut args: Vec<String> = raw.into_iter().skip(1).collect();
    if args.first().map(String::as_str) == Some("build-js") {
        args.remove(0);
    }
    let invoked_cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let help = args.iter().any(|a| a == "--help" || a == "-h");
    if help {
        println!("cargo build-js: build with rustc_codegen_js, no config required");
        return;
    }
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
    let has_manifest = cargo_args.iter().any(|a| a == "--manifest-path")
        || cargo_args.iter().any(|a| a.starts_with("--manifest-path="));
    if !has_manifest {
        let candidate = invoked_cwd.join("Cargo.toml");
        if candidate.exists() {
            let insert_at = if cargo_args.first().map(String::as_str) == Some("build") {
                1
            } else {
                0
            };
            cargo_args.insert(insert_at, "--manifest-path".to_string());
            cargo_args.insert(insert_at + 1, candidate.display().to_string());
        }
    }
    let manifest_for_meta = cargo_args
        .windows(2)
        .find(|w| w[0] == "--manifest-path")
        .map(|w| w[1].clone())
        .or_else(|| {
            cargo_args
                .iter()
                .find_map(|a| a.strip_prefix("--manifest-path=").map(str::to_string))
        });
    let targets = allowed_targets(manifest_for_meta.as_deref());
    let current_exe = env::current_exe().expect("current exe");
    let status = Command::new("cargo")
        .args(&cargo_args)
        .env("RUSTC_WRAPPER", &current_exe)
        .env("RUSTC_CODEGEN_JS_TARGETS", targets.join(","))
        .current_dir(&invoked_cwd)
        .status()
        .expect("run cargo");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

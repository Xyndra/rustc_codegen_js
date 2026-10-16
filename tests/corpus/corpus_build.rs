// Shared build script for the differential corpus packages. (Inner doc
// comments are illegal inside an `include!`, so this header uses `//`.)
//
// Every corpus is a small cargo package (e.g. `tests/corpus/ints/`) whose
// `build.rs` is one line:
//
//     include!("../corpus_build.rs");
//
// The script does, for the package it runs in:
//
// 1. compiles `src/lib.rs` with the JS backend
//    (`-Zcodegen-backend=.../libcodegen_js.so`) and copies the emitted
//    `<name>.mjs` + `<name>.d.ts` to `<target-dir>/<name>/`;
// 2. compiles the same source natively with stock rustc (or the
//    `native.rs` sidecar, when the corpus needs `#[js::import]`) and runs
//    the resulting binary (`<target-dir>/<name>/native`);
// 3. runs the `.mjs` under node (`run.js` sidecar, or the default driver
//    that prints `run()`) and diffs the output;
// 4. fails the build on any difference.
//
// So a plain `cargo build` in a corpus directory (or
// `cargo build --manifest-path tests/corpus/Cargo.toml` for the whole
// suite) both verifies the backend and leaves inspectable JS/TS behind.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

/// Shared object emitted by the backend crate's `[lib]` target.
const BACKEND_SO: &str = "libcodegen_js.so";
/// Node driver used when the corpus has no `run.js` sidecar.
const DEFAULT_DRIVER: &str = "const m = await import(process.argv[2]);\nconsole.log(m.run());\n";

/// Per-step timing when JS_BUILD_TIMING=1 is set.
///
/// Each `tick(name)` closes the previous step and prints its EXACT elapsed
/// wall time; `finish()` closes the last step and prints the total. A step
/// that hangs is identified by its opening tick with no closing line.
struct T {
    enabled: bool,
    start: Instant,
    open: Option<(&'static str, Instant)>,
}
impl T {
    fn new() -> Self {
        Self {
            enabled: std::env::var("JS_BUILD_TIMING").is_ok(),
            start: Instant::now(),
            open: None,
        }
    }
    fn tick(&mut self, name: &'static str) {
        if !self.enabled {
            return;
        }
        let now = Instant::now();
        if let Some((prev, t)) = self.open.replace((name, now)) {
            println!(
                "cargo::warning=timing: step '{prev}' took {:.3}s (total {:.3}s) -> next: {name}",
                now.duration_since(t).as_secs_f64(),
                now.duration_since(self.start).as_secs_f64()
            );
        } else {
            println!(
                "cargo::warning=timing: step '{name}' started at {:.3}s",
                now.duration_since(self.start).as_secs_f64()
            );
        }
    }
    fn finish(&mut self) {
        if !self.enabled {
            return;
        }
        let now = Instant::now();
        if let Some((prev, t)) = self.open.take() {
            println!(
                "cargo::warning=timing: step '{prev}' took {:.3}s (total {:.3}s) -> done",
                now.duration_since(t).as_secs_f64(),
                now.duration_since(self.start).as_secs_f64()
            );
        }
    }
}

fn main() {
    let name = env_str("CARGO_PKG_NAME");
    let manifest = PathBuf::from(env_str("CARGO_MANIFEST_DIR"));
    let out = PathBuf::from(env_str("OUT_DIR"));
    let src = manifest.join("src/lib.rs");
    let native_src = exists(&manifest.join("native.rs")).unwrap_or_else(|| src.clone());

    println!("cargo::rerun-if-changed=src/lib.rs");
    println!("cargo::rerun-if-changed=native.rs");
    println!("cargo::rerun-if-changed=run.js");

    let backend = find_backend(&manifest);
    let art = target_dir(&out).join(&name);
    recreate(&art);

    let mut t = T::new();
    // ---- 1. JS backend ----
    let js_dir = art.join("js");
    recreate(&js_dir);
    // Corpora using the support crates (`support/js_support`, `support/dom`)
    // declare path dependencies in their Cargo.toml, but this harness drives
    // bare `rustc`, so build the rlibs (stock rustc; only metadata is read)
    // and forward them as explicit `--extern`s. `-L` covers transitive
    // metadata (dom's types name js_support).
    let uses_support = fs::read_to_string(manifest.join("Cargo.toml"))
        .map_or(false, |t| t.contains("support/"));
    let sdir = art.join("support");
    if uses_support {
        build_support_rlibs(&manifest, &art);
        // Corpora on the support crates also need the `dom` *bundle* at
        // node time (real functions like `document()` live there; only
        // stubs peer). It lands at `<art>/support/dom.mjs`, which `run.js`
        // addresses relative to `argv[2]` (`../support/dom.mjs`).
        let _ = fs::create_dir_all(&sdir);
        run(
            Command::new("rustc")
                .arg("--crate-name")
                .arg("dom")
                .args(["--crate-type", "rlib", "--edition", "2021"])
                .arg(find_backend_root(&manifest).join("support/dom/src/lib.rs"))
                .arg("--out-dir")
                .arg(&sdir)
                .arg("--extern")
                .arg(format!("js_support={}", sdir.join("libjs_support.rlib").display()))
                .arg("-L")
                .arg(&sdir)
                .arg(format!("-Zcodegen-backend={}", backend.display())),
            Some(&art.join("support_js.err")),
        )
        .unwrap_or_else(|| panic!("{name}: the JS backend could not compile dom"));
    }
    // Forward the support rlibs as explicit `--extern`s (built above).
    let mut extra: Vec<String> = Vec::new();
    if uses_support {
        for krate in ["js_support", "dom"] {
            extra.push("--extern".to_string());
            extra.push(format!("{krate}={}", sdir.join(format!("lib{krate}.rlib")).display()));
        }
        extra.push("-L".to_string());
        extra.push(sdir.display().to_string());
    }
    t.tick("rustc (JS backend)");
    run(
        Command::new("rustc")
            .arg("--crate-name")
            .arg(&name)
            .args(["--crate-type", "rlib", "--edition", "2021"])
            .arg(&src)
            .arg("--out-dir")
            .arg(&js_dir)
            .args(&extra)
            .arg(format!("-Zcodegen-backend={}", backend.display())),
        Some(&art.join("js.err")),
    )
    .unwrap_or_else(|| panic!("{name}: the JS backend could not compile the corpus"));
    let mjs = js_dir.join(format!("{name}.mjs"));
    if !mjs.exists() {
        panic!("{name}: the backend produced no {}", mjs.display());
    }
    for ext in ["mjs", "d.ts"] {
        let produced = js_dir.join(format!("{name}.{ext}"));
        if produced.exists() {
            let _ = fs::copy(&produced, art.join(format!("{name}.{ext}")));
        }
    }

    // ---- 2. native oracle ----
    let main_rs = out.join(format!("{name}_native_main.rs"));
    let mut text = fs::read_to_string(&native_src).expect("read native source");
    text.push_str("\nfn main() { println!(\"{}\", run()); }\n");
    fs::write(&main_rs, text).expect("write native main");
    let native = art.join("native");
    t.tick("rustc (native)");
    run(
        Command::new("rustc")
            .arg("--edition")
            .arg("2021")
            .arg(&main_rs)
            .arg("-o")
            .arg(&native)
            .arg("-Coverflow-checks=off"),
        Some(&art.join("native.err")),
    )
    .unwrap_or_else(|| panic!("{name}: the native oracle did not compile (corpus bug?)"));
    // Stdout and stderr are captured separately: only stdout is diffed.
    // (Stderr carries panic notices with native paths/line numbers that have
    // no JS counterpart -- e.g. `catch_unwind` tests panic by design.)
    let native_out = art.join("native.out");
    let native_err = art.join("native.stderr");
    {
        let status = Command::new(&native)
            .stdout(fs::File::create(&native_out).expect("native.out"))
            .stderr(fs::File::create(&native_err).expect("native.stderr"))
            .status();
        if !matches!(status, Ok(s) if s.success()) {
            tail(&native_err, 10);
            panic!("{name}: the native oracle failed at run time");
        }
    }

    // ---- 3. run the JS ----
    let driver =
        exists(&manifest.join("run.js")).unwrap_or_else(|| out.join(format!("{name}_driver.js")));
    if !manifest.join("run.js").exists() {
        fs::write(&driver, DEFAULT_DRIVER).expect("write driver");
    }
    let js_out = art.join("js.out");
    let js_run_err = art.join("js_run.err");
    t.tick("node (JS run)");
    let status = Command::new("node")
        .arg(&driver)
        .arg(&mjs)
        .stdout(fs::File::create(&js_out).expect("js.out"))
        .stderr(fs::File::create(&js_run_err).expect("js_run.err"))
        .status();
    if !matches!(status, Ok(s) if s.success()) {
        tail(&js_run_err, 10);
        panic!("{name}: node could not run {}", mjs.display());
    }

    // ---- 4. diff ----
    t.finish();
    let js = fs::read(&js_out).unwrap_or_default();
    let nat = fs::read(&native_out).unwrap_or_default();
    if js == nat {
        println!(
            "cargo::warning={name}: OK  js: {}  ts: {}",
            art.join(format!("{name}.mjs")).display(),
            art.join(format!("{name}.d.ts")).display()
        );
    } else {
        print_diff(
            &String::from_utf8_lossy(&nat),
            &String::from_utf8_lossy(&js),
        );
        panic!(
            "{name}: MISMATCH\n  native: {}\n  js:     {}",
            native_out.display(),
            js_out.display()
        );
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn env_str(k: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| panic!("cargo did not set {k}"))
}

/// The path if it exists.
fn exists(p: &Path) -> Option<PathBuf> {
    p.exists().then(|| p.to_path_buf())
}

/// Empty and recreate a directory.
fn recreate(p: &Path) {
    let _ = fs::remove_dir_all(p);
    fs::create_dir_all(p).unwrap_or_else(|e| panic!("cannot create {}: {e}", p.display()));
}

/// The cargo target directory (the one holding `CACHEDIR.TAG`).
fn target_dir(out: &Path) -> PathBuf {
    for anc in out.ancestors() {
        if anc.join("CACHEDIR.TAG").exists() {
            return anc.to_path_buf();
        }
    }
    // <target>/<profile>/build/<pkg>-<hash>/out
    out.ancestors()
        .nth(4)
        .expect("cannot derive the target dir from OUT_DIR")
        .to_path_buf()
}

/// Absolute path of the built backend shared object.
fn find_backend(manifest: &Path) -> PathBuf {
    if let Ok(env) = std::env::var("BACKEND") {
        if let Some(p) = exists(Path::new(&env)) {
            return p;
        }
    }
    // Walk up to the backend crate (the package named `rustc_codegen_js`)
    // and look for its shared object.
    for anc in manifest.ancestors() {
        let Ok(text) = fs::read_to_string(anc.join("Cargo.toml")) else {
            continue;
        };
        if !text.contains("rustc_codegen_js") {
            continue;
        }
        for profile in ["debug", "release"] {
            if let Some(p) = exists(&anc.join("target").join(profile).join(BACKEND_SO)) {
                return p;
            }
        }
        panic!(
            "backend {BACKEND_SO} is not built in {}; run `cargo build` there first",
            anc.display()
        );
    }
    panic!(
        "cannot locate the backend crate from {}",
        manifest.display()
    );
}

/// Build the support crates (`js_support`, then `dom`) to rlibs with stock
/// rustc and return their directory. Located by walking up to the backend
/// crate (same routing as `find_backend`); rebuilt only when missing.
fn build_support_rlibs(manifest: &Path, art: &Path) -> PathBuf {
    let root = find_backend_root(manifest);
    let dir = art.join("support");
    let _ = fs::create_dir_all(&dir);
    for krate in ["js_support", "dom"] {
        let rlib = dir.join(format!("lib{krate}.rlib"));
        if rlib.exists() {
            continue;
        }
        let mut cmd = Command::new("rustc");
        cmd.arg("--crate-name")
            .arg(krate)
            .args(["--crate-type", "rlib", "--edition", "2021"])
            .arg(root.join(format!("support/{krate}/src/lib.rs")))
            .arg("--out-dir")
            .arg(&dir);
        if krate == "dom" {
            // `dom` names `js_support` types: resolve transitively.
            cmd.arg("--extern").arg(format!(
                "js_support={}",
                dir.join("libjs_support.rlib").display()
            ));
            cmd.arg("-L").arg(&dir);
        }
        run(&mut cmd, Some(&art.join("support.err")))
            .unwrap_or_else(|| panic!("cannot build the {krate} crate"));
    }
    dir
}

/// The backend crate root (whose Cargo.toml mentions `rustc_codegen_js`).
fn find_backend_root(manifest: &Path) -> PathBuf {
    for anc in manifest.ancestors() {
        let Ok(text) = fs::read_to_string(anc.join("Cargo.toml")) else {
            continue;
        };
        if text.contains("rustc_codegen_js") {
            return anc.to_path_buf();
        }
    }
    panic!(
        "cannot locate the backend crate from {}",
        manifest.display()
    );
}

/// Run a command, streaming its stderr into `stderr_file`. Returns `None`
/// (and prints the tail of that file) on failure.
fn run(cmd: &mut Command, stderr_file: Option<&Path>) -> Option<()> {
    if let Some(f) = stderr_file {
        cmd.stderr(fs::File::create(f).expect("stderr file"));
    }
    let ok = matches!(cmd.status(), Ok(s) if s.success());
    if ok {
        return Some(());
    }
    if let Some(f) = stderr_file {
        tail(f, 12);
    }
    None
}

/// Print the last `n` lines of a file.
fn tail(path: &Path, n: usize) {
    if let Ok(s) = fs::read_to_string(path) {
        for line in s
            .lines()
            .rev()
            .take(n)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            eprintln!("  {line}");
        }
    }
}

/// First differing lines, native first.
fn print_diff(native: &str, js: &str) {
    let nl = native.lines().collect::<Vec<_>>();
    let jl = js.lines().collect::<Vec<_>>();
    let mut shown = 0usize;
    for i in 0..nl.len().max(jl.len()) {
        let n = nl.get(i).copied().unwrap_or("<missing>");
        let j = jl.get(i).copied().unwrap_or("<missing>");
        if n == j {
            continue;
        }
        println!("  line {}\n- native: {n}\n- js:     {j}", i + 1);
        shown += 1;
        if shown >= 6 {
            println!("  ...");
            break;
        }
    }
}

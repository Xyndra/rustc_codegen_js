// Assembles the dgs playground site into the cargo target dir.
//
// The page itself is the static `dgs-playground/index.html` shell: markup plus
// a two-line loader that runs the Rust `init()` on load. The crate's imports
// bind true JS functions directly, and the Rust code arms every button
// itself, so the shell carries no shims and no wiring. This script fills in
// only the two bundle slots, verbatim, with the backend bundles cargo just
// built (dependencies are compiled before this script runs, so both bundles
// are fresh):
//
// - `/*BUNDLE_DGS*/` <- `dgs.mjs` (must load first: cross-crate calls resolve
//   through `globalThis.__js_imports` at runtime, in dependency order);
// - `/*BUNDLE_PLAYGROUND*/` <- `dgs_playground.mjs`.
//
// Everything lands under `<target>/<profile>/dgs-playground/` (mirroring the
// `tests/corpus/target/<name>/` convention): `index.html` plus copies of both
// bundles and their `.d.ts` files for inspection. No JavaScript is authored
// here; the script fails loudly if a bundle is missing or a slot is absent.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const PLAYGROUND_PKG: &str = "dgs-playground";
const DGS_PKG: &str = "dgs";

fn main() {
    let manifest = PathBuf::from(env_str("CARGO_MANIFEST_DIR"));
    let profile = env_str("PROFILE");

    println!("cargo::rerun-if-changed=../dgs-playground/index.html");
    println!("cargo::rerun-if-changed=../dgs-playground/src/lib.rs");
    println!("cargo::rerun-if-changed=../dgs-playground/src/eval.rs");
    println!("cargo::rerun-if-changed=build.rs");
    // The backend .so is an untracked input to every bundle: cargo cannot see
    // it change, so watch it explicitly (ports always codegen with the
    // release backend). Without this, a backend rebuild leaves a stale site.
    println!("cargo::rerun-if-changed=../../target/release/libcodegen_js.so");

    let target = workspace_target(&manifest);
    let build_dir = target.join(&profile).join("build");

    let dgs_bundle = newest_bundle(&build_dir, DGS_PKG, "dgs.mjs");
    let pg_bundle = newest_bundle(&build_dir, PLAYGROUND_PKG, "dgs_playground.mjs");
    let dgs_text = fs::read_to_string(&dgs_bundle).expect("read dgs.mjs");
    let pg_text = fs::read_to_string(&pg_bundle).expect("read dgs_playground.mjs");
    for (name, text) in [("dgs.mjs", &dgs_text), ("dgs_playground.mjs", &pg_text)] {
        if text.trim().is_empty() {
            panic!("dgs-site: {name} is empty");
        }
        if text.to_lowercase().contains("</script") {
            panic!("dgs-site: {name} contains </script and cannot be inlined");
        }
    }
    // The page drives the crate's main entry by its wrapper name, and the
    // crate's imports are true JS calls (no host shims for the page to give).
    for wrapper in ["export function _js_init(", "export function _js_run("] {
        if !pg_text.contains(wrapper) {
            panic!("dgs-site: dgs_playground.mjs has no `{wrapper}` wrapper for the page to call");
        }
    }
    for call in [
        "getElementById(\"stage\").replaceChildren(",
        "getElementById(\"stage\").insertAdjacentHTML(",
        "getElementById(\"status\").replaceChildren(",
        "getElementById(\"s0\").addEventListener(",
    ] {
        if !pg_text.contains(call) {
            panic!("dgs-site: dgs_playground.mjs never calls `{call}`");
        }
    }

    let template_path = manifest.join("../dgs-playground/index.html");
    let mut page = fs::read_to_string(&template_path).expect("read index.html template");
    for slot in ["/*BUNDLE_DGS*/", "/*BUNDLE_PLAYGROUND*/"] {
        if !page.contains(slot) {
            panic!("dgs-site: template is missing placeholder {slot}");
        }
    }
    page = page
        .replacen("/*BUNDLE_DGS*/", &dgs_text, 1)
        .replacen("/*BUNDLE_PLAYGROUND*/", &pg_text, 1);

    let site = target.join(&profile).join("dgs-playground");
    fs::create_dir_all(&site).expect("create site dir");
    fs::write(site.join("index.html"), &page).expect("write index.html");
    fs::write(site.join("dgs.mjs"), &dgs_text).expect("write dgs.mjs");
    fs::write(site.join("dgs_playground.mjs"), &pg_text).expect("write dgs_playground.mjs");
    for (src, dst) in [
        (dgs_bundle.with_extension("d.ts"), "dgs.d.ts"),
        (pg_bundle.with_extension("d.ts"), "dgs_playground.d.ts"),
    ] {
        if src.exists() {
            let _ = fs::copy(&src, site.join(dst));
        }
    }
    println!(
        "cargo::warning=dgs-site: wrote {}",
        site.join("index.html").display()
    );
}

/// Cargo build-script dirs are `<pkg>` or `<pkg>-<hexhash>`; a plain prefix
/// match would confuse `dgs` with `dgs-browser`.
fn pkg_dir_matches(dir: &str, pkg: &str) -> bool {
    if dir == pkg {
        return true;
    }
    match dir.strip_prefix(&format!("{pkg}-")) {
        Some(rest) => !rest.is_empty() && rest.chars().all(|c| c.is_ascii_hexdigit()),
        None => false,
    }
}

/// Newest `<file>` under `<build_dir>/<pkg>/<unit>/out/`. The unit hash is
/// never referenced by name; stale readers fail loudly. Both the flat
/// (`<pkg>/out/`) and nested (`<pkg>/<hash>/out/`) layouts are accepted.
fn newest_bundle(build_dir: &Path, pkg: &str, file: &str) -> PathBuf {
    let mut best: Option<(SystemTime, PathBuf)> = None;
    let mut consider_out = |out_dir: &Path| {
        let candidate = out_dir.join(file);
        let Ok(meta) = fs::metadata(&candidate) else {
            return;
        };
        let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        if best.as_ref().is_none_or(|(t, _)| mtime >= *t) {
            best = Some((mtime, candidate));
        }
    };
    let entries = fs::read_dir(build_dir)
        .unwrap_or_else(|_| panic!("dgs-site: no build dir at {}", build_dir.display()));
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !pkg_dir_matches(&name, pkg) {
            continue;
        }
        let pkg_dir = entry.path();
        consider_out(&pkg_dir.join("out"));
        for unit in fs::read_dir(&pkg_dir).into_iter().flatten().flatten() {
            consider_out(&unit.path().join("out"));
        }
    }
    best.map(|(_, p)| p).unwrap_or_else(|| {
        panic!(
            "dgs-site: no {file} for package {pkg} under {} (build it first)",
            build_dir.display()
        )
    })
}

/// The cargo target directory (the one holding `CACHEDIR.TAG`), derived from
/// this script's `OUT_DIR`, which always lives under it.
fn workspace_target(manifest: &Path) -> PathBuf {
    let out = PathBuf::from(env_str("OUT_DIR"));
    for anc in out.ancestors() {
        if anc.join("CACHEDIR.TAG").exists() {
            return anc.to_path_buf();
        }
    }
    // Fallback: `<manifest>/../target` (the default layout).
    let fallback = manifest.join("../target");
    if fallback.join("CACHEDIR.TAG").exists() {
        return fallback;
    }
    panic!(
        "dgs-site: cannot derive the target dir from {}",
        manifest.display()
    );
}

fn env_str(k: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| panic!("cargo did not set {k}"))
}

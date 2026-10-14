// Assembles the dgs playground site into the cargo target dir.
//
// The page itself is the static `dgs-playground/index.html` shell: markup plus
// a two-line loader that runs the Rust `init()` on load. The crate's imports
// bind true JS functions directly, and the Rust code arms every button
// itself, so the shell carries no shims and no wiring. This script fills in
// only the bundle slot, verbatim, with the backend bundles:
//
// - `/*BUNDLE*/` <- minified `dgs.mjs` + `dgs_playground.mjs` combined.
//
// `dgs` and `dgs-playground` are normal `[dependencies]` of this crate, so
// cargo builds them (and their bundles) before this crate and a single
// `cargo build-js -p dgs-site` works from a clean target dir. This script
// reads the bundles from the target dir and fails loudly when they are
// missing. They are deliberately NOT build-dependencies: those would link
// natively into this script, but under `cargo build-js` their rlibs are
// JavaScript stubs, so the native build-script link would fail.
//
// Everything lands under `<target>/<profile>/dgs-playground/` (mirroring the
// `tests/corpus/target/<name>/` convention): `index.html` plus copies of both
// bundles and their `.d.ts` files for inspection. No JavaScript is authored
// here; the script fails loudly if a bundle is missing or a slot is absent.

use oxc_allocator::Allocator;
use oxc_codegen::{Codegen, CodegenOptions};
use oxc_minifier::{
    CompressOptions, MangleOptions, MangleOptionsKeepNames as MK, Minifier, MinifierOptions,
};
use oxc_parser::Parser;
use oxc_span::SourceType;
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
    let dgs_raw = fs::read_to_string(&dgs_bundle).expect("read dgs.mjs");
    let pg_raw = fs::read_to_string(&pg_bundle).expect("read dgs_playground.mjs");
    let (runtime_raw, mut dgs_raw2) = split_runtime(&dgs_raw);
    let (_, mut pg_raw2) = split_runtime(&pg_raw);
    // drop dgs exports section (remove duplicate wrappers)
    if let Some(idx) = dgs_raw2.rfind("// ---- exports ----") {
        dgs_raw2 = dgs_raw2[..idx].trim_end().to_string();
    }
    // pg: drop global registration assignments, keep wrappers
    pg_raw2 = regex::Regex::new(r"[^\n]*globalThis\.__js_imports[^;]*;\n?")
        .unwrap()
        .replace_all(&pg_raw2, "")
        .to_string();
    let mut combined = String::new();
    combined.push_str(&runtime_raw);
    combined.push('\n');
    combined.push_str(&dgs_raw2);
    combined.push('\n');
    combined.push_str(&pg_raw2);
    // one shared import table
    combined = combined.replace("const __js_imports = globalThis.__js_imports ?? {};", "");
    combined = format!(
        "const __js_imports = globalThis.__js_imports ?? {{}};\n{}",
        combined
    );
    // inline cross-crate calls
    let re = regex::Regex::new(r"__js_imports\.([A-Za-z0-9_$]+)\(").unwrap();
    combined = re.replace_all(&combined, "$1(").to_string();
    // remove remaining global registry assignments
    let re2 = regex::Regex::new(r"[^\n]*globalThis\.__js_imports[^;]*;\n?").unwrap();
    combined = re2.replace_all(&combined, "").to_string();
    // normalize exports
    combined = combined.replace("export function ", "function ");
    let re3 = regex::Regex::new(r"(?m)^\s*export\s*\{[^}]*\};?\s*$").unwrap();
    combined = re3.replace_all(&combined, "").to_string();
    combined = combined.replace("export ", "");
    // ensure init call site at bottom
    combined = format!("{}\n_js_init();", combined);
    let combined = minify_js(&combined, false, true);
    for (name, text) in [("bundle.mjs", &combined)] {
        if text.trim().is_empty() {
            panic!("dgs-site: {name} is empty");
        }
        if text.to_lowercase().contains("</script") {
            panic!("dgs-site: {name} contains </script and cannot be inlined");
        }
    }
    // The page drives the crate's main entry by its wrapper name, and the
    // crate's imports are true JS calls (no host shims for the page to give).
    for wrapper in ["function _js_init(", "function _js_run("] {
        if !combined.contains(wrapper) {
            panic!("dgs-site: dgs_playground.mjs has no `{wrapper}` wrapper for the page to call");
        }
    }
    for call in [
        "replaceChildren(",
        "insertAdjacentHTML(",
        "addEventListener(",
        "stage",
        "status",
        "s0",
    ] {
        if !combined.contains(call) {
            panic!("dgs-site: dgs_playground.mjs never calls `{call}`");
        }
    }

    let template_path = manifest.join("../dgs-playground/index.html");
    let mut page = fs::read_to_string(&template_path).expect("read index.html template");

    for slot in ["/*BUNDLE*/"] {
        if !page.contains(slot) {
            panic!("dgs-site: template is missing placeholder {slot}");
        }
    }
    page = page.replacen("/*BUNDLE*/", &combined, 1);

    let site = target.join(&profile).join("dgs-playground");
    fs::create_dir_all(&site).expect("create site dir");
    fs::write(site.join("index.html"), &page).expect("write index.html");
    fs::write(site.join("bundle.mjs"), &combined).expect("write bundle.mjs");
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
fn split_runtime(text: &str) -> (String, String) {
    let marker = "// rustc_codegen_js runtime preamble (inlined into every bundle).";
    let start = text
        .find(marker)
        .expect("dgs-site: no runtime marker in bundle");
    let line_start = text[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let end_marker = "\n// ---- functions ----\n";
    let end = text
        .find(end_marker)
        .expect("dgs-site: no functions marker in bundle");
    let runtime = text[line_start..end].to_string();
    let rest = format!("{}{}", &text[..line_start], &text[end..]);
    (runtime, rest)
}

fn minify_js(text: &str, script: bool, top_level: bool) -> String {
    let allocator = Allocator::default();
    let source_type = if script {
        SourceType::script()
    } else {
        SourceType::mjs()
    };
    let ret = Parser::new(&allocator, text, source_type).parse();
    if !ret.diagnostics.is_empty() {
        panic!("dgs-site: minify parse errors: {:?}", ret.diagnostics);
    }
    let mut program = ret.program;
    let opts = MinifierOptions {
        mangle: Some(MangleOptions {
            top_level: Some(top_level),
            keep_names: MK::all_false(),
            reserved: ["_js_init", "_js_run", "_js_run_from_source"]
                .into_iter()
                .map(|s| s.into())
                .collect(),
            ..Default::default()
        }),
        mangle_properties: None,
        compress: Some(CompressOptions::safest()),
    };
    let mr = Minifier::new(opts).minify(&allocator, &mut program);
    Codegen::new()
        .with_options(CodegenOptions::minify())
        .with_scoping(mr.scoping)
        .build(&program)
        .code
}

use regex::Regex;
use site_builder::{process_site, split_runtime};
use std::path::PathBuf;

fn combine(bundles: Vec<String>) -> String {
    let (runtime_raw, mut util_raw) = split_runtime(&bundles[0]);
    let (_, mut wsys_raw) = split_runtime(&bundles[1]);
    if let Some(idx) = util_raw.rfind("// ---- exports ----") {
        util_raw = util_raw[..idx].trim_end().to_string();
    }
    let mut combined = String::new();
    combined.push_str(&runtime_raw);
    combined.push('\n');
    combined.push_str(&util_raw);
    combined.push('\n');
    combined.push_str(&wsys_raw);
    combined = combined.replace("const __js_imports = globalThis.__js_imports ?? {};", "");
    combined = format!("const __js_imports = globalThis.__js_imports ?? {{}};\n{}", combined);
    let re = Regex::new(r"__js_imports\.([A-Za-z0-9_$]+)\(").unwrap();
    combined = re.replace_all(&combined, "$1(").to_string();
    let re2 = Regex::new(r"[^\n]*globalThis\.__js_imports[^;]*;\n?").unwrap();
    combined = re2.replace_all(&combined, "").to_string();
    combined = combined.replace("export function ", "function ");
    let re3 = Regex::new(r"(?m)^\s*export\s*\{[^}]*\};?\s*$").unwrap();
    combined = re3.replace_all(&combined, "").to_string();
    combined = combined.replace("export ", "");
    format!("{}\n_js_init();", combined)
}

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    process_site(
        manifest,
        "wsys-site",
        "../index.html",
        "/*BUNDLE*/",
        &["../index.html", "../src/lib.rs", "../build.rs"],
        true,
        &[("js_util", "js_util.mjs"), ("wsys", "wsys.mjs")],
        true,
        &["_js_init"],
        combine,
    );
}

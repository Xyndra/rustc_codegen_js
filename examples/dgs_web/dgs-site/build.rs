use regex::Regex;
use site_builder::{process_site, split_runtime};
use std::path::PathBuf;

fn combine(bundles: Vec<String>) -> String {
    let (runtime_raw, mut dgs_raw2) = split_runtime(&bundles[0]);
    let (_, mut dom_raw2) = split_runtime(&bundles[1]);
    let (_, mut pg_raw2) = split_runtime(&bundles[2]);
    if let Some(idx) = dgs_raw2.rfind("// ---- exports ----") {
        dgs_raw2 = dgs_raw2[..idx].trim_end().to_string();
    }
    if let Some(idx) = dom_raw2.rfind("// ---- exports ----") {
        dom_raw2 = dom_raw2[..idx].trim_end().to_string();
    }
    pg_raw2 = Regex::new(r"[^\n]*globalThis\.__js_imports[^;]*;\n?")
        .unwrap()
        .replace_all(&pg_raw2, "")
        .to_string();
    let mut combined = String::new();
    combined.push_str(&runtime_raw);
    combined.push('\n');
    combined.push_str(&dgs_raw2);
    combined.push('\n');
    combined.push_str(&dom_raw2);
    combined.push('\n');
    combined.push_str(&pg_raw2);
    combined = combined.replace("const __js_imports = globalThis.__js_imports ?? {};", "");
    combined = format!(
        "const __js_imports = globalThis.__js_imports ?? {{}};\n{}",
        combined
    );
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
        "dgs-playground",
        "../dgs-playground/index.html",
        "/*BUNDLE*/",
        &[
            "../dgs-playground/index.html",
            "../dgs-playground/src/lib.rs",
            "../dgs-playground/src/eval.rs",
            "../dgs-playground/build.rs",
        ],
        true,
        &[("dgs", "dgs.mjs"), ("dom", "dom.mjs"), ("dgs-playground", "dgs_playground.mjs")],
        true,
        &[
            "_js_init",
            "replaceChildren(",
            "insertAdjacentHTML(",
            "addEventListener(",
            "stage",
            "status",
            "s0",
        ],
        combine,
    );
}

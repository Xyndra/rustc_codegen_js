use site_builder::process_site;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    process_site(
        manifest,
        "wsys-site",
        "../index.html",
        "/*BUNDLE*/",
        &["../index.html", "../src/lib.rs", "../build.rs"],
        true,
        &[("wsys", "wsys.mjs")],
        true,
        &["_js_init"],
        |bundles| {
            let raw = bundles.into_iter().next().unwrap_or_default();
            format!("{}\n_js_init();", raw)
        },
    );
}

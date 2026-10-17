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

pub fn env_str(k: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| panic!("site-builder: cargo did not set {k}"))
}

pub fn workspace_target(manifest: &Path) -> PathBuf {
    let out = PathBuf::from(env_str("OUT_DIR"));
    for anc in out.ancestors() {
        if anc.join("CACHEDIR.TAG").exists() {
            return anc.to_path_buf();
        }
    }
    let fallback = manifest.join("../target");
    if fallback.join("CACHEDIR.TAG").exists() {
        return fallback;
    }
    panic!(
        "site-builder: cannot derive the target dir from {}",
        manifest.display()
    );
}

pub fn pkg_dir_matches(dir: &str, pkg: &str) -> bool {
    if dir == pkg {
        return true;
    }
    match dir.strip_prefix(&format!("{pkg}-")) {
        Some(rest) => !rest.is_empty() && rest.chars().all(|c| c.is_ascii_hexdigit()),
        None => false,
    }
}

pub fn newest_bundle(build_dir: &Path, pkg: &str, file: &str) -> PathBuf {
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
        .unwrap_or_else(|_| panic!("site-builder: no build dir at {}", build_dir.display()));
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
            "site-builder: no {file} for package {pkg} under {} (build it first)",
            build_dir.display()
        )
    })
}

pub fn split_runtime(text: &str) -> (String, String) {
    let marker = "// rustc_codegen_js runtime preamble (inlined into every bundle).";
    let start = text
        .find(marker)
        .expect("site-builder: no runtime marker in bundle");
    let line_start = text[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let end_marker = "\n// ---- functions ----\n";
    let end = text
        .find(end_marker)
        .expect("site-builder: no functions marker in bundle");
    let runtime = text[line_start..end].to_string();
    let rest = format!("{}{}", &text[..line_start], &text[end..]);
    (runtime, rest)
}

pub fn minify_js(text: &str, script: bool, top_level: bool) -> String {
    let allocator = Allocator::default();
    let source_type = if script {
        SourceType::script()
    } else {
        SourceType::mjs()
    };
    let ret = Parser::new(&allocator, text, source_type).parse();
    if !ret.diagnostics.is_empty() {
        panic!("site-builder: minify parse errors: {:?}", ret.diagnostics);
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
        compress: Some(CompressOptions::smallest()),
    };
    let mr = Minifier::new(opts).minify(&allocator, &mut program);
    Codegen::new()
        .with_options(CodegenOptions::minify())
        .with_scoping(mr.scoping)
        .build(&program)
        .code
}

pub fn process_site(
    manifest: PathBuf,
    site_dir_name: &str,
    template_rel: &str,
    slot: &str,
    watchers: &[&str],
    backend_so: bool,
    bundles: &[(&str, &str)],
    minify: bool,
    required_contains: &[&str],
    combine: fn(Vec<String>) -> String,
) {
    for w in watchers {
        println!("cargo::rerun-if-changed={}", manifest.join(w).display());
    }
    println!("cargo::rerun-if-changed=build.rs");
    if backend_so {
        println!("cargo::rerun-if-changed=../../target/release/libcodegen_js.so");
    }

    let profile = env_str("PROFILE");
    let target = workspace_target(&manifest);
    let build_dir = target.join(&profile).join("build");

    let mut bundle_paths = Vec::new();
    let mut bundle_texts = Vec::new();
    for (pkg, file) in bundles {
        let path = newest_bundle(&build_dir, pkg, file);
        let text = fs::read_to_string(&path).expect("read bundle");
        bundle_paths.push(path);
        bundle_texts.push(text);
    }
    let combined = combine(bundle_texts);
    let final_bundle = if minify {
        minify_js(&combined, false, true)
    } else {
        combined
    };

    for check in required_contains {
        if !final_bundle.contains(check) {
            panic!(
                "site-builder: final bundle missing required substring `{check}`"
            );
        }
    }
    if final_bundle.to_lowercase().contains("</script") {
        panic!("site-builder: final bundle contains </script and cannot be inlined");
    }

    let template_path = manifest.join(template_rel);
    let mut page = fs::read_to_string(&template_path).expect("read index.html template");
    if !page.contains(slot) {
        panic!("site-builder: template is missing placeholder {slot}");
    }
    page = page.replacen(slot, &final_bundle, 1);

    let site = target.join(&profile).join(site_dir_name);
    fs::create_dir_all(&site).expect("create site dir");
    fs::write(site.join("index.html"), &page).expect("write index.html");
    fs::write(site.join("bundle.mjs"), &final_bundle).expect("write bundle.mjs");
    for bp in &bundle_paths {
        let src = bp.with_extension("d.ts");
        if src.exists() {
            let dst = bp
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| format!("{s}.d.ts"))
                .unwrap_or_else(|| "bundle.d.ts".to_string());
            let _ = fs::copy(&src, site.join(dst));
        }
    }
    println!(
        "cargo::warning=site-builder: wrote {}",
        site.join("index.html").display()
    );
}

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

struct Options {
    command: String,
    manifest: Option<PathBuf>,
    release: bool,
    port: String,
    pages: Option<Vec<String>>,
    no_build: bool,
}

struct Site {
    manifest: PathBuf,
    package: String,
    target_dir: PathBuf,
    pages: Vec<String>,
    site_dir: String,
    bundles_dir: PathBuf,
    client_features: Vec<String>,
    server_features: Vec<String>,
    bin: String,
    src_dir: PathBuf,
    template: PathBuf,
    css_path: PathBuf,
    bundles: Vec<String>,
}

fn resolve_manifest(given: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(p) = given {
        return canonical(&p);
    }
    let cwd = env::current_dir().map_err(|e| e.to_string())?;
    let here = cwd.join("Cargo.toml");
    if here.exists() {
        return canonical(&here);
    }
    let demo = cwd.join("examples/demo-app/Cargo.toml");
    if demo.exists() {
        return canonical(&demo);
    }
    Err("could not find Cargo.toml; pass --manifest-path".into())
}

fn canonical(p: &Path) -> Result<PathBuf, String> {
    fs::canonicalize(p).map_err(|e| format!("{}: {e}", p.display()))
}

fn load_site(opts: &Options) -> Result<Site, String> {
    let manifest = resolve_manifest(opts.manifest.clone())?;
    let out = Command::new("cargo")
        .args(["metadata", "--format-version=1", "--no-deps", "--manifest-path"])
        .arg(&manifest)
        .output()
        .map_err(|e| format!("cargo metadata: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    }
    let v: serde_json::Value =
        serde_json::from_slice(&out.stdout).map_err(|e| format!("cargo metadata: {e}"))?;
    let target_dir = PathBuf::from(
        v["target_directory"]
            .as_str()
            .ok_or("cargo metadata: missing target_directory")?,
    );
    let packages = v["packages"].as_array().ok_or("cargo metadata: missing packages")?;
    let has_site = |p: &&serde_json::Value| {
        p.get("metadata").and_then(|m| m.get("site")).is_some()
    };
    let exact = packages.iter().find(|p| {
        p["manifest_path"]
            .as_str()
            .map(|m| canonical(Path::new(m)).ok().as_deref() == Some(manifest.as_path()))
            .unwrap_or(false)
    });
    let pkg = match exact.filter(has_site) {
        Some(p) => p,
        _ => {
            let mut sites = packages.iter().filter(has_site);
            match (sites.next(), sites.next()) {
                (Some(p), None) => p,
                _ => return Err("workspace has multiple (or no) site packages; pass --manifest-path".into()),
            }
        }
    };
    let package = pkg["name"].as_str().ok_or("cargo metadata: missing package name")?.to_string();
    let meta = pkg.get("metadata").and_then(|m| m.get("site"));
    let strings = |key: &str| -> Vec<String> {
        meta.and_then(|m| m.get(key))
            .and_then(|x| x.as_array())
            .map(|a| a.iter().filter_map(|s| s.as_str().map(str::to_string)).collect())
            .unwrap_or_default()
    };
    let pages = opts.pages.clone().unwrap_or_else(|| strings("pages"));
    let site_dir = meta
        .and_then(|m| m.get("site-dir"))
        .and_then(|x| x.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| format!("{package}-site"));
    let profile_dir = target_dir.join(if opts.release { "release" } else { "debug" });
    let bundles_dir = meta
        .and_then(|m| m.get("bundles-dir"))
        .and_then(|x| x.as_str())
        .map(|p| manifest.parent().unwrap_or(Path::new(".")).join(p))
        .unwrap_or_else(|| profile_dir.join(format!("{package}-bundles")));
    let mut client_features = strings("client-features");
    if client_features.is_empty() {
        client_features.push("client".into());
    }
    let mut server_features = strings("server-features");
    if server_features.is_empty() {
        server_features.push("server".into());
    }
    let bin = meta
        .and_then(|m| m.get("bin"))
        .and_then(|x| x.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| package.clone());
    let workspace_root = PathBuf::from(v["workspace_root"].as_str().unwrap_or(""));
    let manifest_dir = manifest.parent().unwrap_or(Path::new("."));
    let src_dir = pkg["targets"]
        .as_array()
        .and_then(|t| t.first())
        .and_then(|t| t["src_path"].as_str())
        .and_then(|p| Path::new(p).parent())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest_dir.to_path_buf());
    let template = meta
        .and_then(|m| m.get("template"))
        .and_then(|x| x.as_str())
        .map(|p| manifest_dir.join(p))
        .unwrap_or_else(|| manifest_dir.join("index.html"));
    let css_path = meta
        .and_then(|m| m.get("tailwind-css"))
        .and_then(|x| x.as_str())
        .map(|p| manifest_dir.join(p))
        .unwrap_or_else(|| workspace_root.join("tailwind/styles-full.css"));
    let mut bundles = strings("bundles");
    if bundles.is_empty() {
        bundles = ["fw_core", "fw_render", "fw_tailwind", "dom", "js_util"]
            .into_iter()
            .map(str::to_string)
            .collect();
    }
    if !bundles.iter().any(|b| b == &package) {
        bundles.push(package.clone());
    }
    Ok(Site { manifest, package, target_dir, pages, site_dir, bundles_dir, client_features, server_features, bin, src_dir, template, css_path, bundles })
}

fn cargo_build_js() -> Command {
    if let Ok(p) = env::var("CARGO_BUILD_JS") {
        return Command::new(p);
    }
    if Command::new("cargo-build-js").arg("--help").output().is_ok() {
        return Command::new("cargo-build-js");
    }
    let mut c = Command::new("cargo");
    let tool = Path::new(env!("CARGO_MANIFEST_DIR")).join("../cargo-build-js/Cargo.toml");
    c.args(["run", "--quiet", "--manifest-path"]).arg(tool).arg("--");
    c
}

fn run(mut cmd: Command, what: &str) -> Result<(), String> {
    let status = cmd.status().map_err(|e| format!("{what}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{what} failed"))
    }
}

fn collect_classes(dir: &Path, template: &Path) -> (HashSet<String>, HashSet<String>) {
    let class_re = regex::Regex::new(r#"class\s*=\s*["']([^"']*)["']"#).unwrap();
    let possible_re =
        regex::Regex::new(r#"possible_css_classes\s*\([^)]*\)\s*->[^{]*\{([^}]*)\}"#).unwrap();
    let string_re = regex::Regex::new(r#""([^"]*)""#).unwrap();
    let mut used = HashSet::new();
    let mut possible = HashSet::new();
    fn walk(
        dir: &Path,
        class_re: &regex::Regex,
        possible_re: &regex::Regex,
        string_re: &regex::Regex,
        used: &mut HashSet<String>,
        possible: &mut HashSet<String>,
    ) {
        let Ok(entries) = fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, class_re, possible_re, string_re, used, possible);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                if let Ok(text) = fs::read_to_string(&path) {
                    for cap in class_re.captures_iter(&text) {
                        for tok in cap[1].split_whitespace() {
                            used.insert(tok.to_string());
                        }
                    }
                    for cap in possible_re.captures_iter(&text) {
                        for lit in string_re.captures_iter(&cap[1]) {
                            for tok in lit[1].split_whitespace() {
                                possible.insert(tok.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    walk(dir, &class_re, &possible_re, &string_re, &mut used, &mut possible);
    if let Ok(text) = fs::read_to_string(template) {
        for cap in class_re.captures_iter(&text) {
            for tok in cap[1].split_whitespace() {
                used.insert(tok.to_string());
            }
        }
    }
    (used, possible)
}

fn write_css(site: &Site) -> Result<(), String> {
    let Ok(full) = fs::read_to_string(&site.css_path) else {
        eprintln!("==> css: {} missing; skipping split", site.css_path.display());
        return Ok(());
    };
    let (used, possible) = collect_classes(&site.src_dir, &site.template);
    let lazy: HashSet<String> = possible.difference(&used).cloned().collect();
    let split = fw_tailwind_build::split_sets(&full, &used, &lazy)?;
    fs::create_dir_all(&site.bundles_dir).map_err(|e| e.to_string())?;
    fs::write(site.bundles_dir.join("critical.css"), &split.critical).map_err(|e| e.to_string())?;
    fs::write(site.bundles_dir.join("tailwind-lazy.css"), &split.lazy).map_err(|e| e.to_string())?;
    eprintln!("==> css: critical {} bytes, lazy {} bytes", split.critical.len(), split.lazy.len());
    Ok(())
}

fn build(site: &Site, release: bool) -> Result<(), String> {
    let profile = if release { "release" } else { "debug" };
    let profile_dir = site.target_dir.join(profile);
    let build_dir = profile_dir.join("build");
    let pages: Vec<String> = if site.pages.is_empty() { vec![String::new()] } else { site.pages.clone() };
    fs::create_dir_all(&site.bundles_dir).map_err(|e| e.to_string())?;
    for page in &pages {
        let mut features = site.client_features.clone();
        if !page.is_empty() {
            features.push(format!("page_{page}"));
        }
        eprintln!("==> client {}", if page.is_empty() { "(default)" } else { page });
        let mut cmd = cargo_build_js();
        cmd.args(["build", "--manifest-path"]).arg(&site.manifest).args(["-p", &site.package, "--features"]).arg(features.join(","));
        if release {
            cmd.arg("--release");
        }
        run(cmd, "cargo build-js")?;
        let mut texts = Vec::new();
        for pkg in &site.bundles {
            let file = format!("{}.mjs", pkg.replace('-', "_"));
            let path = site_builder::newest_bundle(&build_dir, pkg, &file);
            texts.push(fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?);
        }
        let combined = site_builder::combine_bundles(&texts);
        let bundle = site_builder::minify_js(&combined, false, true);
        let dst = if page.is_empty() {
            site.bundles_dir.join("bundle.mjs")
        } else {
            site.bundles_dir.join(format!("{page}.mjs"))
        };
        fs::write(&dst, bundle).map_err(|e| format!("{}: {e}", dst.display()))?;
    }
    write_css(site)?;
    eprintln!("==> server {}", profile);
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--manifest-path"]).arg(&site.manifest).args(["-p", &site.package, "--features"]).arg(site.server_features.join(","));
    if release {
        cmd.args(["--release"]).env("CARGO_PROFILE_RELEASE_LTO", "off");
    }
    run(cmd, "cargo build")?;
    eprintln!("==> built {} bundles in {}", pages.len(), site.bundles_dir.display());
    Ok(())
}

fn run_site(site: &Site, release: bool, port: &str) -> Result<(), String> {
    let bin = site.target_dir.join(if release { "release" } else { "debug" }).join(&site.bin);
    if !bin.exists() {
        return Err(format!("missing {}; run `cargo site build`", bin.display()));
    }
    eprintln!("==> running {} on 127.0.0.1:{port}", bin.display());
    let mut cmd = Command::new(&bin);
    cmd.env("BUNDLES_DIR", &site.bundles_dir)
        .arg(format!("127.0.0.1:{port}"));
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let err = cmd.exec();
        return Err(format!("run {}: {err}", bin.display()));
    }
    #[cfg(not(unix))]
    {
        let status = cmd.status().map_err(|e| format!("run {}: {e}", bin.display()))?;
        if status.success() { Ok(()) } else { Err("server exited with an error".into()) }
    }
}

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("site") {
        args.remove(0);
    }
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        println!("cargo site build [--manifest-path PATH] [--pages a,b]");
        println!("cargo site run   [--manifest-path PATH] [--port 8080] [--no-build]");
        return ExitCode::SUCCESS;
    }
    let command = args.remove(0);
    let mut opts = Options { command: command.clone(), manifest: None, release: true, port: "8080".into(), pages: None, no_build: false };
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--manifest-path" | "--manifest" => {
                i += 1;
                opts.manifest = args.get(i).map(PathBuf::from);
            }
            "--debug" => opts.release = false,
            "--no-build" => opts.no_build = true,
            "--release" => opts.release = true,
            "--port" | "-p" => {
                i += 1;
                if let Some(p) = args.get(i) { opts.port = p.clone(); }
            }
            "--pages" => {
                i += 1;
                if let Some(p) = args.get(i) {
                    opts.pages = Some(p.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
                }
            }
            other if other.starts_with("--manifest-path=") => {
                opts.manifest = Some(PathBuf::from(other.trim_start_matches("--manifest-path=")));
            }
            other => return fail(&format!("unknown argument: {other}")),
        }
        i += 1;
    }
    let site = match load_site(&opts) {
        Ok(s) => s,
        Err(e) => return fail(&e),
    };
    let result = match opts.command.as_str() {
        "build" => build(&site, opts.release),
        "run" => {
            if opts.no_build {
                run_site(&site, opts.release, &opts.port)
            } else {
                build(&site, opts.release).and_then(|_| run_site(&site, opts.release, &opts.port))
            }
        }
        "clean" => {
            let _ = fs::remove_dir_all(&site.bundles_dir);
            let _ = fs::remove_dir_all(site.target_dir.join(if opts.release { "release" } else { "debug" }).join(&site.site_dir));
            Ok(())
        }
        other => Err(format!("unknown command: {other}")),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => fail(&e),
    }
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("cargo site: {msg}");
    ExitCode::FAILURE
}

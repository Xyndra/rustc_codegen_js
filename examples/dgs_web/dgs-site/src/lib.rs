// Assembled by `dgs-site/build.rs`; see that file.
// The `extern crate` anchors make the `[dependencies] in Cargo.toml real:
// cargo builds `dgs`/`dgs-playground` (and their JavaScript bundles, which
// the build script inlines) before this crate.
extern crate dgs_playground as _;

#[allow(dead_code)]
fn _site_anchor() {}

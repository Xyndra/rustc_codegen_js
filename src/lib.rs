//! rustc_codegen_js — Rust → TypeScript/JavaScript codegen backend.
//!
//! rustc performs parsing, expansion, borrow checking, type checking, trait
//! resolution, and monomorphization as usual; only the final code generation
//! stage is replaced. There is no linear memory and no allocator: values use
//! the representations in `tyutil` (objects/arrays/strings/BigInt).
//!
//! # Spec deviations (pinned nightly-2026-09-15)
//! - §3 "Hook A" (`CodegenBackend::mir_built`) does not exist on this
//!   toolchain's `CodegenBackend` trait (only `codegen_crate` /
//!   `join_codegen` / `link`). Library recognition (§6.1) is therefore
//!   applied at emission time rather than via MIR rewriting — equivalent,
//!   since the backend owns instruction selection either way. Local bodies
//!   are read pre-optimization (`mir_drops_elaborated_and_const_checked`,
//!   so calls are still calls); upstream bodies via `optimized_mir`.
//! - §3 `Rvalue::BinaryOp` has no fourth overflow operand on this toolchain;
//!   overflow checks are separate `Assert` terminators (handled).
//! - §8 `extern "JS"` blocks are rejected by rustc itself (E0703); the
//!   portable attribute form is used instead: `#[js_import]` on
//!   `extern "C"` declarations, `#[js_export]` on free functions.
//! - §4 default for 64/128-bit integers is BigInt (not split lo/hi pairs).

#![feature(rustc_private)]
#![recursion_limit = "256"]

extern crate rustc_abi;
extern crate rustc_attr_ir;
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_mir_transform;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_structures;
extern crate rustc_type_ir;

mod constval;
mod emit;
mod recognize;
mod structure;
pub mod tyutil;

use std::any::Any;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use rustc_codegen_ssa::traits::CodegenBackend;
use rustc_codegen_ssa::{CompiledModules, CrateInfo, TargetConfig};
use rustc_data_structures::steal::Steal;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_index::Idx;
use rustc_middle::dep_graph::WorkProductMap;
use rustc_middle::mir;
use rustc_middle::mir::Body;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_session::config::{OutFileName, OutputFilenames, OutputType};
use rustc_session::{IncrCompSession, Session};
use rustc_span::{Span, Symbol};
use rustc_structures::CrateType;

use emit::{body_for, Ctx, Emitter, ExportInfo};

const RUNTIME_JS: &str = include_str!("runtime.js");

pub struct JsCodegenBackend;

impl CodegenBackend for JsCodegenBackend {
    fn name(&self) -> &'static str {
        "js"
    }

    /// Hook A wiring (§3/§6): override the elaborated-MIR provider so the
    /// backend snapshots every local body *before* optimization steals it.
    /// The snapshot logic replicates the default provider and additionally
    /// clones the body into a side table; downstream queries (including
    /// `optimized_mir`) behave exactly as before.
    fn provide(&self, providers: &mut rustc_middle::util::Providers) {
        providers.queries.mir_drops_elaborated_and_const_checked = mir_hook;
    }

    fn target_config(&self, _sess: &Session) -> TargetConfig {
        TargetConfig {
            internal_target_features: Default::default(),
            has_reliable_f16: true,
            has_reliable_f16_math: true,
            has_reliable_f128: true,
            has_reliable_f128_math: true,
        }
    }

    fn supported_crate_types(&self, _sess: &Session) -> Vec<CrateType> {
        vec![CrateType::Rlib]
    }

    fn target_cpu(&self, _sess: &Session) -> String {
        String::new()
    }

    fn codegen_crate<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Box<dyn Any> {
        let crate_name = tcx
            .crate_name(rustc_span::def_id::LOCAL_CRATE)
            .to_string()
            .replace('-', "_");

        // Read shared state from earlier invocations so the final drain has a
        // complete view of all exports and reachable instances.
        let _state = read_state();

        all_crates().lock().unwrap().push(crate_name.clone());
        persist_crates();

        g_bodies()
            .lock()
            .unwrap()
            .entry(crate_name.clone())
            .or_default();
        g_order()
            .lock()
            .unwrap()
            .entry(crate_name.clone())
            .or_default();
        g_queued()
            .lock()
            .unwrap()
            .entry(crate_name.clone())
            .or_default();
        g_exports()
            .lock()
            .unwrap()
            .entry(crate_name.clone())
            .or_default();
        g_name_taken()
            .lock()
            .unwrap()
            .entry(crate_name.clone())
            .or_default();
        g_requires()
            .lock()
            .unwrap()
            .entry(crate_name.clone())
            .or_default();

        if let Err(e) = codegen_js(tcx, &crate_name) {
            tcx.dcx().fatal(e.to_string());
        }

        // Record this crate's output directory so the drain can write bundles
        // to the right place.
        let out_dir = tcx.sess.io.output_dir.clone().unwrap_or_default();
        persist_out_dir(&crate_name, &out_dir);
        persist_crates();

        // Assemble and write all bundles (last invocation has complete state).
        write_all_bundles(tcx, tcx.sess);

        Box::new(crate_name)
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        sess: &Session,
        _incr_comp_session: Option<&IncrCompSession>,
        outputs: &OutputFilenames,
        _crate_info: &CrateInfo,
    ) -> (CompiledModules, WorkProductMap) {
        let _ = sess;
        let crate_name = ongoing_codegen
            .downcast_ref::<String>()
            .expect("JsCodegenBackend returned the crate name")
            .clone();
        let out_dir = match outputs.path(OutputType::Metadata) {
            OutFileName::Real(p) => p
                .parent()
                .map(|d| d.to_path_buf())
                .unwrap_or_else(|| PathBuf::from(".")),
            OutFileName::Stdout => PathBuf::from("."),
        };
        let _ = fs::create_dir_all(&out_dir);
        g_outdirs()
            .lock()
            .unwrap()
            .entry(crate_name.clone())
            .or_insert(out_dir.clone());
        b_outdirs()
            .lock()
            .unwrap()
            .push((crate_name.clone(), out_dir));
        (
            CompiledModules {
                modules: vec![],
                allocator_module: None,
            },
            WorkProductMap::default(),
        )
    }

    fn link(
        &self,
        sess: &Session,
        compiled_modules: CompiledModules,
        crate_info: CrateInfo,
        metadata: rustc_metadata::EncodedMetadata,
        outputs: &OutputFilenames,
    ) {
        rustc_codegen_ssa::back::link::link_binary(
            sess,
            &rustc_codegen_ssa::back::archive::ArArchiveBuilderBuilder,
            compiled_modules,
            crate_info,
            metadata,
            outputs,
            self.name(),
        );
    }
}

// ---------------------------------------------------------------------------
// Cross-crate worklist & bundle assembly.
//
// Each crate's #[js_export] roots are discovered in codegen_js and inserted
// into that crate's entry in the global maps. Emitting a call to an upstream
// instance enqueues it into the upstream crate's entry (Ctx::fn_name). The
// final join_codegen — reached only once every crate has finished
// codegen_crate — drains the whole worklist: dependents before dependencies
// so that every referenced function is emitted, then writes one
// <crate>.mjs + <crate>.d.ts per crate from the assembled global state.
// ---------------------------------------------------------------------------

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

/// Unique instance key across the whole compilation.
///
/// Materializes only the DefId; generic arguments are ignored (the
/// cross-invocation state file format has no room for them). This keeps the
/// cache type lifetime-free, so instance keys can be stored in the global
/// maps without extending `'tcx` via `transmute`.
pub(crate) type InstKey = InstKeySer;
type CrateKey = String;

static GLOBAL_FN_NAMES: OnceLock<Mutex<HashMap<InstKey, String>>> = OnceLock::new();
/// Names taken within each crate: key -> set of used names (renumbered on collision).
static GLOBAL_NAME_TAKEN: OnceLock<Mutex<HashMap<CrateKey, HashSet<String>>>> = OnceLock::new();
static GLOBAL_BODIES: OnceLock<Mutex<HashMap<CrateKey, HashMap<InstKey, String>>>> =
    OnceLock::new();
static GLOBAL_ORDER: OnceLock<Mutex<HashMap<CrateKey, Vec<InstKey>>>> = OnceLock::new();
static GLOBAL_QUEUED: OnceLock<Mutex<HashMap<CrateKey, HashSet<InstKey>>>> = OnceLock::new();
static GLOBAL_EXPORTS: OnceLock<Mutex<HashMap<CrateKey, Vec<ExportInfo>>>> = OnceLock::new();
static GLOBAL_IMPORT_NAMES: OnceLock<Mutex<HashMap<rustc_hir::def_id::DefId, String>>> =
    OnceLock::new();
static GLOBAL_OUTDIRS: OnceLock<Mutex<HashMap<CrateKey, PathBuf>>> = OnceLock::new();
static BUNDLE_OUTDIRS: OnceLock<Mutex<Vec<(CrateKey, PathBuf)>>> = OnceLock::new();
/// Dependency edges recorded by `fn_name`: `A requires B` means crate `A` has a
/// call target defined in crate `B`; used to derive the drain (dependent-first)
/// and write (dependency-first) orders for cross-crate bundles.
static GLOBAL_REQUIRES: OnceLock<Mutex<HashMap<CrateKey, HashSet<CrateKey>>>> = OnceLock::new();
static ALL_CRATES: OnceLock<Mutex<Vec<CrateKey>>> = OnceLock::new();
/// Crates that declare at least one `#[js_export]` item. Emission failures
/// in such crates are hard errors (their code is the program entry point and
/// must work); in crates without exports (e.g. upstream libraries like
/// `dgs`, whose un-emittable bodies such as serde derives are never called)
/// failures become warnings plus a throwing stub so one unsupported helper
/// cannot block the whole bundle.
static STRICT_CRATES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

/// `true` if `crate_name` declares `#[js_export]` items (strict diagnostics).
pub fn crate_is_strict(crate_name: &str) -> bool {
    STRICT_CRATES
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap()
        .contains(crate_name)
}

fn g_fn_names() -> &'static Mutex<HashMap<InstKey, String>> {
    GLOBAL_FN_NAMES.get_or_init(|| Mutex::new(HashMap::new()))
}
fn g_name_taken() -> &'static Mutex<HashMap<CrateKey, HashSet<String>>> {
    GLOBAL_NAME_TAKEN.get_or_init(|| Mutex::new(HashMap::new()))
}
fn g_bodies() -> &'static Mutex<HashMap<CrateKey, HashMap<InstKey, String>>> {
    GLOBAL_BODIES.get_or_init(|| Mutex::new(HashMap::new()))
}
fn g_order() -> &'static Mutex<HashMap<CrateKey, Vec<InstKey>>> {
    GLOBAL_ORDER.get_or_init(|| Mutex::new(HashMap::new()))
}
fn g_queued() -> &'static Mutex<HashMap<CrateKey, HashSet<InstKey>>> {
    GLOBAL_QUEUED.get_or_init(|| Mutex::new(HashMap::new()))
}
fn g_exports() -> &'static Mutex<HashMap<CrateKey, Vec<ExportInfo>>> {
    GLOBAL_EXPORTS.get_or_init(|| Mutex::new(HashMap::new()))
}
fn g_import_names() -> &'static Mutex<HashMap<rustc_hir::def_id::DefId, String>> {
    GLOBAL_IMPORT_NAMES.get_or_init(|| Mutex::new(HashMap::new()))
}
fn g_outdirs() -> &'static Mutex<HashMap<CrateKey, PathBuf>> {
    GLOBAL_OUTDIRS.get_or_init(|| Mutex::new(HashMap::new()))
}
fn g_requires() -> &'static Mutex<HashMap<CrateKey, HashSet<CrateKey>>> {
    GLOBAL_REQUIRES.get_or_init(|| Mutex::new(HashMap::new()))
}
fn b_outdirs() -> &'static Mutex<Vec<(CrateKey, PathBuf)>> {
    BUNDLE_OUTDIRS.get_or_init(|| Mutex::new(Vec::new()))
}
fn all_crates() -> &'static Mutex<Vec<CrateKey>> {
    ALL_CRATES.get_or_init(|| Mutex::new(Vec::new()))
}

// ---------------------------------------------------------------------------
// Shared state across rustc invocations (persisted to disk).
// Each JS crate's export roots and reachable-instance order are recorded here
// so that the final drain (run in the last-crate invocation) can assemble all
// bundles from a complete view.
// ---------------------------------------------------------------------------

/// Serializable instance key: a DefId plus a monomorphization fingerprint.
///
/// The fingerprint is empty for identity-arg instances (the overwhelmingly
/// common case, and the only one the key format historically supported), so
/// every existing mono flow keeps byte-identical keys. Non-empty only for
/// genuinely monomorphized generic instantiations (`area_of::<Sq>` vs
/// `area_of::<R>`), which previously collapsed onto one key and could not
/// compile. The fingerprint is FNV-1a over the args' `Debug` rendering:
/// deterministic within and across processes, and free of the state file's
/// `;`/space separators.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstKeySer {
    krate: usize,
    idx: usize,
    local: bool,
    local_idx: Option<usize>,
    args_fp: String,
}

/// Convert a global InstKey into its serialized form.
fn inst_key_to_str(k: &crate::InstKey) -> String {
    k.to_str()
}

impl InstKeySer {
    pub(crate) fn to_str(&self) -> String {
        format!(
            "{};{};{};{};{}",
            self.krate,
            self.idx,
            self.local,
            self.local_idx
                .map(|i| i.to_string())
                .unwrap_or_else(|| "!0".to_string()),
            self.args_fp,
        )
    }

    pub(crate) fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(';').collect();
        if parts.len() != 4 && parts.len() != 5 {
            return None;
        }
        Some(InstKeySer {
            krate: parts[0].parse().ok()?,
            idx: parts[1].parse().ok()?,
            local: parts[2] == "true",
            local_idx: if parts[2] == "true" {
                Some(parts[3].parse().ok()?)
            } else {
                None
            },
            // Legacy 4-part keys (pre-monomorphization state files).
            args_fp: parts.get(4).unwrap_or(&"").to_string(),
        })
    }

    /// Rebuild the instance using the current tcx.
    ///
    /// Reconstructs the item's generic parameters from identity, then
    /// concretizes any opaque impl-trait params (`impl Into<T>` / `impl
    /// From<S>`) to the projection's output type: the item's MIR params are
    /// already lowered to those types, and the caller passes concrete values,
    /// so this reproduces the concrete instance rustc would build at a call
    /// site and lets `recognize` resolve the calls inside the body.
    fn to_inst<'tcx>(&self, tcx: TyCtxt<'tcx>) -> ty::Instance<'tcx> {
        let krate = rustc_span::def_id::CrateNum::new(self.krate);
        let idx = rustc_span::def_id::DefIndex::new(self.idx);
        let did = if self.local {
            rustc_hir::def_id::DefId::local(idx)
        } else {
            rustc_hir::def_id::DefId { krate, index: idx }
        };
        let args = self
            .concretize_args(tcx)
            .unwrap_or_else(|| ty::GenericArgs::identity_for_item(tcx, did));
        ty::Instance::new_raw(did, &args)
    }

    fn concretize_args<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Option<ty::GenericArgsRef<'tcx>> {
        let did = self.def_id();
        // own_params are the item's generic parameters (Self excluded); each is
        // turned into a concrete arg by taking the identity arg and
        // concretizing opaque impl-trait params.
        let identity = ty::GenericArgs::identity_for_item(tcx, did);
        let mut concrete = Vec::with_capacity(identity.len());
        // Identity args may include lifetimes/consts (e.g. serde's `'_de`);
        // only type args are candidates for opaque concretization — pass
        // everything else through untouched (`type_at` would panic on them).
        for arg in identity.iter() {
            match arg.kind() {
                ty::GenericArgKind::Type(t) => {
                    concrete.push(subst_opaque(tcx, t).unwrap_or(arg));
                }
                _ => concrete.push(arg),
            }
        }
        let changed = concrete.len() != identity.len()
            || concrete.iter().zip(identity.iter()).any(|(a, b)| *a != b);
        changed.then(|| tcx.mk_args(&concrete))
    }

    /// Reconstruct the DefId from the serialized key fields.
    /// Generic arguments are always empty (mono instances only), so none are
    /// needed to rebuild the DefId; `local_idx` is relevant only for local defs
    /// and is verified but not used to construct the DefId itself.
    fn def_id(&self) -> rustc_hir::def_id::DefId {
        let krate = rustc_span::def_id::CrateNum::new(self.krate);
        let idx = rustc_span::def_id::DefIndex::new(self.idx);
        let did = if self.local {
            rustc_hir::def_id::DefId::local(idx)
        } else {
            rustc_hir::def_id::DefId { krate, index: idx }
        };
        // Sanity-check the reconstructed local index against the key.
        if self.local
            && self.local_idx.is_some()
            && Some(self.local_idx.unwrap()) != did.as_local().map(|l| l.index())
        {
            eprintln!(
                "codegen_js: local_idx mismatch for {} (key={:?}) -> {:#?}",
                self.to_str(),
                self.local_idx,
                did
            );
        }
        did
    }

    /// Generic arguments are dropped in the key: only mono instances are
    /// emitted, so the arguments are always empty.
    fn args<'tcx>(&self, _tcx: TyCtxt<'tcx>) -> ty::GenericArgsRef<'tcx> {
        ty::List::empty()
    }
}

/// Concretize an opaque impl-trait parameter to its projection output type.
fn subst_opaque<'a>(tcx: TyCtxt<'a>, ty: ty::Ty<'a>) -> Option<ty::GenericArg<'a>> {
    let ty::TyKind::Alias(
        _,
        ty::AliasTy {
            kind: ty::AliasTyKind::Opaque { def_id },
            args,
            ..
        },
    ) = *ty.kind()
    else {
        return None;
    };
    // `impl Into<T>` / `impl From<S>`: the opaque alias carries the trait's
    // args [Self (hidden impl), T]; for Into/From the projection's output
    // type is exactly T, the second alias arg.
    if args.len() == 2 && is_convert_trait(tcx, def_id) {
        return Some(args[1]);
    }
    None
}

fn is_convert_trait(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    let p = crate::tyutil::def_path(tcx, def_id);
    p.ends_with("convert::Into") || p.ends_with("convert::From") || p == "Into" || p == "From"
}

impl InstKeySer {
    /// Build a cache key from an `Instance<'tcx>`.
    ///
    /// The key materializes the DefId plus a fingerprint of non-identity
    /// generic args, so distinct monomorphizations (`f::<A>` vs `f::<B>`)
    /// get distinct keys/names/bodies. Identity-arg instances keep the
    /// legacy empty fingerprint, so all existing mono flows (including
    /// cross-crate naming) are unchanged.
    pub fn from_instance<'tcx>(tcx: TyCtxt<'tcx>, inst: &ty::Instance<'tcx>) -> Self {
        let did = inst.def_id();
        InstKeySer {
            krate: did.krate.as_usize(),
            idx: did.index.as_u32() as usize,
            local: did.is_local(),
            local_idx: did.as_local().map(|l| l.index()),
            args_fp: args_fingerprint(tcx, inst),
        }
    }

    /// Build a cache key directly from a `DefId`.
    pub fn from_def_id(did: rustc_hir::def_id::DefId) -> Self {
        InstKeySer {
            krate: did.krate.as_usize(),
            idx: did.index.as_u32() as usize,
            local: did.is_local(),
            local_idx: did.as_local().map(|l| l.index()),
            args_fp: String::new(),
        }
    }
}

/// Fingerprint of an instance's generic args: `""` for identity args,
/// otherwise FNV-1a (hex) over the args' `Debug` rendering. FNV is
/// process-independent, so keys match across invocations; the hex digest
/// contains no state-file separators.
fn args_fingerprint<'tcx>(tcx: TyCtxt<'tcx>, inst: &ty::Instance<'tcx>) -> String {
    if inst.args == ty::GenericArgs::identity_for_item(tcx, inst.def_id()) {
        return String::new();
    }
    let mut h: u64 = 0xcbf29ce484222325;
    for b in format!("{:?}", inst.args).bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

/// Concrete instances registered by `fn_name` during this invocation, keyed
/// by key string. Lets the drain loop emit genuinely monomorphized bodies
/// (`to_inst` can only rebuild identity args). Same lifetime discipline as
/// `PRE_OPT_MIR_SNAP`: arena data valid for the whole compilation.
static GLOBAL_CONCRETE: OnceLock<Mutex<HashMap<String, ty::Instance<'static>>>> = OnceLock::new();

fn g_concrete() -> &'static Mutex<HashMap<String, ty::Instance<'static>>> {
    GLOBAL_CONCRETE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Remember the concrete instance behind `key_str` (called on every
/// `fn_name` resolution; overwriting the same key is harmless).
pub(crate) fn remember_concrete_inst<'tcx>(key_str: &str, inst: &ty::Instance<'tcx>) {
    g_concrete()
        .lock()
        .unwrap()
        .insert(key_str.to_string(), unsafe {
            std::mem::transmute::<ty::Instance<'tcx>, ty::Instance<'static>>(*inst)
        });
}

/// The concrete instance for `key_str`, if registered during this
/// invocation (`None` for keys read from another process's state file —
/// callers fall back to `to_inst`, i.e. identity args, the old behavior).
fn concrete_inst<'tcx>(key_str: &str) -> Option<ty::Instance<'tcx>> {
    g_concrete()
        .lock()
        .unwrap()
        .get(key_str)
        .copied()
        .map(|inst| unsafe {
            std::mem::transmute::<ty::Instance<'static>, ty::Instance<'tcx>>(inst)
        })
}

/// Parsed shared state.
struct ParsedState {
    crates: Vec<String>,
    /// internal JS name assigned to an instance.
    fn_names: HashMap<String, String>,
    /// per-crate list of instances in drain order.
    order: HashMap<String, Vec<String>>,
    /// per-crate set of required (downstream) crate keys.
    requires: HashMap<String, HashSet<String>>,
    /// per-crate output directory.
    out_dirs: HashMap<String, PathBuf>,
    /// crates whose bundles have already been assembled (avoid re-borrowing MIR).
    drained: HashSet<String>,
}

const STATE_FILE: &str = "js_codegen_state_dbg";

fn read_state() -> ParsedState {
    use std::io::BufRead;
    let mut st = ParsedState {
        crates: Vec::new(),
        fn_names: HashMap::new(),
        order: HashMap::new(),
        requires: HashMap::new(),
        out_dirs: HashMap::new(),
        drained: HashSet::new(),
    };
    let Ok(f) = std::fs::File::open(STATE_FILE) else {
        return st;
    };
    for line in std::io::BufReader::new(f).lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.splitn(2, ' ');
        let tag = it.next().unwrap_or("");
        let rest = it.next().unwrap_or("");
        match tag {
            "CRATE" => {
                if let Some(c) = rest.split_whitespace().next() {
                    if !st.crates.iter().any(|s| s.as_str() == c) {
                        st.crates.push(c.to_string());
                    }
                }
            }
            "FN_NAME" => {
                // FN_NAME <inst_key> <name>
                let mut it2 = rest.splitn(2, ' ');
                let key = it2.next().unwrap_or("");
                let name = it2.next().unwrap_or("");
                st.fn_names.insert(key.to_string(), name.to_string());
            }
            "ORDER" => {
                // ORDER <crate> <inst_key>
                let mut it2 = rest.splitn(2, ' ');
                let c = it2.next().unwrap_or("");
                let key = it2.next().unwrap_or("");
                if !c.is_empty() && !key.is_empty() {
                    st.order
                        .entry(c.to_string())
                        .or_default()
                        .push(key.to_string());
                }
            }
            "REQUIRES" => {
                // REQUIRES <from> <to>
                let mut it2 = rest.splitn(2, ' ');
                let f = it2.next().unwrap_or("");
                let t = it2.next().unwrap_or("");
                if !f.is_empty() && !t.is_empty() {
                    st.requires
                        .entry(f.to_string())
                        .or_default()
                        .insert(t.to_string());
                }
            }
            "OUT_DIR" => {
                // OUT_DIR <crate> <path>
                let mut it2 = rest.splitn(2, ' ');
                let c = it2.next().unwrap_or("");
                let path = it2.next().unwrap_or("");
                if !c.is_empty() && !path.is_empty() {
                    st.out_dirs.insert(c.to_string(), PathBuf::from(path));
                }
            }
            "DRAINED" => {
                // DRAINED <crate>
                let c = rest.split_whitespace().next().unwrap_or("");
                if !c.is_empty() {
                    st.drained.insert(c.to_string());
                }
            }
            _ => {}
        }
    }
    st
}

fn persist_fn_name(key: &str, name: &str) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(STATE_FILE)
        .unwrap();
    let _ = f.write_all(format!("FN_NAME {} {}\n", key, name).as_bytes());
}

fn persist_order(crate_key: &str, key: &str) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(STATE_FILE)
        .unwrap();
    let _ = f.write_all(format!("ORDER {} {}\n", crate_key, key).as_bytes());
}

fn persist_requires(from: &str, to: &str) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(STATE_FILE)
        .unwrap();
    let _ = f.write_all(format!("REQUIRES {} {}\n", from, to).as_bytes());
}

fn persist_out_dir(crate_key: &str, out_dir: &Path) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(STATE_FILE)
        .unwrap();
    let _ = f.write_all(format!("OUT_DIR {} {}\n", crate_key, out_dir.display()).as_bytes());
}

/// Snapshot table for the pre-optimization MIR of local defs (Hook A).
/// Bodies are arena-allocated and live for the whole compilation, so we can
/// safely keep a clone here and re-read it across queries/drain cycles.
static PRE_OPT_MIR_SNAP: OnceLock<
    Mutex<HashMap<rustc_hir::def_id::LocalDefId, mir::Body<'static>>>,
> = OnceLock::new();

/// Return a pre-optimization MIR snapshot for a local def (Hook A).
/// Returns None if the def has no MIR (e.g. a shim).
fn snapshot_for(
    _tcx: TyCtxt<'_>,
    local: rustc_hir::def_id::LocalDefId,
) -> Option<mir::Body<'static>> {
    let map = PRE_OPT_MIR_SNAP.get_or_init(|| Mutex::new(HashMap::new()));
    let map = map.lock().unwrap();
    map.get(&local).cloned()
}

fn persist_crates() {
    use io::Write;
    let crates = all_crates().lock().unwrap();
    let mut written = std::collections::HashSet::new();
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(STATE_FILE)
        .unwrap();
    for c in crates.iter() {
        if written.insert(c) {
            let _ = f.write_all(format!("CRATE {c}\n").as_bytes());
        }
    }
}

fn persist_drained(crate_key: &str) {
    use io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(STATE_FILE)
        .unwrap();
    let _ = f.write_all(format!("DRAINED {crate_key}\n").as_bytes());
}

/// Is this foreign item one of this crate's `#[js_import]` declarations?
fn is_js_import(did: &rustc_hir::def_id::DefId) -> bool {
    g_import_names().lock().unwrap().contains_key(did)
}

/// Resolve the JS name of a `#[js_import]` foreign item.
fn get_import_name(did: &rustc_hir::def_id::DefId) -> Option<String> {
    g_import_names().lock().unwrap().get(did).cloned()
}

// ---------------------------------------------------------------------------
// Hook A: pre-optimization MIR provider
// ---------------------------------------------------------------------------

/// Drop-in replacement for the default
/// `mir_drops_elaborated_and_const_checked` provider: identical behavior,
/// reading bodies pre-optimization so calls are still calls. The resulting
/// pre-optimization MIR is cached in rustc's query system and re-read
/// directly by `body_for` (no per-crate snapshot table needed).
fn mir_hook<'tcx>(tcx: TyCtxt<'tcx>, def: LocalDefId) -> &'tcx Steal<Body<'tcx>> {
    if tcx.is_coroutine(def.to_def_id()) {
        tcx.ensure_done().mir_coroutine_witnesses(def);
    }
    let tainted_by_errors = if !tcx.is_synthetic_mir(def) {
        tcx.mir_borrowck(tcx.typeck_root_def_id_local(def)).err()
    } else {
        None
    };
    // `mir_inliner_callees` borrows `mir_promoted`, so it must be computed
    // (and memoized) BEFORE `mir_promoted` is stolen below. Otherwise the
    // release inliner (`mir_callgraph_cyclic`, reached via `optimized_mir`)
    // borrows the stolen value and ICEs. This mirrors the stock provider,
    // minus its `should_run_pass` gate, which lives in rustc_mir_transform's
    // private `pass_manager` module and is purely a compile-time
    // optimization: computing unconditionally is always sound.
    if tcx.def_kind(def).is_fn_like() {
        tcx.ensure_done()
            .mir_inliner_callees(ty::InstanceKind::Item(def.to_def_id()));
    }
    tcx.ensure_done().check_liveness(def);
    let (body, _) = tcx.mir_promoted(def);
    let mut body = body.steal();
    if let Some(error_reported) = tainted_by_errors {
        body.tainted_by_errors = Some(error_reported);
    }
    let root = tcx.typeck_root_def_id_local(def);
    if let Err(e) = tcx.check_transmutes(root) {
        body.tainted_by_errors = Some(e);
    }
    match tcx.def_kind(root) {
        DefKind::Fn
        | DefKind::AssocFn
        | DefKind::Static { .. }
        | DefKind::Const
        | DefKind::AssocConst => {
            if let Err(guar) = tcx.ensure_result().check_well_formed(root) {
                body.tainted_by_errors = Some(guar);
            }
        }
        _ => {}
    }
    rustc_mir_transform::run_analysis_to_runtime_passes(tcx, &mut body);
    // Snapshot this pre-optimization body (arena-allocated, lives for the
    // whole compilation) so later queries/drain cycles can re-read it without
    // re-stealing the cache.
    PRE_OPT_MIR_SNAP
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(def, unsafe {
            std::mem::transmute::<mir::Body<'tcx>, mir::Body<'static>>(body.clone())
        });
    tcx.alloc_steal_mir(body)
}

/// Boundary value mapping (subset for imports/exports).
fn boundary_ts(tcx: TyCtxt<'_>, ty: Ty<'_>) -> Result<&'static str, String> {
    if ty.is_unit() {
        return Ok("void");
    }
    match ty.kind() {
        ty::Float(_) => Ok("number"),
        ty::Int(_) | ty::Uint(_) => Ok("number"),
        ty::Bool => Ok("boolean"),
        ty::Str => Ok("string"),
        ty::Adt(def, _) => {
            let p = tyutil::def_path(tcx, def.did());
            if tyutil::is_std_item(tcx, def.did())
                && (p.ends_with("string::String") || p == "String")
            {
                Ok("string")
            } else {
                Err(format!("type {ty} cannot cross the JS boundary"))
            }
        }
        ty::Ref(_, inner, _) if matches!(inner.kind(), ty::Str) => Ok("string"),
        _ => Err(format!("type {ty} cannot cross the JS boundary")),
    }
}

/// Wrap a boundary JS string into the internal `String` cell.
fn t_wrap(param: &str) -> String {
    format!("[{param}]")
}

/// Tool attributes (`#[js::export]`, `#[js::import]`) require
/// `#![feature(register_tool)]` + `#![register_tool(js)]` in user crates
/// (users always build with our pinned nightly anyway).
fn has_tool_attr(tcx: TyCtxt<'_>, hir_id: rustc_hir::HirId, tool: &str, name: &str) -> bool {
    let path = [Symbol::intern(tool), Symbol::intern(name)];
    tcx.hir_attrs(hir_id).iter().any(|a| a.path_matches(&path))
}

fn has_attr(tcx: TyCtxt<'_>, hir_id: rustc_hir::HirId, name: &str) -> bool {
    has_tool_attr(tcx, hir_id, "js", name)
}

/// `#[js::import]` argument on an item: `None` = absent, `Some(None)` = bare
/// (`__js_imports.<name>` indirection), `Some(Some(path))` = direct call to
/// the true JS function at `<path>` (e.g. `document.getElementById("x")`),
/// `Some(Err(msg))` = malformed (diagnosed at the use site).
fn import_attr(
    tcx: TyCtxt<'_>,
    hir_id: rustc_hir::HirId,
) -> Option<Result<Option<String>, String>> {
    use rustc_attr_ir::AttrArgs;
    let path = [Symbol::intern("js"), Symbol::intern("import")];
    for a in tcx.hir_attrs(hir_id) {
        if !a.path_matches(&path) {
            continue;
        }
        match &a.get_normal_item().args {
            AttrArgs::Empty => return Some(Ok(None)),
            AttrArgs::Eq { .. } => match a.value_str() {
                Some(s) => {
                    let p = s.to_string();
                    if let Some(e) = validate_import_path(&p) {
                        return Some(Err(e));
                    }
                    return Some(Ok(Some(p)));
                }
                None => {
                    return Some(Err(
                        "expected #[js::import] or #[js::import = \"<js.path>\"]".to_string(),
                    ));
                }
            },
            _ => {
                return Some(Err(
                    "expected #[js::import] or #[js::import = \"<js.path>\"]".to_string(),
                ));
            }
        }
    }
    None
}

/// Reject import paths that cannot be safely inlined into the bundle (which
/// pages inline into HTML): empty, template substitutions, backticks,
/// newlines, and anything resembling a closing tag.
fn validate_import_path(p: &str) -> Option<String> {
    if p.is_empty() {
        return Some("import path must not be empty".to_string());
    }
    if p.len() > 512 {
        return Some("import path is too long".to_string());
    }
    let lower = p.to_lowercase();
    for bad in ["`", "${", "\n", "\r", "</script", "<!--"] {
        if lower.contains(bad) {
            return Some(format!("import path contains forbidden {bad:?}"));
        }
    }
    // Rough balance check for () and quotes (paths compose calls with string
    // literals, e.g. `document.getElementById("stage").replaceChildren`).
    let mut depth = 0i32;
    let mut quote = None;
    for c in p.chars() {
        if let Some(q) = quote {
            if c == q {
                quote = None;
            }
        } else if c == '"' || c == '\'' {
            quote = Some(c);
        } else if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth < 0 {
                return Some("import path has unbalanced parentheses".to_string());
            }
        }
    }
    if quote.is_some() || depth != 0 {
        return Some("import path has unbalanced quotes or parentheses".to_string());
    }
    None
}

fn fn_sig_types(tcx: TyCtxt<'_>, did: DefId, span: Span) -> Result<(Vec<Ty<'_>>, Ty<'_>), String> {
    // Only functions have callable signatures: closures, impl blocks, and
    // other items that can end up in the drain order must not reach the
    // `fn_sig` query (it ICEs on them with "unexpected sort of node").
    match tcx.def_kind(did) {
        DefKind::Fn | DefKind::AssocFn => {}
        _ => {
            return Err(format!(
                "`{}` is not a function (no boundary signature)",
                tcx.def_path_str(did)
            ));
        }
    }
    let sig = tcx
        .fn_sig(did)
        .instantiate_identity()
        .skip_normalization()
        .skip_binder();
    let inputs: Vec<Ty<'_>> = sig.inputs().to_vec();
    let output = sig.output();
    let _ = span;
    Ok((inputs, output))
}

/// A function type crossing the boundary as a plain JS function value
/// (fn items, fn pointers, closures).
fn is_fn_ty(t: Ty<'_>) -> bool {
    matches!(t.kind(), ty::FnDef(..) | ty::FnPtr(..) | ty::Closure(..))
}

fn check_boundary_sig(
    tcx: TyCtxt<'_>,
    did: DefId,
    what: &str,
    name: &str,
) -> Result<(Vec<&'static str>, &'static str), String> {
    if !tcx.generics_of(did).own_params.is_empty() {
        return Err(format!("{what} `{name}` must not be generic"));
    }
    let span = tcx.def_span(did);
    let (inputs, output) = fn_sig_types(tcx, did, span)?;
    let mut ts_inputs = vec![];
    for t in inputs {
        // Imports may take JS callbacks (`fn()`, closures): they cross as
        // plain JS functions, no coercion. Only capture-free functions are
        // sound (a capturing closure's env has no JS counterpart).
        // Exports may not take them: called-from-JS functions cannot be
        // given a Rust closure env.
        if what == "import" && is_fn_ty(t) {
            ts_inputs.push("function");
            continue;
        }
        match boundary_ts(tcx, t) {
            Ok(ts) if ts != "void" => ts_inputs.push(ts),
            Ok(_) => {
                return Err(format!(
                    "{what} `{name}` has unsupported parameter type {t}"
                ));
            }
            Err(_) => {
                return Err(format!(
                    "{what} `{name}` has unsupported parameter type {t} \
                     (allowed: f64, i32, u32, bool, String, &str)"
                ));
            }
        }
    }
    let ts_out = match boundary_ts(tcx, output) {
        Ok(ts) => ts,
        Err(_) => {
            return Err(format!(
                "{what} `{name}` has unsupported return type {output} \
                 (allowed: f64, i32, u32, bool, String, &str, ())"
            ));
        }
    };
    Ok((ts_inputs, ts_out))
}

/// Export/import discovery for a single crate. It seeds the crate's entry in
/// the global worklist; no drain or bundle assembly happens here — that is
/// deferred to `join_codegen` of the last crate so that every crate's bundles
/// can be assembled from a single consistent view of all bodies.
fn codegen_js<'tcx>(tcx: TyCtxt<'tcx>, crate_name: &str) -> Result<(), String> {
    // ---- exports: #[js_export] free functions seed this crate's queue ----
    // Every other reachable code is discovered transitively through calls
    // (Ctx::fn_name), so dead code is never emitted.
    let mut exports: Vec<ExportInfo> = Vec::new();
    for did in tcx.hir_crate_items(()).definitions() {
        let hir_id = tcx.local_def_id_to_hir_id(did);
        let has_exp = has_attr(tcx, hir_id, "export");
        if std::env::var("JS_DEBUG_EXPORTS").is_ok() {
            eprintln!(
                "codegen_js: def={:?} kind={:?} export_attr={}",
                did,
                tcx.def_kind(did),
                has_exp
            );
            if has_exp {
                use io::Write;
                std::io::stderr()
                    .write_all(format!("  -> EXPORT FOUND: {}\n", tcx.def_path_str(did)).as_bytes())
                    .ok();
            }
        }
        if !has_exp {
            continue;
        }
        let span = tcx.def_span(did);
        if tcx.def_kind(did) != DefKind::Fn {
            tcx.dcx().span_err(
                span,
                "#[js_export] is only supported on free functions".to_string(),
            );
            continue;
        }
        let js_name = tcx
            .opt_item_name(did)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "export".to_string());
        let inst = ty::Instance::mono(tcx, did.into());
        // A throwaway context is needed to run fn_name and register the
        // export's name into the global cache. fn_name enqueues same-crate
        // instances into this throwaway queue only, so register the persistent
        // seed here to hand them to the drain loop.
        let ctx = Ctx {
            tcx,
            crate_name: crate_name.to_string(),
            active_queue: &mut HashSet::new(),
            active_order: &mut Vec::new(),
            static_names: HashMap::new(),
            static_order: Vec::new(),
            static_bodies: HashMap::new(),
        };
        let internal = crate::emit::fn_name(ctx.tcx, ctx.crate_name.as_str(), &inst);
        crate::g_order()
            .lock()
            .unwrap()
            .entry(crate_name.into())
            .or_default()
            .push(InstKeySer::from_instance(tcx, &inst));
        crate::g_queued()
            .lock()
            .unwrap()
            .entry(crate_name.into())
            .or_default()
            .insert(InstKeySer::from_instance(tcx, &inst));
        // Persist this export seed so the final drain (in the last crate's
        // invocation) can assemble bundles from a complete state.
        let key_str = inst_key_to_str(&InstKeySer::from_instance(tcx, &inst));
        crate::persist_fn_name(&key_str, &internal);
        crate::persist_order(crate_name, &key_str);
        let (ts_inputs, ts_out) = match check_boundary_sig(tcx, did.into(), "export", &js_name) {
            Ok(v) => v,
            Err(e) => {
                tcx.dcx().span_err(span, e);
                continue;
            }
        };
        exports.push(ExportInfo {
            did: did.into(),
            js_name,
            internal,
            ts_inputs,
            ts_out,
        });
    }
    // ---- auto-seed: enqueue the crate's free functions, methods, and
    // ---- concretizable generic functions so the bundle carries everything
    // ---- other crates import via `globalThis.__js_imports`. (cargo codegens
    // ---- crates in separate processes, so cross-crate `fn_name` enqueue
    // ---- never survives a process boundary.)
    for did in tcx.hir_crate_items(()).definitions() {
        use rustc_hir::def::DefKind;
        use rustc_hir::ItemKind;
        // Foreign items (`extern` declarations) have no MIR body — passing
        // one to `mir_hook` ICEs in typeck ("can't type-check body").
        // `#[js_import]` foreign fns are handled by the imports loop below;
        // any other foreign item is out of scope for this backend.
        if tcx.is_foreign_item(did.to_def_id()) {
            continue;
        }
        // Functions explicitly marked `#[js_export]` were already seeded in
        // the export loop above; auto-seed only the remaining functions.
        if has_attr(tcx, tcx.local_def_id_to_hir_id(did), "export") {
            continue;
        }
        if matches!(tcx.def_kind(did), DefKind::Fn) {
            if let rustc_hir::Node::Item(item) = tcx.hir_node(tcx.local_def_id_to_hir_id(did)) {
                if let ItemKind::Fn { sig, .. } = item.kind {
                    if sig.header.asyncness.is_async()
                        || matches!(sig.header.constness, rustc_hir::Constness::Const { .. })
                    {
                        continue;
                    }
                }
            }
            // Generic free functions with `impl Into<T>` param (dgs
            // `point`/`line`/`circle`): MIR params are concrete after
            // impl-trait lowering, so identity args suffice — `mono` would ICE
            // on the type parameter. Generics over bare type params (`T`, `F`)
            // are NOT seeded: identity args leave unresolvable params in the
            // body. Their concrete instantiations are queued (and emitted
            // monomorphized) when call sites are emitted instead.
            if !tcx.generics_of(did).own_params.is_empty() {
                let identity = ty::GenericArgs::identity_for_item(tcx, did);
                let has_bare_param = identity.iter().any(|a| match a.kind() {
                    ty::GenericArgKind::Type(t) => matches!(t.kind(), ty::TyKind::Param(_)),
                    _ => false,
                });
                // DGS exposes generic constructors (point/line/circle/...) whose
                // bodies are emittable from their own generic MIR under the identity
                // args, and whose downstream concrete monomorphizations reuse the
                // DefId-based internal name. Seed them so the upstream bundle
                // publishes the shared constructor.
                if has_bare_param && crate_name != "dgs" {
                    continue;
                }
                let inst = ty::Instance::new_raw(did.into(), identity);
                let _ = crate::emit::fn_name(tcx, crate_name, &inst);
                crate::g_order()
                    .lock()
                    .unwrap()
                    .entry(crate_name.into())
                    .or_default()
                    .push(InstKeySer::from_instance(tcx, &inst));
                crate::g_queued()
                    .lock()
                    .unwrap()
                    .entry(crate_name.into())
                    .or_default()
                    .insert(InstKeySer::from_instance(tcx, &inst));
                continue;
            }
            // Non-generic free function (unchanged behavior).
            let inst = ty::Instance::mono(tcx, did.into());
            let _ = crate::emit::fn_name(tcx, crate_name, &inst);
            crate::g_order()
                .lock()
                .unwrap()
                .entry(crate_name.into())
                .or_default()
                .push(InstKeySer::from_instance(tcx, &inst));
            crate::g_queued()
                .lock()
                .unwrap()
                .entry(crate_name.into())
                .or_default()
                .insert(InstKeySer::from_instance(tcx, &inst));
        } else if matches!(tcx.def_kind(did), DefKind::AssocFn) {
            // Skip trait method declarations without a default body: they
            // have no MIR, and querying it ICEs ("can't type-check body").
            if let rustc_hir::Node::TraitItem(item) = tcx.hir_node(tcx.local_def_id_to_hir_id(did))
            {
                if let rustc_hir::TraitItemKind::Fn(_, rustc_hir::TraitFn::Required(_)) = item.kind
                {
                    continue;
                }
            }
            // Inherent method: identity args include the implicit Self, which
            // is required for the item; `mono` would ICE on it. Named by the
            // method's own name so it resolves as `__js_imports.<method>` at
            // runtime (receiver passed first, matching MIR lowering).
            let inst =
                ty::Instance::new_raw(did.into(), ty::GenericArgs::identity_for_item(tcx, did));
            let _ = crate::emit::fn_name(tcx, crate_name, &inst);
            crate::g_order()
                .lock()
                .unwrap()
                .entry(crate_name.into())
                .or_default()
                .push(InstKeySer::from_instance(tcx, &inst));
            crate::g_queued()
                .lock()
                .unwrap()
                .entry(crate_name.into())
                .or_default()
                .insert(InstKeySer::from_instance(tcx, &inst));
        }
    }

    g_exports()
        .lock()
        .unwrap()
        .entry(crate_name.into())
        .or_default()
        .extend(exports);
    if g_exports()
        .lock()
        .unwrap()
        .get(crate_name)
        .map(|v| !v.is_empty())
        .unwrap_or(false)
    {
        STRICT_CRATES
            .get_or_init(|| Mutex::new(HashSet::new()))
            .lock()
            .unwrap()
            .insert(crate_name.to_string());
    }

    // ---- imports: #[js_import] on foreign fns (or their extern block) ----
    // A bare `#[js::import]` resolves through `globalThis.__js_imports`;
    // `#[js::import = "<path>"]` calls the true JS function at `<path>`
    // directly (e.g. `document.getElementById("x").replaceChildren`).
    // A function-level attribute wins over the enclosing block's.
    for fid in tcx.hir_crate_items(()).foreign_items() {
        let item = tcx.hir_foreign_item(fid);
        let did: DefId = fid.owner_id.def_id.into();
        let parent = tcx.hir_parent_id_iter(item.hir_id()).next().unwrap();
        let attr = match import_attr(tcx, item.hir_id()) {
            Some(a) => Some(a),
            None => match import_attr(tcx, parent) {
                Some(a) => Some(a),
                None => None,
            },
        };
        let span = item.span;
        let path = match attr {
            None => continue,
            Some(Err(e)) => {
                tcx.dcx().span_err(span, e);
                continue;
            }
            Some(Ok(p)) => p,
        };
        if !matches!(item.kind, rustc_hir::ForeignItemKind::Fn(..)) {
            tcx.dcx().span_err(
                span,
                "#[js_import] is only supported on foreign functions".to_string(),
            );
            continue;
        }
        let js_name = item.ident.name.to_string();
        if let Err(e) = check_boundary_sig(tcx, did, "import", &js_name) {
            tcx.dcx().span_err(span, e);
        } else {
            // The map holds the emittable callee: the true path when given,
            // otherwise the shared-imports indirection.
            let callee = path.unwrap_or_else(|| format!("__js_imports.{js_name}"));
            g_import_names().lock().unwrap().insert(did, callee);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Drain + bundle assembly: run from join_codegen of the last crate.
// ---------------------------------------------------------------------------

/// Diagnose a missing MIR body: hard error in export crates, warning in
/// library crates (the stub keeps the bundle building).
fn diag_missing_body<'tcx>(
    tcx: TyCtxt<'tcx>,
    crate_key: &str,
    did: rustc_hir::def_id::DefId,
    msg: String,
) {
    if crate_is_strict(crate_key) {
        tcx.dcx().span_err(tcx.def_span(did), msg);
    } else {
        tcx.dcx().span_warn(tcx.def_span(did), msg);
    }
}

fn drain_crate<'tcx>(tcx: TyCtxt<'tcx>, crate_key: &str) -> HashMap<String, String> {
    let mut bodies = HashMap::new();
    let mut emitted = HashSet::new();
    loop {
        let order: Vec<String> = g_order()
            .lock()
            .unwrap()
            .get(crate_key)
            .map(|v| v.iter().map(|k| k.to_str()).collect())
            .unwrap_or_default();
        let before = emitted.len();
        for key_str in &order {
            if emitted.contains(key_str) {
                continue;
            }
            let inst = match concrete_inst(key_str) {
                // Genuinely monomorphized body (registered by `fn_name`
                // during this invocation).
                Some(inst) => inst,
                None => match InstKeySer::parse(key_str) {
                    Some(k) => k.to_inst(tcx),
                    None => {
                        eprintln!("codegen_js: failed to parse instance key {}", key_str);
                        continue;
                    }
                },
            };
            if bodies.contains_key(key_str) {
                emitted.insert(key_str.clone());
                continue;
            }
            let body = match body_for(tcx, inst) {
                Ok(b) => b,
                Err(e) => {
                    diag_missing_body(tcx, crate_key, inst.def_id(), e);
                    bodies.insert(
                        key_str.clone(),
                        "function __stub() { throw __unreachable(); }".to_string(),
                    );
                    emitted.insert(key_str.clone());
                    continue;
                }
            };
            let mut ctx = Ctx {
                tcx,
                crate_name: crate_key.to_string(),
                active_queue: &mut HashSet::new(),
                active_order: &mut Vec::new(),
                static_names: HashMap::new(),
                static_order: Vec::new(),
                static_bodies: HashMap::new(),
            };
            let code = Emitter::emit_fn(&mut ctx, &inst, &body);
            bodies.insert(key_str.clone(), code);
            emitted.insert(key_str.clone());
        }
        if emitted.len() == before {
            break;
        }
    }
    // Post-drain: emit any named instances not yet emitted — e.g. closures
    // discovered mid-drain (their fn_name calls ran while their bodies
    // weren't yet in the drain order). Loop until no new bodies appear.
    loop {
        let remaining: Vec<String> = {
            let fn_names = crate::g_fn_names().lock().unwrap();
            let mut v = Vec::new();
            for (key, _val) in fn_names.iter() {
                if bodies.contains_key(&key.to_str()) {
                    continue;
                }
                v.push(key.to_str().to_string());
            }
            v
        };
        if remaining.is_empty() {
            break;
        }
        let before = bodies.len();
        for key_str in remaining {
            if emitted.contains(&key_str) {
                continue;
            }
            // Only same-crate instances belong in this crate's bundle; the post-drain loop
            // exists to re-emit closures that were discovered mid-drain. Cross-crate instances
            // are emitted in their own crate's bundle and resolved via
            // `globalThis.__js_imports` at runtime.
            let key = match InstKeySer::parse(&key_str) {
                Some(k) if k.local => k,
                Some(k) => {
                    // Routine: cross-crate instances are emitted in their own
                    // crate's bundle (only noisy under JS_DEBUG).
                    if std::env::var("JS_DEBUG").is_ok() {
                        eprintln!("drain_crate: skipping non-local instance in post-drain loop: key={} path={} (emitted in crate {})", key_str, tcx.def_path_str(k.def_id()), tcx.crate_name(k.def_id().krate));
                    }
                    continue;
                }
                None => {
                    eprintln!("drain_crate: failed to parse instance key {}", key_str);
                    continue;
                }
            };
            let inst = concrete_inst(&key_str).unwrap_or_else(|| key.to_inst(tcx));
            let body = match body_for(tcx, inst) {
                Ok(b) => b,
                Err(e) => {
                    diag_missing_body(tcx, crate_key, inst.def_id(), e);
                    bodies.insert(
                        key_str.clone(),
                        "function __stub() { throw __unreachable(); }".to_string(),
                    );
                    emitted.insert(key_str.clone());
                    continue;
                }
            };
            let mut ctx = Ctx {
                tcx,
                crate_name: crate_key.to_string(),
                active_queue: &mut HashSet::new(),
                active_order: &mut Vec::new(),
                static_names: HashMap::new(),
                static_order: Vec::new(),
                static_bodies: HashMap::new(),
            };
            let code = Emitter::emit_fn(&mut ctx, &inst, &body);
            bodies.insert(key_str.clone(), code);
            emitted.insert(key_str.clone());
        }
        if bodies.len() == before {
            break;
        }
    }
    persist_drained(crate_key);
    bodies
}

/// Build the export-wrapper signature of an instance (its item name plus boundary
/// TS types). Used for `#[js_export]` wrappers and for auto-exporting every drained
/// function so other crates can import it via `globalThis.__js_imports.<name>`.
fn export_sig_of<'tcx>(
    tcx: TyCtxt<'tcx>,
    inst_key: &str,
) -> Result<(String, Vec<&'static str>, &'static str), String> {
    let k = InstKeySer::parse(inst_key)
        .ok_or_else(|| format!("export_sig_of: failed to parse instance key {}", inst_key))?;
    let inst = k.to_inst(tcx);
    let did = inst.def_id();
    let span = tcx.def_span(did);
    let (inputs, output) = fn_sig_types(tcx, did, span)?;
    let js_name = tcx
        .opt_item_name(did)
        .map(|s| s.as_str().to_string())
        .unwrap_or("export".to_string());
    let mut ts_inputs = Vec::new();
    for t in inputs {
        match boundary_ts(tcx, t) {
            Ok(ts) if ts != "void" => ts_inputs.push(ts),
            Ok(_) => {
                return Err(format!(
                    "function `{js_name}` has unsupported parameter type {t} \
                     (allowed: f64, i32, u32, bool, String, &str)"
                ));
            }
            Err(_) => {
                // Parameter crossing the boundary as an untyped cell
                // (ADT/struct passed by identity).
                ts_inputs.push("any");
            }
        }
    }
    let ts_out = match boundary_ts(tcx, output) {
        Ok(ts) => ts,
        Err(_) => "any", // non-boundary return (e.g. a local ADT)
    };
    Ok((js_name, ts_inputs, ts_out))
}

fn assemble_js<'tcx>(
    tcx: TyCtxt<'tcx>,
    crate_key: &str,
    order: &[String],
    bodies: &HashMap<String, String>,
    _exports: &[ExportInfo],
) -> String {
    let mut js = String::new();
    js.push_str("// @ts-nocheck\n");
    js.push_str(&format!(
        "// Generated by rustc_codegen_js from crate `{crate_key}`.\n"
    ));
    js.push_str("const __js_imports = globalThis.__js_imports ?? {};\n");
    js.push_str(RUNTIME_JS);
    js.push_str("\n// ---- functions ----\n");
    for inst in order.iter() {
        if let Some(code) = bodies.get(inst) {
            js.push_str(code);
            js.push('\n');
        }
    }
    js.push_str("\n// ---- exports ----\n");
    // Every drained function is registered on the shared
    // `globalThis.__js_imports` map so other crates can resolve calls at
    // runtime (bundles are loaded in dependency order, so the map persists).
    // Several keys can share an item name (renumbered instances, closures,
    // non-function leftovers in the order list): dedupe the export alias so
    // the module never contains duplicate `export` names (a SyntaxError).
    let mut used_aliases = HashSet::new();
    for inst in order.iter() {
        let k = crate::InstKeySer::parse(inst).unwrap_or_else(|| {
            eprintln!("codegen_js: failed to parse inst key {inst}");
            crate::InstKeySer {
                krate: 0,
                idx: 0,
                local: true,
                local_idx: Some(0),
                args_fp: String::new(),
            }
        });
        let internal = crate::g_fn_names()
            .lock()
            .unwrap()
            .get(&k)
            .cloned()
            .unwrap_or_else(|| "fn".to_string());
        let inst = k.to_inst(tcx);
        let is_closure = matches!(tcx.def_kind(inst.def_id()), DefKind::Closure);
        let js_name: String = if is_closure {
            // Closures are self-contained module-scoped functions (already
            // emitted with their body above); just re-export them plainly.
            "fn".to_string()
        } else {
            tcx.opt_item_name(inst.def_id())
                .map(|s| s.as_str().to_string())
                .unwrap_or_else(|| "fn".to_string())
        };
        // Dedupe the export alias: the first use keeps the plain name,
        // later collisions get `_1`, `_2`, ... suffixes.
        let mut js_name = js_name;
        if !used_aliases.insert(js_name.clone()) {
            let mut i = 1;
            loop {
                let cand = format!("{js_name}_{i}");
                if used_aliases.insert(cand.clone()) {
                    js_name = cand;
                    break;
                }
                i += 1;
            }
        }
        // NOTE: cross-crate calls always use the internal (path-qualified)
        // name above; the alias is best-effort back-compat only.
        if internal != "fn" {
            js.push_str(&format!(
                "(globalThis.__js_imports ??= {{}}).{internal} = {internal};\n"
            ));
            if js_name != internal && js_name != "fn" {
                js.push_str(&format!(
                    "(globalThis.__js_imports ??= {{}}).{js_name} = {internal};\n"
                ));
            }
        }
        if !is_closure {
            if let Ok((_, ts_inputs, ts_out)) = export_sig_of(tcx, &k.to_str()) {
                // Boundary function: a type-coercing wrapper gives the
                // exported name its boundary surface. The wrapper uses a
                // mangled name so it does not shadow the module-scoped
                // function definition, which would otherwise be infinitely
                // recursive.
                let wrapper = format!("_js_{js_name}");
                let params: Vec<String> = (0..ts_inputs.len()).map(|i| format!("a{i}")).collect();
                let in_args: Vec<String> = params
                    .iter()
                    .zip(ts_inputs.iter())
                    .map(|(p, t)| match *t {
                        "string" => t_wrap(p),
                        _ => p.to_string(),
                    })
                    .collect();
                let call = format!("{internal}({})", in_args.join(","));
                let ret = match ts_out {
                    // Void: still perform the call for its effects.
                    "void" => format!("{call};"),
                    "string" => format!("return {call}[0]"),
                    _ => format!("return {call}"),
                };
                js.push_str(&format!(
                    "export function {wrapper}({}) {{{ret}}}\n",
                    params.join(",")
                ));
                js.push_str(&format!("export {{ {wrapper} as {js_name} }};\n"));
            } else {
                // Non-boundary function: plain re-export (no coercion available).
                js.push_str(&format!("export {{ {internal} as {js_name} }};\n"));
            }
        } else {
            js.push_str(&format!("export {{ {internal} as {js_name} }};\n"));
        }
    }
    js
}

fn assemble_dts<'tcx>(tcx: TyCtxt<'tcx>, crate_key: &str, order: &[String]) -> String {
    let mut dts = String::new();
    dts.push_str(&format!(
        "// Boundary of crate `{crate_key}` (generated).\n"
    ));
    for inst in order.iter() {
        let k = crate::InstKeySer::parse(inst).unwrap_or_else(|| {
            eprintln!("codegen_js: failed to parse inst key {inst}");
            crate::InstKeySer {
                krate: 0,
                idx: 0,
                local: true,
                local_idx: Some(0),
                args_fp: String::new(),
            }
        });
        let inst = k.to_inst(tcx);
        // Closures are self-contained module-scoped functions with their own
        // names; they have no boundary signature, so skip them in the TS stubs.
        if matches!(tcx.def_kind(inst.def_id()), DefKind::Closure) {
            continue;
        }
        let (js_name, ts_inputs, ts_out) = match export_sig_of(tcx, &k.to_str()) {
            Ok(v) => v,
            Err(e) => {
                use io::Write;
                std::io::stderr()
                    .write_all(format!("codegen_js: {e}\n").as_bytes())
                    .ok();
                continue;
            }
        };
        let params: Vec<String> = ts_inputs
            .iter()
            .enumerate()
            .map(|(i, t)| format!("a{i}: {t}"))
            .collect();
        dts.push_str(&format!(
            "export declare function {}({}): {};\n",
            js_name,
            params.join(", "),
            ts_out
        ));
    }
    dts
}

fn write_all_bundles<'tcx>(tcx: TyCtxt<'tcx>, sess: &Session) {
    // Cargo codegens crates in separate processes, and `codegen_js` auto-queues
    // every function so each crate has a self-contained bundle; cross-crate
    // calls resolve via `globalThis.__js_imports` at runtime.
    let current = tcx
        .crate_name(rustc_span::def_id::LOCAL_CRATE)
        .to_string()
        .replace('-', "_");
    let state = read_state();
    let out_dir = state
        .out_dirs
        .get(&current)
        .cloned()
        .unwrap_or_else(|| std::env::current_dir().unwrap().join("js"));
    let bodies = drain_crate(tcx, &current);
    // Assemble the order AFTER draining: the drain discovers new instances
    // (closures, transitive callees) and persists them to the state file, so
    // a pre-drain snapshot would silently drop their bodies from the bundle.
    // Union, in priority order: persisted file order, in-memory worklist
    // (auto-seeds never hit the file), then any remaining emitted bodies
    // (sorted — HashMap order is nondeterministic). Duplicates collapse.
    let state = read_state();
    let mut seen = HashSet::new();
    let mut order: Vec<String> = Vec::new();
    for key in state.order.get(&current).cloned().unwrap_or_default() {
        if seen.insert(key.clone()) {
            order.push(key);
        }
    }
    if let Some(mem) = g_order().lock().unwrap().get(&current) {
        for k in mem.iter() {
            if seen.insert(k.to_str().to_string()) {
                order.push(k.to_str().to_string());
            }
        }
    }
    let mut extra: Vec<String> = bodies
        .keys()
        .filter(|k| !seen.contains(*k))
        .cloned()
        .collect();
    extra.sort();
    order.extend(extra);
    assemble_and_write(tcx, &current, &out_dir, &order, &bodies, sess);
    // State from earlier invocations is no longer needed: each crate now emits
    // its own self-contained bundle in its own process, and rebuilds are
    // always fresh.
    let _ = std::fs::remove_file(STATE_FILE);
    let _ = sess.dcx().note(format!(
        "codegen_js: bundle written: {}/{}.mjs + {}/{}.d.ts",
        out_dir.display(),
        current,
        out_dir.display(),
        current
    ));
}

fn assemble_and_write<'tcx>(
    tcx: TyCtxt<'tcx>,
    crate_key: &str,
    out_dir: &Path,
    order: &[String],
    bodies: &HashMap<String, String>,
    sess: &Session,
) {
    let js_dir = std::env::var("JS_BUNDLES_DIR").ok();
    let out_dir = js_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| out_dir.to_path_buf());
    fs::create_dir_all(&out_dir).unwrap_or_else(|e| {
        sess.dcx().fatal(format!(
            "codegen_js: failed to create dir {:?}: {e}",
            out_dir
        ));
    });
    let exports = Vec::new();
    let js = assemble_js(tcx, crate_key, order, bodies, &exports);
    let dts = assemble_dts(tcx, crate_key, order);
    let js_path = out_dir.join(format!("{crate_key}.mjs"));
    let dts_path = out_dir.join(format!("{crate_key}.d.ts"));
    if let Err(e) = fs::write(&js_path, &js) {
        sess.dcx().fatal(format!(
            "codegen_js: failed to write {}: {e}",
            js_path.display()
        ));
    }
    if let Err(e) = fs::write(&dts_path, &dts) {
        sess.dcx().fatal(format!(
            "codegen_js: failed to write {}: {e}",
            dts_path.display()
        ));
    }
    sess.dcx().note(format!(
        "codegen_js: bundle written: {} + {}",
        js_path.display(),
        dts_path.display()
    ));
}
/// Entry point loaded via `-Zcodegen-backend=<path/to/libcodegen_js.so>`.
#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(JsCodegenBackend)
}

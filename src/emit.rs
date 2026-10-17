//! MIR → JavaScript emission (§7 place resolution, §10 output shape).
//!
//! Each monomorphized instance becomes one JS function. Control flow is
//! structured first: `if`/`else if`/`else` chains for two-arm branches, a
//! `switch` on the discriminant for genuine `match`es, and `L: while (true)`
//! loops (with `break`/`continue`) for `loop`/`while`/`for`. Bodies that do
//! not structuralize fall back to a `while(true)` + `switch(bb)` state
//! machine. Values follow the representations in `tyutil`.

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use rustc_abi::{FieldIdx, VariantIdx};
use rustc_hir::def_id::DefId;
use rustc_index::IndexVec;
use rustc_middle::mir;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::{Span, Spanned};
use std::sync::Mutex;

use crate::constval::{self, ConstCx};
use crate::recognize::{
    self, ArgNewKind, Callee, FloatArithOp, FloatOp, Intrinsic, PanicKind, Shim,
};
use crate::tyutil::{
    self, closure_upvar_ty, def_path, discriminant_value, mono_env, repr_of, subst_args, subst_ty,
    BigKind, NumKind, Repr,
};
use crate::InstKeySer;

// ---------------------------------------------------------------------------
// Shared backend context
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct ExportInfo {
    pub did: DefId,
    pub js_name: String,
    pub internal: String,
    pub ts_inputs: Vec<&'static str>,
    pub ts_out: &'static str,
}

/// A `SwitchInt` value rendered as a JS literal.
fn switch_lit_of<'tcx>(tcx: rustc_middle::ty::TyCtxt<'tcx>, v: u128, ty: Ty<'tcx>) -> String {
    match repr_of(tcx, ty) {
        Repr::Big(_) => format!("{v}n"),
        Repr::Bool => (v != 0).to_string(),
        _ => v.to_string(),
    }
}

/// Bit width of a `<= 32`-bit integer kind.
fn num_bits(k: NumKind) -> u32 {
    match k {
        NumKind::I8 | NumKind::U8 => 8,
        NumKind::I16 | NumKind::U16 => 16,
        NumKind::I32 | NumKind::U32 | NumKind::Isize | NumKind::Usize | NumKind::Char => 32,
    }
}

fn num_signed(k: NumKind) -> bool {
    matches!(
        k,
        NumKind::I8 | NumKind::I16 | NumKind::I32 | NumKind::Isize
    )
}

/// Smallest value of a 64/128-bit integer kind, as a BigInt literal.
fn big_int_lo(k: BigKind) -> String {
    match k {
        BigKind::I64 => "-9223372036854775808n".to_string(),
        BigKind::U64 => "0n".to_string(),
        BigKind::I128 => "-170141183460469231731687303715884105728n".to_string(),
        BigKind::U128 => "0n".to_string(),
    }
}

/// Largest value of a 64/128-bit integer kind, as a BigInt literal.
fn big_int_hi(k: BigKind) -> String {
    match k {
        BigKind::I64 => "9223372036854775807n".to_string(),
        BigKind::U64 => "18446744073709551615n".to_string(),
        BigKind::I128 => "170141183460469231731687303715884105727n".to_string(),
        BigKind::U128 => "340282366920938463463374607431768211455n".to_string(),
    }
}

pub struct Ctx<'a, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    /// Crate key for this context (set by the drain loop).
    pub crate_name: String,
    /// The queue being drained right now: used by `fn_name` to enqueue
    /// instances without holding the global maps' locks during emission.
    pub active_queue: &'a mut HashSet<crate::InstKey>,
    /// The order being drained right now; new instances are appended here
    /// when `fn_name` assigns them a name.
    pub active_order: &'a mut Vec<crate::InstKey>,
    /// Static names, kept for future static support.
    pub static_names: HashMap<DefId, String>,
    pub static_order: Vec<DefId>,
    pub static_bodies: HashMap<DefId, String>,
}

impl<'a, 'tcx> Ctx<'a, 'tcx> {
    pub fn diag(&mut self, span: Span, msg: String) {
        self.tcx.dcx().span_err(span, msg);
    }

    pub fn static_name(&mut self, did: DefId) -> String {
        if let Some(n) = self.static_names.get(&did) {
            return n.clone();
        }
        let base: String = format!(
            "s_{}",
            self.tcx
                .opt_item_name(did)
                .map(|s| s.as_str().to_string())
                .unwrap_or_else(|| "static".to_string())
        );
        let mut name = base.clone();
        let mut i = 0;
        let crate_key = self.crate_name.clone();
        loop {
            let mut taken = crate::g_name_taken().lock().unwrap();
            let crate_taken = taken.entry(crate_key.clone()).or_default();
            if !crate_taken.contains(&name) {
                crate_taken.insert(name.clone());
                drop(taken);
                break;
            }
            i += 1;
            name = format!("{base}_{i}");
        }
        self.static_names.insert(did, name.clone());
        self.static_order.push(did);
        name
    }
}

/// Load the MIR body for an instance: pre-optimization bodies for local defs
/// (read from rustc's cached `mir_drops_elaborated_and_const_checked` query),
/// optimized MIR for upstream defs (only reached through already-resolved
/// calls). Calls are still calls in the pre-optimization MIR — the §3
/// invariant — because the body is never optimized before we read it.
pub fn body_for<'tcx>(
    tcx: TyCtxt<'tcx>,
    instance: ty::Instance<'tcx>,
) -> Result<mir::Body<'tcx>, String> {
    let did = instance.def_id();
    if let Some(local) = did.as_local() {
        // Trait method declarations without a default body have no MIR;
        // querying it would ICE ("can't type-check body"), so report it as
        // missing and let the caller emit a stub.
        if let rustc_hir::Node::TraitItem(item) = tcx.hir_node(tcx.local_def_id_to_hir_id(local)) {
            if let rustc_hir::TraitItemKind::Fn(_, rustc_hir::TraitFn::Required(_)) = item.kind {
                return Err(format!(
                    "`{}` is a trait declaration without a default body (no MIR)",
                    tcx.def_path_str(did)
                ));
            }
        }
        // Read from the Hook A snapshot table first: it holds arena-allocated
        // clones valid for the whole compilation, so we never re-steal the
        // cached MIR (which would panic if rustc or a prior drain cycle
        // already consumed the Steal).
        if let Some(body) = crate::snapshot_for(tcx, local) {
            let body: mir::Body<'tcx> =
                unsafe { std::mem::transmute::<mir::Body<'static>, mir::Body<'tcx>>(body) };
            return Ok(body);
        }
        // First computation: take the pre-optimization body and snapshot it.
        let body: mir::Body<'tcx> = tcx.mir_drops_elaborated_and_const_checked(local).steal();
        crate::PRE_OPT_MIR_SNAP
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .insert(local, unsafe {
                std::mem::transmute::<mir::Body<'tcx>, mir::Body<'static>>(body.clone())
            });
        Ok(body)
    } else {
        // Upstream body (rolling checksum-free path: only reached through
        // already-resolved calls). Ask for MIR only when the crate metadata
        // actually carries it: the query panics otherwise (`overflow_panic`
        // helpers are not serialized), which would surface as an ICE.
        if !tcx.is_mir_available(did) {
            return Err(format!(
                "`{}` has no MIR in the crate metadata (it must be a shim)",
                tcx.def_path_str(did)
            ));
        }
        Ok(tcx.optimized_mir(did).clone())
    }
}

// ---------------------------------------------------------------------------

/// Assign (or look up) the JS name for an instance; enqueue for emission.
/// Names are unique within a crate (derived from `item_name`); the global
/// `g_fn_names` cache is keyed by instance.

fn path_of<'tcx>(inst: &ty::Instance<'tcx>) -> String {
    let inst_id = inst.def_id();
    if inst_id.is_local() {
        format!("local{}", inst_id.index.index())
    } else {
        format!("crate{:?}_idx{}", inst_id.krate, inst_id.index.index())
    }
}

/// Resolve the JS name for an instance during seeding (no body emitted yet):
/// unique within a crate, persisted to the shared state file, but not
/// tracked in an emission queue.
pub fn fn_name<'tcx>(tcx: TyCtxt<'tcx>, crate_name: &str, inst: &ty::Instance<'tcx>) -> String {
    fn_name_crate_inner(tcx, crate_name, inst, None, None).0
}

/// Emit-time name resolution: tracks the instance in this Emitter's active
/// queue/order so the drain loop observes emission order.
pub fn fn_name_crate<'tcx>(em: &mut Emitter<'tcx>, inst: &ty::Instance<'tcx>) -> (String, String) {
    fn_name_crate_inner(
        em.tcx,
        &em.crate_name,
        inst,
        Some(&mut em.active_queue),
        Some(&mut em.active_order),
    )
}

fn fn_name_crate_inner<'tcx>(
    tcx: TyCtxt<'tcx>,
    crate_name: &str,
    inst: &ty::Instance<'tcx>,
    active_queue: Option<&mut HashSet<crate::InstKeySer>>,
    mut active_order: Option<&mut Vec<crate::InstKeySer>>,
) -> (String, String) {
    let inst_crate = tcx
        .crate_name(inst.def_id().krate)
        .to_string()
        .replace('-', "_");
    let inst_key = InstKeySer::from_instance(tcx, inst);
    if let Some(n) = crate::g_fn_names().lock().unwrap().get(&inst_key) {
        return (n.clone(), inst_crate.clone());
    }
    let is_closure = matches!(
        tcx.def_kind(inst.def_id()),
        rustc_hir::def::DefKind::Closure
    );
    if std::env::var("JS_DEBUG_FN").is_ok() {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/js_debug.log")
            .unwrap();
        f.write_all(
            format!(
                "fn_name_crate: {} crate={} ctx={}\n",
                tcx.def_path_str(inst.def_id()),
                inst_crate,
                crate_name
            )
            .as_bytes(),
        )
        .ok();
    }
    let name = if is_closure {
        // Closures have no item name; synthesize a stable unique name from the
        // enclosing item and the closure's id so different closure instances
        // never collide. Nested closures (a closure inside another closure) are
        // named via the parent's synthesized name rather than item_name, since
        // the parent DefId is itself a closure.
        let parent = tcx.parent(inst.def_id());
        let parent_name = if matches!(tcx.def_kind(parent), rustc_hir::def::DefKind::Closure) {
            let parent_key = crate::InstKeySer::from_def_id(parent);
            crate::g_fn_names()
                .lock()
                .unwrap()
                .get(&parent_key)
                .cloned()
                .unwrap_or_else(|| "fn".to_string())
        } else {
            // Path-qualified (not bare item name): same-named parents in
            // different modules must yield different closure names.
            js_path_name(tcx, parent)
        };
        let closure_idx = inst.def_id().index.as_u32();
        format!("{}__closure_{}", parent_name, closure_idx)
    } else {
        assign_name(tcx, crate_name.to_string(), inst)
    };
    // Register the name (closure or not) in the taken set to avoid collisions.
    crate::g_fn_names()
        .lock()
        .unwrap()
        .insert(inst_key.clone(), name.clone());
    // Remember the concrete instance so the drain loop can emit a genuinely
    // monomorphized body (the serializable key alone cannot rebuild args).
    crate::remember_concrete_inst(&inst_key.to_str(), inst);
    // Persist the name, order, and dependency edge to the shared state file so
    // that the final drain (in the last-crate invocation) can assemble all
    // bundles from a complete, cross-invocation view.
    crate::persist_fn_name(&crate::inst_key_to_str(&inst_key), &name);
    crate::persist_order(&inst_crate, &crate::inst_key_to_str(&inst_key));
    if inst_crate != crate_name {
        crate::persist_requires(crate_name, &inst_crate);
    }
    if inst_crate == crate_name {
        if let Some(q) = active_queue {
            if q.insert(inst_key.clone()) {
                active_order.as_mut().unwrap().push(inst_key);
            }
        }
    } else {
        // Cross-crate call target: enqueue it into the *target* crate's
        // worklist (plus an edge so the drain order knows the dependency).
        crate::g_queued()
            .lock()
            .unwrap()
            .entry(inst_crate.clone())
            .or_default()
            .insert(inst_key.clone());
        crate::g_order()
            .lock()
            .unwrap()
            .entry(inst_crate.clone())
            .or_default()
            .push(inst_key);
        crate::g_requires()
            .lock()
            .unwrap()
            .entry(crate_name.to_string().clone())
            .or_default()
            .insert(inst_crate.clone());
    }
    (name, inst_crate)
}

/// Determine the (deterministic) JS name for an instance from its readable
/// item name, renumbering on collision within the crate.
fn assign_name<'tcx>(tcx: TyCtxt<'tcx>, crate_key: String, inst: &ty::Instance<'tcx>) -> String {
    let base = js_path_name(tcx, inst.def_id());
    let mut name = base.clone();
    let mut i = 0;
    loop {
        let mut taken = crate::g_name_taken().lock().unwrap();
        let crate_taken = taken.entry(crate_key.clone()).or_default();
        if !crate_taken.contains(&name) {
            crate_taken.insert(name.clone());
            drop(taken);
            break;
        }
        i += 1;
        name = format!("{base}_{i}");
    }
    name
}

/// Deterministic JS-safe name for a DefId: `crate$item$defindex`.
/// Unlike `def_path_str` (which renders inherent-impl methods differently
/// for local vs upstream items, e.g. `viewport::Viewport::new` vs
/// `dgs::Viewport::new`), the crate name + item name + DefIndex triple is
/// identical in every compilation that loads the same crate metadata, so
/// the defining crate and every importing crate compute the same internal
/// name and `__js_imports` linkage lines up. `$` cannot appear in a Rust
/// identifier, so joins are collision-free; residual collisions still fall
/// back to numeric suffixes in `assign_name`.
pub fn js_path_name(tcx: TyCtxt<'_>, did: DefId) -> String {
    let krate = tcx.crate_name(did.krate).to_string().replace('-', "_");
    let item = tcx
        .opt_item_name(did)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "anon".to_string());
    let item = item
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();
    sanitize_js_name(&format!("{krate}${item}${}", did.index.as_u32()))
}

/// Map a Rust item name to a legal JS binding name. Bundles are ES modules
/// (always strict mode), where `eval`/`arguments` cannot be bindings and
/// reserved words cannot be identifiers at all. Suffixed so the mapping is
/// deterministic across processes (cross-crate imports rely on it).
fn sanitize_js_name(base: &str) -> String {
    match base {
        // Strict-mode-forbidden bindings.
        "eval" | "arguments" => format!("{base}_fn"),
        // Reserved words that are legal Rust identifiers.
        "break" | "case" | "catch" | "class" | "const" | "continue" | "debugger" | "default"
        | "delete" | "do" | "else" | "export" | "extends" | "finally" | "for" | "function"
        | "if" | "implements" | "import" | "in" | "instanceof" | "interface" | "new"
        | "package" | "private" | "protected" | "public" | "static" | "super" | "switch"
        | "this" | "throw" | "try" | "typeof" | "var" | "void" | "while" | "with" | "yield"
        | "let" | "null" | "true" | "false" => format!("{base}_fn"),
        _ => base.to_string(),
    }
}
// Per-function emitter
// ---------------------------------------------------------------------------

pub struct Emitter<'tcx> {
    tcx: TyCtxt<'tcx>,
    instance: crate::InstKey,
    args: ty::GenericArgsRef<'tcx>,
    body: mir::Body<'tcx>,
    span: Span,
    cur_span: Span,
    local_tys: Vec<Ty<'tcx>>,
    out: String,
    poisoned: bool,
    skipped: std::collections::HashSet<mir::Local>,
    /// Control-flow structure of the current body (structured emission).
    structure: Option<crate::structure::Structure>,
    /// Loop label counter for structured emission.
    loop_seq: usize,
    /// Suppress diagnostics while probing whether structured emission works.
    quiet: bool,
    /// Last suppressed error (structured emission tries before it reports).
    last_err: Option<String>,
    /// Mutable-reference bindings: a local assigned via `&mut place` holds the
    /// *value* of that place, so a later `*local = x` must actually store to
    /// the tracked location. Maps local -> the JS location expression.
    ref_locals: std::collections::HashMap<mir::Local, String>,
    /// Name-queue for the function currently being emitted: fn_name_crate
    /// enqueues here so the drain loop knows the instance order.
    active_queue: std::collections::HashSet<crate::InstKeySer>,
    /// Instance order for the function currently being emitted.
    active_order: std::vec::Vec<crate::InstKeySer>,
    /// Crate key of the currently-emitted function.
    crate_name: String,
}

enum CallTarget<'tcx> {
    Direct {
        name: String,
    },
    /// Closure called with already-spread args: `name(env, args...)`.
    ClosureDirect {
        name: String,
        env: String,
    },
    /// Closure called through `Fn`/`FnMut`/`FnOnce` trait method:
    /// `name(call_arg0, ...call_arg1_tuple)`.
    ClosureTrait {
        name: String,
    },
    CtorValue,
    Dynamic {
        expr: String,
    },
    Shim {
        op: Shim,
        args: ty::GenericArgsRef<'tcx>,
    },
    Intrinsic {
        op: Intrinsic,
        args: ty::GenericArgsRef<'tcx>,
    },
    JsImport {
        /// Emittable callee: the shared-imports indirection
        /// (`__js_imports.<internal>`) for cross-crate targets, or the true
        /// JS path from `#[js::import = "<path>"]`.
        name: String,
        args: ty::GenericArgsRef<'tcx>,
        /// `true` when `name` is an explicit user path containing `$N`
        /// splice placeholders (emitted as a full expression, no `(args)`
        /// appended). Internal `__js_imports.*` names are never templates,
        /// even though they contain `$`-defindex suffixes.
        template: bool,
    },
    /// Peered inline JS (`#[js::inline]` stub): emitted from the call-site
    /// template + splice operands, never via a named function.
    JsInline {
        did: DefId,
    },
    Panic(PanicKind),
    FmtNew,
    ArgNew(ArgNewKind),
    Unsupported(&'static str),
    /// Negated inner call (`!=` via `!eq`).
    Not(Box<CallTarget<'tcx>>),
}

impl<'tcx> Emitter<'tcx> {
    pub fn emit_fn(
        ctx: &mut Ctx<'_, 'tcx>,
        inst: &ty::Instance<'tcx>,
        body: &mir::Body<'tcx>,
    ) -> String {
        let tcx = ctx.tcx;
        let args = inst.args;
        let span = tcx.def_span(inst.def_id());
        let local_tys = body
            .local_decls
            .iter()
            .map(|d| subst_ty(tcx, args, d.ty))
            .collect();
        let inst_key = InstKeySer::from_instance(tcx, inst);
        let name = crate::g_fn_names()
            .lock()
            .unwrap()
            .get(&inst_key)
            .cloned()
            .unwrap_or_default();
        let mut em = Emitter {
            tcx,
            instance: InstKeySer::from_instance(tcx, inst),
            args,
            body: body.clone(),
            span,
            cur_span: span,
            local_tys,
            out: String::new(),
            poisoned: false,
            skipped: std::collections::HashSet::new(),
            structure: None,
            loop_seq: 0,
            quiet: false,
            last_err: None,
            ref_locals: std::collections::HashMap::new(),
            crate_name: ctx.crate_name.to_string(),
            active_queue: std::collections::HashSet::new(),
            active_order: std::vec::Vec::new(),
        };
        if let Some(desc) = em.structural_debug_desc() {
            let params: Vec<String> = (1..=body.arg_count).map(|i| format!("l{i}")).collect();
            return format!(
                "function {name}({}){{return __debug_val({desc},l1);}}\n",
                params.join(",")
            );
        }
        em.find_peer_only_calls();
        em.emit_body(body);
        if em.poisoned && !crate::crate_is_strict(em.crate_name.as_str()) {
            // Library crate without exports: replace the partial output with
            // a throwing stub so the bundle still builds. The warning above
            // names the gap; the stub names it again at runtime.
            let msg = em
                .last_err
                .clone()
                .unwrap_or_else(|| "unsupported construct".to_string());
            let inst_path = tcx.def_path_str(inst.def_id());
            return format!(
                "function {name}(){{\nthrow new Error({:?});\n}}\n",
                format!("codegen_js: {msg} (in {inst_path})")
            );
        }
        if std::env::var("JS_TIMING").is_ok() {
            eprintln!("[js-timing] emit_fn DONE {}", path_of(inst));
        }
        if std::env::var("JS_DEBUG").is_ok() {
            use std::io::Write;
            std::io::stderr()
                .write_all(format!("emit_fn: END {}\n", path_of(inst)).as_bytes())
                .ok();
        }
        std::mem::replace(&mut em.out, String::new())
    }

    /// Type descriptor for a `Debug::fmt` body we can replace wholesale.
    ///
    /// `#[derive(Debug)]` (and hand-written impls) lower to
    /// `Formatter::debug_tuple/struct/...` calls, which this backend has no
    /// runtime for. The structural debug the descriptors already describe is
    /// exactly what those calls produce for the types we support, so emit it
    /// directly. Only local ADTs: std's own `Debug` impls are reached through
    /// descriptors, never as a body.
    fn structural_debug_desc(&mut self) -> Option<String> {
        let did = self.instance.def_id();
        if did.as_local().is_none() {
            return None;
        }
        let path = def_path(self.tcx, did);
        if !tyutil::ends_seg(&path, &["Debug", "fmt"]) {
            return None;
        }
        let self_ty = self.subst(self.instance.args(self.tcx).type_at(0));
        let local_adt = matches!(self_ty.kind(), ty::Adt(def, _) if def.did().as_local().is_some());
        if !local_adt {
            return None;
        }
        self.fmt_desc(self_ty).ok()
    }

    fn err(&mut self, msg: String) {
        self.err_at(self.cur_span, msg);
    }

    fn err_at(&mut self, span: Span, msg: String) {
        if self.quiet {
            self.poisoned = true;
            self.last_err = Some(msg);
            return;
        }
        if !self.poisoned {
            self.poisoned = true;
        }
        let inst = self.tcx.def_path_str(self.instance.def_id());
        // Export crates (program entry points) fail hard so broken exports
        // surface at build time. In library crates without exports, an
        // un-emittable body (e.g. a serde derive helper that is never called)
        // must not block the whole bundle: warn, and the caller (`emit_fn`)
        // replaces the partial output with a throwing stub.
        if crate::crate_is_strict(&self.crate_name) {
            self.tcx
                .dcx()
                .span_err(span, format!("codegen_js: {msg} (in {inst})"));
        } else {
            self.tcx
                .dcx()
                .span_warn(span, format!("codegen_js: {msg} (in {inst})"));
        }
    }

    fn ccx(&self) -> ConstCx<'tcx> {
        ConstCx::new(self.tcx, self.args, self.cur_span)
    }

    fn subst(&self, ty: Ty<'tcx>) -> Ty<'tcx> {
        subst_ty(self.tcx, self.args, ty)
    }

    // ---------------- locals / places ----------------

    /// The JS expression for a local slot. Locals live in one per-frame
    /// object (`M[i]`), so a declaration list of mostly-dead temporaries is
    /// never emitted.
    fn local(&self, l: mir::Local) -> String {
        format!("M[{}]", l.index())
    }

    fn place_ty(&mut self, place: mir::Place<'tcx>) -> Result<Ty<'tcx>, ()> {
        Ok(self.place_expr(place)?.1)
    }

    /// The central §7 routine: static walk producing one access chain.
    fn place_expr(&mut self, place: mir::Place<'tcx>) -> Result<(String, Ty<'tcx>), ()> {
        let mut expr = self.local(place.local);
        let mut ty = match self.local_tys.get(place.local.index()) {
            Some(t) => *t,
            None => {
                self.err(format!("local _({}) out of range", place.local.index()));
                return Err(());
            }
        };
        let mut downcast: Option<VariantIdx> = None;
        for i in 0..place.projection.len() {
            match place.projection[i] {
                mir::ProjectionElem::Deref => {
                    match ty.kind() {
                        ty::Ref(_, inner, _) => ty = *inner,
                        ty::Adt(def, args) if def.is_box() => {
                            ty = args.type_at(0);
                        }
                        // Raw pointers have no address to follow in this model:
                        // the pointee value is stored directly, so dereferencing
                        // is the identity. Pointer *arithmetic* is still rejected.
                        ty::RawPtr(inner, _) => ty = *inner,
                        _ => {
                            self.err(format!("cannot dereference value of type {ty}"));
                            return Err(());
                        }
                        #[allow(unreachable_patterns)]
                        _ => unreachable!(),
                    }
                    // Transparent: no expression change.
                }
                mir::ProjectionElem::Field(idx, _) => {
                    let i = idx.as_usize();
                    // Transparent single-field handle wrappers project
                    // through `.0` as the identity. Any other opaque handle
                    // has no Rust-visible fields.
                    if let ty::Adt(def, args) = ty.kind() {
                        if tyutil::handle_field_ty(self.tcx, def, args).is_some() {
                            if i != 0 {
                                self.err(format!("bad field {i} of handle wrapper {ty}"));
                                return Err(());
                            }
                            let f = &def.non_enum_variant().fields[FieldIdx::from_usize(0)];
                            ty = self.subst(tyutil::subst_unchecked(
                                self.tcx,
                                args,
                                self.tcx.type_of(f.did).skip_binder(),
                            ));
                            downcast = None;
                            continue;
                        }
                        if tyutil::is_js_opaque(self.tcx, def.did()) {
                            self.err(format!(
                                "cannot access field {i} of opaque JS handle {ty} \
                                 (use #[js::import]/#[js::inline] to manipulate it in JS)"
                            ));
                            return Err(());
                        }
                    }
                    let is_enum = matches!(ty.kind(), ty::Adt(d, _) if d.is_enum());
                    if downcast.is_some() || is_enum {
                        let v = downcast.unwrap_or(VariantIdx::from_usize(0));
                        expr = format!("{expr}.v[{i}]");
                        match tyutil::adt_field_ty(self.tcx, ty, v, i) {
                            Some(t) => ty = self.subst(t),
                            None => {
                                self.err(format!("bad enum field access on {ty}"));
                                return Err(());
                            }
                        }
                    } else {
                        // `Box`/`Rc`/`Arc` are transparent: the field value is
                        // the inner value itself, so no array index.
                        let transparent_adt = match ty.kind() {
                            ty::Adt(def, _) => {
                                def.is_box() || {
                                    let p = def_path(self.tcx, def.did());
                                    p.ends_with("rc::Rc") || p.ends_with("sync::Arc")
                                }
                            }
                            _ => false,
                        };
                        if transparent_adt {
                            let fty = match ty.kind() {
                                ty::Adt(_, args) => args.type_at(0),
                                _ => unreachable!(),
                            };
                            ty = self.subst(fty);
                            downcast = None;
                            continue;
                        }
                        // Struct fields (including ZST slots, which the
                        // aggregate constructor emits as `undefined`) live at
                        // their declaration index, exactly like tuples and
                        // enum variant fields.
                        let expr_idx = i;
                        expr = format!("{expr}[{expr_idx}]");
                        ty = match ty.kind() {
                            ty::Tuple(tys) => match tys.get(i) {
                                Some(t) => *t,
                                None => {
                                    self.err(format!("tuple field {i} out of range on {ty}"));
                                    return Err(());
                                }
                            },
                            ty::Adt(def, args) => {
                                let variant = def.non_enum_variant();
                                match variant.fields.get(FieldIdx::from_usize(i)) {
                                    Some(f) => {
                                        let ft = tyutil::subst_unchecked(
                                            self.tcx,
                                            args,
                                            self.tcx.type_of(f.did).skip_binder(),
                                        );
                                        self.subst(ft)
                                    }
                                    None => {
                                        self.err(format!("field {i} out of range on {ty}"));
                                        return Err(());
                                    }
                                }
                            }
                            ty::Closure(..) | ty::CoroutineClosure(..) => {
                                match closure_upvar_ty(self.tcx, ty, i) {
                                    Some(t) => self.subst(t),
                                    None => {
                                        self.err(format!("upvar {i} out of range on {ty}"));
                                        return Err(());
                                    }
                                }
                            }
                            _ => {
                                self.err(format!("cannot project field {i} of {ty}"));
                                return Err(());
                            }
                        };
                    }
                    downcast = None;
                }
                mir::ProjectionElem::Index(idx) => {
                    let ie = self.local(idx);
                    match ty.kind() {
                        ty::Array(elem, _) | ty::Slice(elem) => ty = *elem,
                        ty::Adt(def, args) => {
                            let p = def_path(self.tcx, def.did());
                            if p != "alloc::vec::Vec" {
                                self.err(format!("cannot index into {ty}"));
                                return Err(());
                            }
                            ty = args.type_at(0);
                        }
                        _ => {
                            self.err(format!("cannot index into {ty}"));
                            return Err(());
                        }
                    }
                    expr = format!("{expr}[{ie}]");
                    downcast = None;
                }
                mir::ProjectionElem::ConstantIndex { offset, .. } => {
                    match ty.kind() {
                        ty::Array(elem, _) | ty::Slice(elem) => ty = *elem,
                        ty::Adt(def, args) => {
                            let p = def_path(self.tcx, def.did());
                            if p != "alloc::vec::Vec" {
                                self.err(format!("cannot index into {ty}"));
                                return Err(());
                            }
                            ty = args.type_at(0);
                        }
                        _ => {
                            self.err(format!("cannot index into {ty}"));
                            return Err(());
                        }
                    }
                    expr = format!("{expr}[{offset}]");
                    downcast = None;
                }
                mir::ProjectionElem::Subslice { from, to, from_end } => {
                    match ty.kind() {
                        ty::Array(elem, _) | ty::Slice(elem) => {
                            ty = Ty::new_slice(self.tcx, *elem);
                        }
                        _ => {
                            self.err(format!("cannot slice {ty}"));
                            return Err(());
                        }
                    }
                    if from_end {
                        expr = format!("{expr}.slice({from},{expr}.length-{to})");
                    } else {
                        expr = format!("{expr}.slice({from},{to})");
                    }
                    downcast = None;
                }
                mir::ProjectionElem::Downcast(_, v) => {
                    downcast = Some(v);
                }
                mir::ProjectionElem::OpaqueCast(t) => ty = self.subst(t),
                mir::ProjectionElem::UnwrapUnsafeBinder(_) | mir::ProjectionElem::PhantomDeref => {
                    self.err("binder/phantom projections are not supported".to_string());
                    return Err(());
                }
            }
        }
        Ok((expr, ty))
    }

    // ---------------- operands ----------------

    fn operand_ty(&mut self, op: &mir::Operand<'tcx>) -> Result<Ty<'tcx>, ()> {
        match op {
            mir::Operand::Copy(p) | mir::Operand::Move(p) => self.place_ty(*p),
            mir::Operand::Constant(c) => Ok(self.subst(c.const_.ty())),
            mir::Operand::RuntimeChecks(_) => Ok(self.tcx.types.bool),
        }
    }

    fn operand_expr(&mut self, op: &mir::Operand<'tcx>) -> Result<(String, Ty<'tcx>), ()> {
        match op {
            mir::Operand::Copy(p) | mir::Operand::Move(p) => self.place_expr(*p),
            mir::Operand::Constant(c) => {
                let ty = self.subst(c.const_.ty());
                // Function items and ctors decay to JS function references.
                match ty.kind() {
                    ty::FnDef(did, fargs) => {
                        let fargs = subst_args(self.tcx, self.args, fargs.skip_binder());
                        let e = self.fn_value_of(*did, fargs)?;
                        Ok((e, ty))
                    }
                    ty::Closure(..) => {
                        // Only non-capturing (ZST) closures can be constants.
                        Ok(("undefined".to_string(), ty))
                    }
                    _ => {
                        let ccx = self.ccx();
                        match constval::render_mir_const(&ccx, c.const_) {
                            Ok(s) => Ok((s, ty)),
                            Err(e) => {
                                self.err(e);
                                Err(())
                            }
                        }
                    }
                }
            }
            mir::Operand::RuntimeChecks(rc) => Ok((
                if rc.value(self.tcx.sess) {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
                self.tcx.types.bool,
            )),
        }
    }

    /// A function item used as a *value*: `$name` or a ctor arrow.
    fn fn_value_of(&mut self, did: DefId, fargs: ty::GenericArgsRef<'tcx>) -> Result<String, ()> {
        let enclosing_params = if self.body.arg_count > 0 {
            let param_tys: Vec<ty::GenericArg<'tcx>> = self.body.local_decls
                [mir::Local::from_usize(1)..mir::Local::from_usize(1 + self.body.arg_count)]
                .iter()
                .map(|ld| ty::GenericArg::from(ld.ty))
                .collect();
            Some(self.tcx.mk_args(&param_tys))
        } else {
            None
        };
        let enclosing_fn_def_id = self.body.source.def_id();
        let resolved = recognize::resolve_and_classify(
            self.tcx,
            did,
            fargs,
            self.cur_span,
            &|did| crate::is_js_import(did),
            enclosing_params,
            Some(enclosing_fn_def_id),
        );
        self.callee_as_value(resolved, did, fargs)
    }

    /// A resolved callee used as a *value*. `Not` (from `!=` via `!eq`)
    /// recurses; a bare negated function value is an arrow that negates.
    fn callee_as_value(
        &mut self,
        resolved: Result<Callee<'tcx>, String>,
        did: DefId,
        fargs: ty::GenericArgsRef<'tcx>,
    ) -> Result<String, ()> {
        match resolved {
            Ok(Callee::Direct(inst)) => {
                let (name, inst_crate) = fn_name_crate(self, &inst);
                if inst_crate != self.crate_name {
                    Ok(format!("__js_imports.{name}"))
                } else {
                    Ok(name)
                }
            }
            Ok(Callee::CtorFn) => Ok(self.ctor_arrow(did, fargs)?),
            Ok(Callee::Shim(_)) | Ok(Callee::Intrinsic(_)) => {
                self.err("intrinsic/shim function cannot be used as a value".to_string());
                Err(())
            }
            Ok(Callee::JsImport(did)) => {
                let name = match crate::get_import_name(&did) {
                    Some(n) => n,
                    None => match crate::import_path_of(self.tcx, did) {
                        Some(Ok(n)) => n,
                        _ => {
                            self.err(format!(
                                "could not resolve import `{}`",
                                self.tcx.def_path_str(did)
                            ));
                            return Err(());
                        }
                    },
                };
                // Splice templates are call shapes, not values.
                if is_splice_template(&name) {
                    self.err("template import cannot be used as a value (call it)".to_string());
                    return Err(());
                }
                Ok(name)
            }
            Ok(Callee::JsInline(_)) => {
                self.err("inline-JS stub cannot be used as a value (call it)".to_string());
                Err(())
            }
            Ok(Callee::Not(_)) => {
                self.err("`!=` cannot be used as a function value".to_string());
                Err(())
            }
            Ok(_) => {
                self.err("function cannot be used as a value here".to_string());
                Err(())
            }
            Err(e) => {
                self.err(e);
                Err(())
            }
        }
    }

    /// `(a0, a1) => ({t: D, v: [a0, a1]})` or `(...a) => ([...a])`.
    fn ctor_arrow(&mut self, did: DefId, fargs: ty::GenericArgsRef<'tcx>) -> Result<String, ()> {
        use rustc_hir::def::{CtorOf, DefKind};
        match self.tcx.def_kind(did) {
            DefKind::Ctor(CtorOf::Variant, _) => {
                let variant_did = self.tcx.parent(did);
                let enum_did = self.tcx.parent(variant_did);
                let adt = self.tcx.adt_def(enum_did);
                // Find variant index by ctor.
                let mut found = None;
                for (i, v) in adt.variants().iter().enumerate() {
                    if v.ctor_def_id() == Some(did) {
                        found = Some(VariantIdx::from_usize(i));
                        break;
                    }
                }
                let v = match found {
                    Some(v) => v,
                    None => {
                        self.err("constructor variant not found".to_string());
                        return Err(());
                    }
                };
                let enum_ty = Ty::new_adt(self.tcx, adt, fargs);
                let d = match discriminant_value(self.tcx, enum_ty, v) {
                    Some(d) => d,
                    None => {
                        self.err("bad enum constructor".to_string());
                        return Err(());
                    }
                };
                let n = adt.variant(v).fields.len();
                let params: Vec<String> = (0..n).map(|i| format!("c{i}")).collect();
                Ok(format!(
                    "({})=>({{t:{d},v:[{}]}})",
                    params.join(","),
                    params.join(",")
                ))
            }
            DefKind::Ctor(CtorOf::Struct, _) => Ok("(...c)=>([...c])".to_string()),
            _ => {
                self.err("not a constructor".to_string());
                Err(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Rvalues
// ---------------------------------------------------------------------------

impl<'tcx> Emitter<'tcx> {
    fn is_f32(&self, ty: Ty<'tcx>) -> bool {
        matches!(ty.kind(), ty::Float(ty::FloatTy::F32))
    }

    fn wrap_num(&self, e: String, k: NumKind) -> String {
        match k {
            NumKind::I8 => format!("(({e})<<24)>>24"),
            NumKind::I16 => format!("(({e})<<16)>>16"),
            NumKind::I32 | NumKind::Isize => format!("({e})|0"),
            NumKind::U8 => format!("({e})&255"),
            NumKind::U16 => format!("({e})&65535"),
            NumKind::U32 | NumKind::Usize | NumKind::Char => format!("({e})>>>0"),
        }
    }

    fn wrap_big(&self, e: String, k: BigKind) -> String {
        let b = k.bits();
        if k.signed() {
            format!("BigInt.asIntN({b},({e}))")
        } else {
            format!("BigInt.asUintN({b},({e}))")
        }
    }

    fn num_kind_of(&mut self, ty: Ty<'tcx>) -> Result<NumKind, ()> {
        match repr_of(self.tcx, ty) {
            Repr::Num(k) => Ok(k),
            _ => {
                self.err(format!("expected integer type, found {ty}"));
                Err(())
            }
        }
    }

    fn big_kind_of(&mut self, ty: Ty<'tcx>) -> Result<BigKind, ()> {
        match repr_of(self.tcx, ty) {
            Repr::Big(k) => Ok(k),
            _ => {
                self.err(format!("expected 64/128-bit integer type, found {ty}"));
                Err(())
            }
        }
    }

    fn maybe_fround(&self, e: String, ty: Ty<'tcx>) -> String {
        if self.is_f32(ty) {
            format!("Math.fround({e})")
        } else {
            e
        }
    }

    fn rvalue_expr(&mut self, rv: &mir::Rvalue<'tcx>, dest_ty: Ty<'tcx>) -> Result<String, ()> {
        match rv {
            mir::Rvalue::Use(op, _) => Ok(self.operand_expr(op)?.0),
            mir::Rvalue::Repeat(op, len) => {
                let (e, _) = self.operand_expr(op)?;
                let ccx = self.ccx();
                let n = constval::eval_usize(&ccx, *len).map_err(|m| {
                    self.err(m);
                })?;
                Ok(format!("Array({n}).fill({e})"))
            }
            mir::Rvalue::Ref(_, _, place) => Ok(self.place_expr(*place)?.0),
            mir::Rvalue::ThreadLocalRef(_) => {
                self.err("thread locals are not supported".to_string());
                Err(())
            }
            mir::Rvalue::RawPtr(..) => {
                self.err("raw pointers are not supported".to_string());
                Err(())
            }
            mir::Rvalue::Cast(kind, op, ty) => {
                let (e, oty) = self.operand_expr(op)?;
                let ty = self.subst(*ty);
                self.cast_expr(*kind, e, oty, ty)
            }
            mir::Rvalue::BinaryOp(op, box_ops) => {
                let (l, lty) = self.operand_expr(&box_ops.0)?;
                let (r, _) = self.operand_expr(&box_ops.1)?;
                self.binop_expr(*op, l, r, lty, dest_ty)
            }
            mir::Rvalue::UnaryOp(op, opnd) => {
                let (e, ty) = self.operand_expr(opnd)?;
                self.unop_expr(*op, e, ty)
            }
            mir::Rvalue::Discriminant(place) => {
                let (e, _) = self.place_expr(*place)?;
                Ok(format!("{e}.t"))
            }
            mir::Rvalue::Aggregate(box_kind, ops) => self.aggregate_expr(box_kind, ops),
            mir::Rvalue::CopyForDeref(place) => Ok(self.place_expr(*place)?.0),
            mir::Rvalue::WrapUnsafeBinder(op, _) => Ok(self.operand_expr(op)?.0),
            mir::Rvalue::Reborrow(_, _, place) => Ok(self.place_expr(*place)?.0),
        }
    }

    // ---------------- casts ----------------

    fn cast_expr(
        &mut self,
        kind: mir::CastKind,
        e: String,
        oty: Ty<'tcx>,
        ty: Ty<'tcx>,
    ) -> Result<String, ()> {
        use mir::CastKind;
        if oty == ty {
            return Ok(e);
        }
        match kind {
            CastKind::Subtype => Ok(e),
            CastKind::BoxDerefTransmute | CastKind::PtrToPtr => Ok(e),
            CastKind::PointerCoercion(..) => {
                let ro = repr_of(self.tcx, oty);
                let rd = repr_of(self.tcx, ty);
                if ro == Repr::BufStr && rd == Repr::Str {
                    // `&String` coerced to `&str`: read the cell.
                    return Ok(format!("{e}[0]"));
                }
                if matches!(ro, Repr::BufStr | Repr::Str)
                    && matches!(rd, Repr::BufStr | Repr::Str)
                    && ro == rd
                {
                    return Ok(e);
                }
                if ro == rd
                    || matches!(
                        (ro, rd),
                        (Repr::Array, Repr::Array) | (Repr::Transparent, Repr::Transparent)
                    )
                {
                    Ok(e)
                } else {
                    self.err(format!("pointer coercion {oty} -> {ty} is not supported"));
                    Err(())
                }
            }
            CastKind::Transmute => {
                if matches!(oty.kind(), ty::RawPtr(..))
                    && matches!(ty.kind(), ty::Int(_) | ty::Uint(_))
                {
                    return Ok("0".to_string());
                }
                self.err(format!("transmute {oty} -> {ty} is not supported"));
                Err(())
            }
            CastKind::FnPtrToPtr
            | CastKind::PointerExposeProvenance
            | CastKind::PointerWithExposedProvenance => {
                self.err(format!("pointer cast {oty} -> {ty} is not supported"));
                Err(())
            }
            CastKind::IntToInt => self.int_to_int(e, oty, ty),
            CastKind::FloatToInt => self.float_to_int(e, ty),
            CastKind::IntToFloat => self.int_to_float(e, oty, ty),
            CastKind::FloatToFloat => {
                if self.is_f32(ty) {
                    Ok(format!("Math.fround({e})"))
                } else {
                    Ok(e)
                }
            }
        }
    }

    fn int_to_int(&mut self, e: String, oty: Ty<'tcx>, ty: Ty<'tcx>) -> Result<String, ()> {
        // bool destination
        if matches!(repr_of(self.tcx, ty), Repr::Bool) {
            return match repr_of(self.tcx, oty) {
                Repr::Big(_) => Ok(format!("({e}!==0n)")),
                _ => Ok(format!("({e}!==0)")),
            };
        }
        // bool source
        if matches!(repr_of(self.tcx, oty), Repr::Bool) {
            return match repr_of(self.tcx, ty) {
                Repr::Big(k) => {
                    let v = format!("({e}?1n:0n)");
                    Ok(self.wrap_big(v, k))
                }
                Repr::Num(k) => {
                    let v = format!("({e}?1:0)");
                    Ok(self.wrap_num(v, k))
                }
                _ => {
                    self.err(format!("bad bool cast to {ty}"));
                    Err(())
                }
            };
        }
        match (repr_of(self.tcx, oty), repr_of(self.tcx, ty)) {
            (Repr::Num(_), Repr::Num(k)) => Ok(self.wrap_num(e, k)),
            (Repr::Num(_), Repr::Big(k)) => Ok(self.wrap_big(format!("BigInt({e})"), k)),
            (Repr::Big(_), Repr::Num(k)) => Ok(self.wrap_num(format!("Number({e})"), k)),
            (Repr::Big(_), Repr::Big(k)) => Ok(self.wrap_big(e, k)),
            _ => {
                self.err(format!("integer cast {oty} -> {ty} is not supported"));
                Err(())
            }
        }
    }

    fn float_to_int(&mut self, e: String, ty: Ty<'tcx>) -> Result<String, ()> {
        if matches!(repr_of(self.tcx, ty), Repr::Bool) {
            return Ok(format!("({e}!==0)"));
        }
        match repr_of(self.tcx, ty) {
            Repr::Num(k) => {
                let (lo, hi) = match k {
                    NumKind::I8 => ("-128", "127"),
                    NumKind::U8 => ("0", "255"),
                    NumKind::I16 => ("-32768", "32767"),
                    NumKind::U16 => ("0", "65535"),
                    NumKind::I32 | NumKind::Isize => ("-2147483648", "2147483647"),
                    NumKind::U32 | NumKind::Usize => ("0", "4294967295"),
                    NumKind::Char => ("0", "1114111"),
                };
                Ok(self.wrap_num(format!("__cast_num({e},{lo},{hi})"), k))
            }
            Repr::Big(k) => {
                Ok(self.wrap_big(format!("__cast_big({e},{},{})", k.bits(), k.signed()), k))
            }
            _ => {
                self.err(format!("float cast to {ty} is not supported"));
                Err(())
            }
        }
    }

    fn int_to_float(&mut self, e: String, oty: Ty<'tcx>, ty: Ty<'tcx>) -> Result<String, ()> {
        if !matches!(repr_of(self.tcx, ty), Repr::Float) {
            self.err(format!("int-to-float cast to {ty} is not supported"));
            return Err(());
        }
        match repr_of(self.tcx, oty) {
            Repr::Num(_) | Repr::Float => Ok(e),
            Repr::Big(_) => Ok(format!("Number({e})")),
            Repr::Bool => Ok(format!("({e}?1:0)")),
            _ => {
                self.err(format!("cast {oty} -> {ty} is not supported"));
                Err(())
            }
        }
    }

    // ---------------- binary / unary ops ----------------

    /// Receiver type for a primitive operator-trait shim. References and
    /// transparent boxes hold the same value as their pointee/inner, so peel
    /// them before reusing `binop_expr`/`unop_expr`.
    fn prim_self_ty(&mut self, tys: &[Ty<'tcx>]) -> Result<Ty<'tcx>, ()> {
        let t = match tys.first().copied() {
            Some(t) => t,
            None => {
                self.err("primitive operator call without operands".to_string());
                return Err(());
            }
        };
        Ok(tyutil::peel_eq_self(self.tcx, self.subst(t)))
    }

    fn binop_expr(
        &mut self,
        op: mir::BinOp,
        l: String,
        r: String,
        lty: Ty<'tcx>,
        dest_ty: Ty<'tcx>,
    ) -> Result<String, ()> {
        use mir::BinOp;
        let rep = repr_of(self.tcx, lty);
        // `String` comparisons need cell contents (arrays compare by identity).
        if matches!(rep, Repr::BufStr) {
            let (l, r) = (format!("{l}[0]"), format!("{r}[0]"));
            return self.binop_expr(op, l, r, self.tcx.types.str_, dest_ty);
        }
        // Comparisons work uniformly across number/bool/BigInt/string.
        match op {
            BinOp::Eq => return Ok(format!("({l}==={r})")),
            BinOp::Ne => return Ok(format!("({l}!=={r})")),
            BinOp::Lt => return Ok(format!("({l}<{r})")),
            BinOp::Le => return Ok(format!("({l}<={r})")),
            BinOp::Gt => return Ok(format!("({l}>{r})")),
            BinOp::Ge => return Ok(format!("({l}>={r})")),
            _ => {}
        }
        match rep {
            Repr::Bool => match op {
                BinOp::BitAnd => Ok(format!("({l}&&{r})")),
                BinOp::BitOr => Ok(format!("({l}||{r})")),
                BinOp::BitXor => Ok(format!("({l}!=={r})")),
                _ => {
                    self.err(format!("bad boolean operator {op:?}"));
                    Err(())
                }
            },
            Repr::Float => {
                let e = match op {
                    BinOp::Add => format!("({l}+{r})"),
                    BinOp::Sub => format!("({l}-{r})"),
                    BinOp::Mul => format!("({l}*{r})"),
                    BinOp::Div => format!("({l}/{r})"),
                    BinOp::Rem => format!("({l}%{r})"),
                    _ => {
                        self.err(format!("bad float operator {op:?}"));
                        return Err(());
                    }
                };
                Ok(self.maybe_fround(e, dest_ty))
            }
            Repr::Num(k) => {
                let e = match op {
                    BinOp::Add | BinOp::AddUnchecked => format!("({l}+{r})"),
                    BinOp::Sub | BinOp::SubUnchecked => format!("({l}-{r})"),
                    BinOp::Mul | BinOp::MulUnchecked => format!("({l}*{r})"),
                    BinOp::Div => {
                        format!("Math.trunc({l}/{r})")
                    }
                    BinOp::Rem => format!("({l}%{r})"),
                    BinOp::BitXor => format!("({l}^{r})"),
                    BinOp::BitAnd => format!("({l}&{r})"),
                    BinOp::BitOr => format!("({l}|{r})"),
                    BinOp::Shl | BinOp::ShlUnchecked => format!("({l}<<{r})"),
                    BinOp::Shr | BinOp::ShrUnchecked => {
                        if self.num_signed(k) {
                            format!("({l}>>{r})")
                        } else {
                            format!("({l}>>>{r})")
                        }
                    }
                    BinOp::AddWithOverflow => {
                        return Ok(self.with_overflow_num("|", "+", &l, &r, k));
                    }
                    BinOp::SubWithOverflow => {
                        return Ok(self.with_overflow_num("|", "-", &l, &r, k));
                    }
                    BinOp::MulWithOverflow => {
                        return Ok(self.with_overflow_num("|", "*", &l, &r, k));
                    }
                    BinOp::Cmp => return self.cmp_expr(l, r, dest_ty),
                    BinOp::Offset => {
                        self.err("pointer offset is not supported".to_string());
                        return Err(());
                    }
                    _ => {
                        self.err(format!("bad integer operator {op:?}"));
                        return Err(());
                    }
                };
                Ok(self.wrap_num(e, k))
            }
            Repr::Big(k) => {
                let e = match op {
                    BinOp::Add | BinOp::AddUnchecked => format!("({l}+{r})"),
                    BinOp::Sub | BinOp::SubUnchecked => format!("({l}-{r})"),
                    BinOp::Mul | BinOp::MulUnchecked => format!("({l}*{r})"),
                    BinOp::Div => format!("({l}/{r})"),
                    BinOp::Rem => format!("({l}%{r})"),
                    BinOp::BitXor => format!("({l}^{r})"),
                    BinOp::BitAnd => format!("({l}&{r})"),
                    BinOp::BitOr => format!("({l}|{r})"),
                    BinOp::Shl | BinOp::ShlUnchecked => {
                        format!("({l}<<BigInt(({r})&{}))", k.bits() - 1)
                    }
                    BinOp::Shr | BinOp::ShrUnchecked => {
                        if k.signed() {
                            format!("({l}>>BigInt(({r})&{}))", k.bits() - 1)
                        } else {
                            format!("__lshr_big({l},{r},{})", k.bits())
                        }
                    }
                    BinOp::AddWithOverflow => {
                        return Ok(self.with_overflow_big("+", &l, &r, k));
                    }
                    BinOp::SubWithOverflow => {
                        return Ok(self.with_overflow_big("-", &l, &r, k));
                    }
                    BinOp::MulWithOverflow => {
                        return Ok(self.with_overflow_big("*", &l, &r, k));
                    }
                    BinOp::Cmp => return self.cmp_expr(l, r, dest_ty),
                    BinOp::Offset => {
                        self.err("pointer offset is not supported".to_string());
                        return Err(());
                    }
                    _ => {
                        self.err(format!("bad integer operator {op:?}"));
                        return Err(());
                    }
                };
                Ok(self.wrap_big(e, k))
            }
            _ => {
                self.err(format!("binary operator {op:?} on {lty} is not supported"));
                Err(())
            }
        }
    }

    fn num_signed(&self, k: NumKind) -> bool {
        matches!(
            k,
            NumKind::I8 | NumKind::I16 | NumKind::I32 | NumKind::Isize | NumKind::Char
        )
    }

    fn int_bounds(&self, k: NumKind) -> (i128, i128) {
        match k {
            NumKind::I8 => (-128, 127),
            NumKind::U8 => (0, 255),
            NumKind::I16 => (-32768, 32767),
            NumKind::U16 => (0, 65535),
            NumKind::I32 | NumKind::Isize => (-2147483648, 2147483647),
            NumKind::U32 | NumKind::Usize | NumKind::Char => (0, 4294967295),
        }
    }

    /// `(lo, hi)` of an integer type as BigInt literals, for comparing
    /// against an exact BigInt power.
    fn int_bounds_lit(&mut self, ty: Ty<'tcx>) -> Result<(String, String), ()> {
        match repr_of(self.tcx, ty) {
            Repr::Num(k) => {
                let (lo, hi) = self.int_bounds(k);
                Ok((format!("{lo}n"), format!("{hi}n")))
            }
            Repr::Big(k) => Ok((big_int_lo(k), big_int_hi(k))),
            _ => {
                self.err(format!("expected an integer type, found {ty}"));
                Err(())
            }
        }
    }

    /// `(lo, hi)` of an integer type as a value of that type (to saturate to).
    fn int_extrema_lit(&mut self, ty: Ty<'tcx>) -> Result<(String, String), ()> {
        match repr_of(self.tcx, ty) {
            Repr::Num(k) => {
                let (lo, hi) = self.int_bounds(k);
                Ok((lo.to_string(), hi.to_string()))
            }
            Repr::Big(k) => Ok((big_int_lo(k), big_int_hi(k))),
            _ => {
                self.err(format!("expected an integer type, found {ty}"));
                Err(())
            }
        }
    }

    /// Narrow an exact BigInt expression to the integer type `ty`.
    /// `Self` of an integer `pow` call, taken from the destination type:
    /// `checked_pow` returns `Option<Self>` (so peel the `Some` field), the
    /// others return `Self` directly.
    fn pow_self_ty(&mut self, dest_ty: Ty<'tcx>, checked: bool) -> Result<Ty<'tcx>, ()> {
        let ty = self.subst(dest_ty);
        let ty = if checked {
            match tyutil::adt_field_ty(self.tcx, ty, VariantIdx::from_usize(1), 0) {
                Some(t) => self.subst(t),
                None => {
                    self.err(format!("checked_pow must return Option, found {ty}"));
                    return Err(());
                }
            }
        } else {
            ty
        };
        Ok(ty)
    }

    /// Narrow an exact BigInt expression to the integer type `ty`.
    fn narrow_big(&mut self, e: &str, ty: Ty<'tcx>) -> Result<String, ()> {
        match repr_of(self.tcx, ty) {
            Repr::Num(k) => {
                let n = num_bits(k);
                let wide = if num_signed(k) {
                    format!("BigInt.asIntN({n},{e})")
                } else {
                    format!("BigInt.asUintN({n},{e})")
                };
                Ok(self.wrap_num(format!("Number({wide})"), k))
            }
            Repr::Big(k) => {
                let n = k.bits();
                Ok(if k.signed() {
                    format!("BigInt.asIntN({n},({e}))")
                } else {
                    format!("BigInt.asUintN({n},({e}))")
                })
            }
            _ => {
                self.err(format!("`pow` is only defined on integers, found {ty}"));
                Err(())
            }
        }
    }

    fn with_overflow_num(&self, _wrap: &str, op: &str, l: &str, r: &str, k: NumKind) -> String {
        let (lo, hi) = self.int_bounds(k);
        // Width-correct wrap of the raw sum (same lattice as `wrap_num`,
        // but applied to the `__s` temporary).
        let w = match k {
            NumKind::I8 => "((__s<<24)>>24)",
            NumKind::I16 => "((__s<<16)>>16)",
            NumKind::I32 | NumKind::Isize => "(__s|0)",
            NumKind::U8 => "(__s&255)",
            NumKind::U16 => "(__s&65535)",
            NumKind::U32 | NumKind::Usize | NumKind::Char => "(__s>>>0)",
        };
        let s = format!("{l}{op}{r}");
        format!("(()=>{{const __s={s};return [{w},(__s<{lo}||__s>{hi})];}})()")
    }

    fn with_overflow_big(&self, op: &str, l: &str, r: &str, k: BigKind) -> String {
        let b = k.bits();
        let w = if k.signed() {
            format!("BigInt.asIntN({b},__s)")
        } else {
            format!("BigInt.asUintN({b},__s)")
        };
        format!("(()=>{{const __s=({l}{op}{r});const __w={w};return [__w,__s!==__w];}})()")
    }

    /// `BinOp::Cmp` → `Ordering` value `{t, v: []}`.
    fn cmp_expr(&mut self, l: String, r: String, dest_ty: Ty<'tcx>) -> Result<String, ()> {
        let less = self.enum_discr_by_name(dest_ty, "Less")?;
        let equal = self.enum_discr_by_name(dest_ty, "Equal")?;
        let greater = self.enum_discr_by_name(dest_ty, "Greater")?;
        Ok(format!(
            "(({l}<{r}?{{t:{less},v:[]}}:({l}>{r}?{{t:{greater},v:[]}}:{{t:{equal},v:[]}})))"
        ))
    }

    fn enum_discr_by_name(&mut self, ty: Ty<'tcx>, name: &str) -> Result<u128, ()> {
        // Callers frequently hold a reference (&Option<T>, &mut Option<T>).
        // Refs are transparent, so peel them before reading the ADT.
        let mut ty = ty;
        loop {
            match ty.kind() {
                ty::Ref(_, inner, _) => {
                    ty = *inner;
                }
                _ => break,
            }
        }
        let (def, _) = match ty.kind() {
            ty::Adt(def, args) if def.is_enum() => (*def, args),
            _ => {
                self.err(format!("expected enum type, found {ty}"));
                return Err(());
            }
        };
        for (v, d) in def.discriminants(self.tcx) {
            if def.variant(v).name.as_str() == name {
                return Ok(d.val);
            }
        }
        self.err(format!("enum {ty} has no variant {name}"));
        Err(())
    }

    fn unop_expr(&mut self, op: mir::UnOp, e: String, ty: Ty<'tcx>) -> Result<String, ()> {
        use mir::UnOp;
        match op {
            UnOp::Not => match repr_of(self.tcx, ty) {
                Repr::Bool => Ok(format!("(!{e})")),
                Repr::Num(k) => Ok(self.wrap_num(format!("~{e}"), k)),
                Repr::Big(k) => Ok(self.wrap_big(format!("~{e}"), k)),
                _ => {
                    self.err(format!("`!` on {ty} is not supported"));
                    Err(())
                }
            },
            UnOp::Neg => match repr_of(self.tcx, ty) {
                Repr::Float => Ok(format!("(-{e})")),
                Repr::Num(k) => Ok(self.wrap_num(format!("-({e})"), k)),
                Repr::Big(k) => Ok(self.wrap_big(format!("-({e})"), k)),
                _ => {
                    self.err(format!("negation of {ty} is not supported"));
                    Err(())
                }
            },
            UnOp::PtrMetadata => {
                let inner = match ty.kind() {
                    ty::Ref(_, t, _) | ty::RawPtr(t, _) => *t,
                    _ => ty,
                };
                match repr_of(self.tcx, inner) {
                    Repr::Array => Ok(format!("({e}.length)")),
                    Repr::Str | Repr::BufStr => Ok(format!("__str_len({e})")),
                    _ => {
                        self.err(format!("pointer metadata of {ty} is not supported"));
                        Err(())
                    }
                }
            }
        }
    }

    // ---------------- aggregates ----------------

    fn aggregate_expr(
        &mut self,
        kind: &mir::AggregateKind<'tcx>,
        ops: &IndexVec<FieldIdx, mir::Operand<'tcx>>,
    ) -> Result<String, ()> {
        use mir::AggregateKind;
        // Helper: render operands with per-field f32 rounding.
        match kind {
            AggregateKind::Tuple | AggregateKind::Array(_) => {
                let mut out = Vec::with_capacity(ops.len());
                for op in ops.iter() {
                    let (e, t) = self.operand_expr(op)?;
                    out.push(self.maybe_fround(e, t));
                }
                Ok(format!("[{}]", out.join(",")))
            }
            AggregateKind::Adt(did, variant, args, _, _) => {
                // Transparent single-field handle wrappers build as their
                // inner value. Any other opaque handle cannot be
                // constructed in Rust; it only arrives from JS
                // (imports/inline).
                {
                    let adt = self.tcx.adt_def(*did);
                    let sargs = subst_args(self.tcx, self.args, args);
                    if tyutil::handle_field_ty(self.tcx, &adt, sargs).is_some() {
                        if ops.len() == 1 {
                            let (e, t) = self.operand_expr(&ops[FieldIdx::from_usize(0)])?;
                            return Ok(self.maybe_fround(e, t));
                        }
                        self.err(format!("bad aggregate arity for handle wrapper {did:?}"));
                        return Err(());
                    }
                }
                if tyutil::is_js_opaque(self.tcx, *did) {
                    self.err(
                        "cannot construct opaque JS handle in Rust \
                         (obtain it from #[js::import]/#[js::inline])"
                            .to_string(),
                    );
                    return Err(());
                }
                let args = subst_args(self.tcx, self.args, args);
                let adt = self.tcx.adt_def(*did);
                let mut out = Vec::with_capacity(ops.len());
                for (i, op) in ops.iter().enumerate() {
                    let (e, _) = self.operand_expr(op)?;
                    let f = &adt.variant(*variant).fields[FieldIdx::from_usize(i)];
                    let fty = self.subst(tyutil::subst_unchecked(
                        self.tcx,
                        args,
                        self.tcx.type_of(f.did).skip_binder(),
                    ));
                    out.push(self.maybe_fround(e, fty));
                }
                if adt.is_enum() {
                    let enum_ty = Ty::new_adt(self.tcx, adt, args);
                    let d = match discriminant_value(self.tcx, enum_ty, *variant) {
                        Some(d) => d,
                        None => {
                            self.err("bad enum aggregate".to_string());
                            return Err(());
                        }
                    };
                    Ok(format!("{{t:{d},v:[{}]}}", out.join(",")))
                } else {
                    Ok(format!("[{}]", out.join(",")))
                }
            }
            AggregateKind::Closure(did, _args) => {
                let _ = did;
                let mut out = Vec::with_capacity(ops.len());
                for op in ops.iter() {
                    let (e, _) = self.operand_expr(op)?;
                    out.push(e);
                }
                Ok(format!("[{}]", out.join(",")))
            }
            AggregateKind::Coroutine(..) | AggregateKind::CoroutineClosure(..) => {
                self.err("async coroutines are not supported".to_string());
                Err(())
            }
            AggregateKind::RawPtr(..) => {
                self.err("raw pointers are not supported".to_string());
                Err(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Calls
// ---------------------------------------------------------------------------

impl<'tcx> Emitter<'tcx> {
    fn resolve_target(&mut self, func: &mir::Operand<'tcx>) -> Result<CallTarget<'tcx>, ()> {
        match func {
            mir::Operand::Constant(c) => {
                let ty = self.subst(c.const_.ty());
                match ty.kind() {
                    ty::FnDef(did, fargs) => {
                        let fargs = subst_args(self.tcx, self.args, fargs.skip_binder());
                        self.classify_did(*did, fargs)
                    }
                    ty::Closure(did, cargs) => {
                        let cargs = subst_args(self.tcx, self.args, *cargs);
                        match ty::Instance::try_resolve(self.tcx, mono_env(), *did, cargs) {
                            Ok(Some(inst)) => {
                                let name = fn_name(self.tcx, &self.crate_name, &inst);
                                Ok(CallTarget::ClosureDirect {
                                    name,
                                    env: "undefined".to_string(),
                                })
                            }
                            _ => {
                                self.err("could not resolve closure".to_string());
                                Err(())
                            }
                        }
                    }
                    _ => {
                        self.err("call to non-function constant".to_string());
                        Err(())
                    }
                }
            }
            mir::Operand::Copy(p) | mir::Operand::Move(p) => {
                let (expr, ty) = self.place_expr(*p)?;
                match ty.kind() {
                    ty::FnDef(did, fargs) => {
                        let fargs = subst_args(self.tcx, self.args, fargs.skip_binder());
                        if crate::is_js_import(did) {
                            // Bound foreign (#[js_import]) fn: carry the resolved
                            // instance args so the call-site wrapper passes cells
                            // only for raw &str params (String/ADT params arrive
                            // as cells and must not be unwrapped).
                            let name = crate::get_import_name(did)
                                .unwrap_or_else(|| "undefined".to_string());
                            let template = is_splice_template(&name);
                            return Ok(CallTarget::JsImport {
                                name,
                                args: fargs,
                                template,
                            });
                        }
                        // Static target known: direct call.
                        self.classify_did(*did, fargs)
                    }
                    ty::Closure(did, cargs) => {
                        let cargs = subst_args(self.tcx, self.args, *cargs);
                        match ty::Instance::try_resolve(self.tcx, mono_env(), *did, cargs) {
                            Ok(Some(inst)) => {
                                let name = fn_name(self.tcx, &self.crate_name, &inst);
                                Ok(CallTarget::ClosureDirect { name, env: expr })
                            }
                            _ => {
                                self.err("could not resolve closure".to_string());
                                Err(())
                            }
                        }
                    }
                    ty::FnPtr(..) => Ok(CallTarget::Dynamic { expr }),
                    _ => {
                        self.err(format!("call to non-function value of type {ty}"));
                        Err(())
                    }
                }
            }
            mir::Operand::RuntimeChecks(_) => {
                self.err("call to session flag".to_string());
                Err(())
            }
        }
    }

    fn classify_did(
        &mut self,
        did: DefId,
        fargs: ty::GenericArgsRef<'tcx>,
    ) -> Result<CallTarget<'tcx>, ()> {
        // Resolve free inference params left by impl-trait param method calls
        // (e.g. `dgs::point(name: impl Into<String>).into()`): the callee's
        // fargs carry ParamTy(0) pointing at the enclosing function's
        // inference param. Subst the enclosing function's *param types*
        // (opaque `impl Into<T>` → its hidden type), which `subst_opaque_param`
        // then concretizes to the projection output (`args[1]`), so
        // `try_resolve` monomorphizes the call.
        let enclosing_params = if self.body.arg_count > 0 {
            let param_tys: Vec<ty::GenericArg<'tcx>> = self.body.local_decls
                [mir::Local::from_usize(1)..mir::Local::from_usize(1 + self.body.arg_count)]
                .iter()
                .map(|ld| ty::GenericArg::from(ld.ty))
                .collect();
            Some(self.tcx.mk_args(&param_tys))
        } else {
            None
        };
        let enclosing_fn_def_id = self.body.source.def_id();
        let resolved = recognize::resolve_and_classify(
            self.tcx,
            did,
            fargs,
            self.cur_span,
            &|did| crate::is_js_import(did),
            enclosing_params,
            Some(enclosing_fn_def_id),
        );
        self.callee_to_target(resolved, did, fargs)
    }

    /// Map a resolved callee to a terminator call target. `Not` (from the
    /// `PartialEq::ne` → `!eq` redirect) recurses into the inner callee.
    fn callee_to_target(
        &mut self,
        resolved: Result<Callee<'tcx>, String>,
        did: DefId,
        fargs: ty::GenericArgsRef<'tcx>,
    ) -> Result<CallTarget<'tcx>, ()> {
        match resolved {
            Ok(Callee::Direct(inst)) => {
                let (nm, _) = fn_name_crate(self, &inst);
                let _ = nm;
                // A trait-method call (`Fn::call`, ...) resolving to a closure
                // body carries (env, args-tuple): spread the tuple.
                let is_closure_call = match inst.def {
                    ty::InstanceKind::Item(d) => {
                        self.tcx.def_kind(d) == rustc_hir::def::DefKind::Closure && d != did
                    }
                    _ => false,
                };
                let (name, inst_crate) = fn_name_crate(self, &inst);
                if std::env::var("JS_DEBUG_CALL").is_ok() {
                    use std::io::Write;
                    let mut f = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("/tmp/js_debug.log")
                        .unwrap();
                    f.write_all(
                        format!(
                            "classify_did: {} name={} inst_crate={} ctx={}\n",
                            self.tcx.def_path_str(inst.def_id()),
                            name,
                            inst_crate,
                            self.crate_name
                        )
                        .as_bytes(),
                    )
                    .ok();
                    if inst_crate != self.crate_name {
                        f.write_all(b"classify_did -> JsImport\n").ok();
                    }
                }
                if inst_crate != self.crate_name {
                    if std::env::var("JS_DEBUG_CALL").is_ok() {
                        use std::io::Write;
                        std::io::stderr()
                            .write_all(b"classify_did -> JsImport\n")
                            .ok();
                    }
                    // Cross-crate: resolve through the shared-imports map
                    // (the defining crate publishes every drained function
                    // there under its internal name). The `JsImport` target
                    // always carries the emittable callee.
                    Ok(CallTarget::JsImport {
                        name: format!("__js_imports.{name}"),
                        args: ty::GenericArgs::identity_for_item(self.tcx, did),
                        // Cross-crate linkage names are never templates.
                        template: false,
                    })
                } else if is_closure_call {
                    Ok(CallTarget::ClosureTrait { name })
                } else {
                    Ok(CallTarget::Direct { name })
                }
            }
            Ok(Callee::Shim(op)) => {
                // `into_iter` is the one shim whose impl is invisible after
                // path normalization: `<slice::Iter<..> as IntoIterator>` and
                // `<Vec<T> as IntoIterator>` both render as
                // `std::iter::IntoIterator::into_iter`. Dispatch on `Self`:
                // a Vec/array/slice needs a fresh iterator, an iterator (or
                // a range) is passed through unchanged.
                let op = if matches!(op, Shim::VecIntoIter)
                    && tyutil::is_iterator_ty(self.tcx, self.subst(fargs.type_at(0)))
                {
                    Shim::IntoFn
                } else {
                    op
                };
                Ok(CallTarget::Shim { op, args: fargs })
            }
            Ok(Callee::Intrinsic(op)) => Ok(CallTarget::Intrinsic { op, args: fargs }),
            Ok(Callee::CtorFn) => Ok(CallTarget::CtorValue),
            Ok(Callee::JsImport(did)) => {
                // Upstream method stubs live in no import map (their crate
                // ran in another process): read the path from metadata.
                let name = match crate::get_import_name(&did) {
                    Some(n) => n,
                    None => match crate::import_path_of(self.tcx, did) {
                        Some(Ok(n)) => n,
                        Some(Err(e)) => {
                            self.err(e);
                            return Err(());
                        }
                        None => {
                            self.err(format!(
                                "could not resolve import `{}`",
                                self.tcx.def_path_str(did)
                            ));
                            return Err(());
                        }
                    },
                };
                // The resolver carries no args for #[js_import] extern items;
                // use identity args as a fallback (js_import fns are typically
                // non-generic).
                let template = is_splice_template(&name);
                Ok(CallTarget::JsImport {
                    name,
                    args: ty::GenericArgs::identity_for_item(self.tcx, did),
                    template,
                })
            }
            Ok(Callee::JsInline(did)) => Ok(CallTarget::JsInline { did }),
            Ok(Callee::Panic(k)) => Ok(CallTarget::Panic(k)),
            Ok(Callee::FmtNew) => Ok(CallTarget::FmtNew),
            Ok(Callee::ArgNew(_)) => {
                self.err("fmt::Argument constructor must be peered, never called".to_string());
                Err(())
            }
            Ok(Callee::ClosureOnce(closure, cargs)) => {
                match ty::Instance::try_resolve(self.tcx, mono_env(), closure, cargs) {
                    Ok(Some(inst)) => Ok(CallTarget::ClosureTrait {
                        name: fn_name(self.tcx, &self.crate_name, &inst),
                    }),
                    _ => {
                        self.err("could not resolve closure".to_string());
                        Err(())
                    }
                }
            }
            Ok(Callee::Unsupported(msg)) => {
                self.err(msg.to_string());
                Err(())
            }
            Ok(Callee::Not(inner)) => Ok(CallTarget::Not(Box::new(self.callee_to_target(
                Ok(*inner),
                did,
                fargs,
            )?))),
            Err(e) => {
                self.err(e);
                Err(())
            }
        }
    }

    /// If `ty` is `AssertUnwindSafe<F>`, return `F`. The wrapper is the
    /// canonical `catch_unwind` pattern and is transparent at runtime (a
    /// single-field struct, so the inner value is field 0).
    fn unwind_safe_inner(&self, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
        match ty.kind() {
            ty::Adt(def, args) => {
                let p = def_path(self.tcx, def.did());
                if (p.ends_with("panic::AssertUnwindSafe") || p == "AssertUnwindSafe")
                    && !def.is_enum()
                    && args.len() >= 1
                {
                    Some(self.subst(args.type_at(0)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Resolve a closure *type* to its emitted function name (for shims that
    /// take closures: retain, catch_unwind, ...).
    fn closure_name_of_ty(&mut self, ty: Ty<'tcx>) -> Result<String, ()> {
        let ty = self.subst(ty);
        let ty = self.unwind_safe_inner(ty).unwrap_or(ty);
        match ty.kind() {
            ty::Closure(did, _cargs) => {
                // Resolve the closure's JS name from its DefId alone. try_resolve
                // with the closure's full generic args ICEs on this nightly when
                // the erased RustCall signature is inspected, so build an
                // identity-arg instance instead (mirrors fn_name_crate_inner's
                // closure path which names closures by parent + unique index).
                let inst =
                    ty::Instance::new_raw(*did, ty::GenericArgs::identity_for_item(self.tcx, *did));
                Ok(fn_name(self.tcx, &self.crate_name, &inst))
            }
            _ => {
                self.err(format!("expected closure type, found {ty}"));
                Err(())
            }
        }
    }

    /// Emit a call expression. `arg_ops` are the MIR call args.
    fn call_expr(
        &mut self,
        target: &CallTarget<'tcx>,
        arg_ops: &[Spanned<mir::Operand<'tcx>>],
        dest_ty: Ty<'tcx>,
    ) -> Result<String, ()> {
        // Peered targets analyze the raw operands (which may contain
        // pointer constants); rendering them eagerly would fail.
        let peered = matches!(
            target,
            CallTarget::FmtNew
                | CallTarget::Shim {
                    op: Shim::FmtFormat | Shim::FmtWriteFmt | Shim::IoPrint,
                    ..
                }
                | CallTarget::Panic(PanicKind::Fmt)
        );
        let mut ae: Vec<(String, Ty<'tcx>)> = Vec::with_capacity(arg_ops.len());
        if !peered {
            for sp in arg_ops.iter() {
                ae.push(self.operand_expr(&sp.node)?);
            }
        }
        let args: Vec<String> = ae.iter().map(|(e, _)| e.clone()).collect();
        let tys: Vec<Ty<'tcx>> = ae.iter().map(|(_, t)| *t).collect();
        match target {
            CallTarget::Direct { name } if name.contains("closure_to_js_function") => {
                let arg_ty = tys.get(0).copied().unwrap_or(dest_ty);
                let cname = self.closure_name_of_ty(arg_ty).unwrap_or_else(|_| "undefined".to_string());
                Ok(format!(
                    "(function(...args){{ return {cname}({}, ...args); }})",
                    ae[0].0
                ))
            }
            CallTarget::Direct { name } => Ok(format!("{name}({})", args.join(","))),
            CallTarget::ClosureDirect { name, env } => {
                // A closure is called as (env, args...): env is the captured
                // value array, args are the Fn/FnMut/FnOnce parameters.
                if args.is_empty() {
                    Ok(format!("{name}({env})"))
                } else {
                    Ok(format!("{name}({env}, {})", args.join(",")))
                }
            }
            CallTarget::ClosureTrait { name } => {
                if ae.len() != 2 {
                    self.err(format!(
                        "closure trait call has {} args, expected (env, args-tuple)",
                        ae.len()
                    ));
                    return Err(());
                }
                if !matches!(tys[1].kind(), ty::Tuple(_)) {
                    self.err(format!("closure args must be a tuple, found {}", tys[1]));
                    return Err(());
                }
                Ok(format!("{name}({} ,...{})", ae[0].0, ae[1].0))
            }
            CallTarget::Dynamic { expr } => Ok(format!("{expr}({})", args.join(","))),
            CallTarget::CtorValue => {
                self.err("constructor cannot be called".to_string());
                Err(())
            }
            CallTarget::Shim {
                op,
                args: callee_args,
            } => self.shim_expr(*op, *callee_args, arg_ops, &args, &tys, dest_ty),
            CallTarget::Intrinsic {
                op,
                args: callee_args,
            } => self.intrinsic_expr(*op, *callee_args, &args, &tys, dest_ty),
            CallTarget::JsImport {
                name,
                args: param_tys,
                template,
            } => {
                let mut v: Vec<String> = Vec::with_capacity(args.len());
                for (i, arg) in args.iter().enumerate() {
                    // Unwrap only raw boundary buffer types (&str) that arrive
                    // wrapped in a string cell; Cell-typed params (String, ADTs)
                    // are passed through as cells. Use the callee's param type
                    // rather than the caller's operand type so `String` params
                    // (e.g. `impl Into<String>` resolved to `String`) are not
                    // unwrapped.
                    let t = param_tys
                        .get(i)
                        .copied()
                        .and_then(|ga| ga.as_type())
                        .unwrap_or_else(|| *tys.get(i).unwrap_or(&dest_ty));
                    let mut tt = self.subst(t);
                    loop {
                        match tt.kind() {
                            ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => tt = *inner,
                            _ => break,
                        }
                    }
                    let mut e = match tt.kind() {
                        // Idempotent unwrap: cells arrive as `[str]`, but
                        // values that already passed through a deref are raw
                        // strings (unwrapping those again would take the
                        // first *character*).
                        ty::Str => format!("__str_raw({arg})"),
                        _ => arg.clone(),
                    };
                    // A Rust closure passed to a foreign/JS-imported
                    // function is lowered to a JavaScript function that
                    // invokes the closure's generated body with its captured
                    // environment, so DOM/event callbacks can be expressed in
                    // Rust without a hand-written JS wrapper.
                    if let Some(ty) = tys.get(i).copied() {
                        let ty = self.subst(ty);
                        if matches!(ty.kind(), ty::Closure(..)) {
                            let name = self.closure_name_of_ty(ty)?;
                            e = format!(
                                "(function(...args){{ return {name}({}, ...args); }})",
                                e
                            );
                        }
                    }
                    // Template imports own their call shape, so `Option`
                    // operands map to null-or-value here (bare indirections
                    // keep the `{t, v}` object for user JS to interpret).
                    if *template {
                        if let Some(ty) = tys.get(i).copied() {
                            if let Some(u) = self.unwrap_option_splice(&e, ty) {
                                e = u;
                            }
                        }
                    }
                    v.push(e);
                }
                // `name` is the emittable callee: either the shared-imports
                // indirection (`__js_imports.<fn>`) or a true JS path from
                // `#[js::import = "<path>"]` (called as written, so methods
                // keep their receiver). An explicit template path (flagged
                // at target creation) substitutes the (already unwrapped)
                // argument expressions and emits the full expression.
                let call = if *template {
                    match substitute_splices(name, &v) {
                        Ok(s) => s,
                        Err(e) => {
                            self.err(e);
                            return Err(());
                        }
                    }
                } else {
                    format!("{name}({})", v.join(","))
                };
                self.wrap_js_return(call, dest_ty)
            }
            CallTarget::JsInline { did } => self.inline_js_expr(*did, arg_ops, dest_ty),
            CallTarget::Panic(k) => self.panic_expr(*k, arg_ops, &args),
            CallTarget::Not(inner) => {
                Ok(format!("!({})", self.call_expr(inner, arg_ops, dest_ty)?))
            }
            CallTarget::FmtNew => {
                self.err("fmt::Arguments::new must be peered, never called".to_string());
                Err(())
            }
            CallTarget::ArgNew(_) => {
                self.err("fmt::Argument constructor must be peered, never called".to_string());
                Err(())
            }
            CallTarget::Unsupported(msg) => {
                self.err(msg.to_string());
                Err(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Inline JS (`#[js::inline]` stubs) and `$N` template substitution.
// ---------------------------------------------------------------------------

/// `true` if `s` contains a `$N` splice placeholder (`$` + ASCII digits).
fn template_has_splice(s: &str) -> bool {
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c == '$' {
            if let Some(&d) = it.peek() {
                if d.is_ascii_digit() {
                    return true;
                }
            }
        }
    }
    false
}

/// `true` when `name` is an explicit `#[js::import = "path"]` template rather
/// than a `__js_imports.*` indirection: indirections (bare imports and
/// cross-crate linkages, whose `$`-defindex suffixes only look like splices)
/// are always plain callees.
fn is_splice_template(name: &str) -> bool {
    !name.starts_with("__js_imports.") && template_has_splice(name)
}

/// Substitute `$N` placeholders in `template` with `splices[N]`. A `$` not
/// followed by digits is left as-is (jQuery `$`, `${x}` in JS template
/// literals, etc. pass through). Out-of-range indices are an error.
fn substitute_splices(template: &str, splices: &[String]) -> Result<String, String> {
    let mut out = String::with_capacity(template.len() + splices.len() * 8);
    let mut it = template.chars().peekable();
    while let Some(c) = it.next() {
        if c != '$' {
            out.push(c);
            continue;
        }
        let mut num = String::new();
        while let Some(&d) = it.peek() {
            if d.is_ascii_digit() {
                num.push(d);
                it.next();
            } else {
                break;
            }
        }
        if num.is_empty() {
            out.push('$');
            continue;
        }
        let idx: usize = num
            .parse()
            .map_err(|_| format!("bad splice placeholder ${num}"))?;
        match splices.get(idx) {
            Some(s) => out.push_str(s),
            None => {
                return Err(format!(
                    "splice ${idx} out of range (template takes {} argument(s))",
                    splices.len()
                ));
            }
        }
    }
    Ok(out)
}

/// Unquote a `"..."` JS string literal produced by `constval::js_string`.
fn unquote_js_string(lit: &str) -> Option<String> {
    if lit.len() < 2 || !lit.starts_with('"') || !lit.ends_with('"') {
        return None;
    }
    let inner = &lit[1..lit.len() - 1];
    let mut out = String::with_capacity(inner.len());
    let mut it = inner.chars();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next()? {
            '"' => out.push('"'),
            '\\' => out.push('\\'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            '0' => out.push('\0'),
            'u' => {
                if it.next()? != '{' {
                    return None;
                }
                let mut hex = String::new();
                for h in it.by_ref() {
                    if h == '}' {
                        break;
                    }
                    hex.push(h);
                }
                let cp = u32::from_str_radix(&hex, 16).ok()?;
                out.push(char::from_u32(cp)?);
            }
            _ => return None,
        }
    }
    Some(out)
}

/// Safety gate for inline templates (bundles inline into HTML `<script>`):
/// forbid tag-breaking sequences and cap length. Newlines, backticks and
/// `${` are allowed (raw JS); `$N` splices are substituted before emission.
fn validate_inline_template(t: &str) -> Option<String> {
    if t.len() > 8192 {
        return Some("inline-JS template is too long (max 8192 bytes)".to_string());
    }
    let lower = t.to_lowercase();
    for bad in ["</script", "<!--"] {
        if lower.contains(bad) {
            return Some(format!("inline-JS template contains forbidden {bad:?}"));
        }
    }
    None
}

impl<'tcx> Emitter<'tcx> {
    /// Emit a `#[js::inline]` stub call: the first operand is the `&str`
    /// template, the rest are `$N` splices. Return wrapping mirrors imports
    /// (owned `String` results arrive raw and are cell-wrapped).
    fn inline_js_expr(
        &mut self,
        did: DefId,
        arg_ops: &[Spanned<mir::Operand<'tcx>>],
        dest_ty: Ty<'tcx>,
    ) -> Result<String, ()> {
        let _ = did;
        if arg_ops.is_empty() {
            self.err("#[js::inline] stub needs a `&str` template argument".to_string());
            return Err(());
        }
        let template = self.peer_inline_template(arg_ops[0].node.clone())?;
        if let Some(e) = validate_inline_template(&template) {
            self.err(e);
            return Err(());
        }
        // Render splices: `Option` maps to null-or-value; string-like
        // values unwrap idempotently so both `&str` (raw) and `String`
        // (cell) splice as JS strings.
        let mut splices = Vec::with_capacity(arg_ops.len().saturating_sub(1));
        for sp in arg_ops.iter().skip(1) {
            let (e, t) = self.operand_expr(&sp.node)?;
            if let Some(u) = self.unwrap_option_splice(&e, t) {
                splices.push(u);
                continue;
            }
            let mut inner = self.subst(t);
            loop {
                match inner.kind() {
                    ty::Ref(_, i, _) | ty::RawPtr(i, _) => inner = *i,
                    _ => break,
                }
            }
            let is_string = matches!(inner.kind(), ty::Str)
                || matches!(repr_of(self.tcx, inner), Repr::BufStr);
            splices.push(if is_string {
                format!("__str_raw({e})")
            } else {
                e
            });
        }
        let code = match substitute_splices(&template, &splices) {
            Ok(s) => s,
            Err(e) => {
                self.err(e);
                return Err(());
            }
        };
        self.wrap_js_return(code, dest_ty)
    }

    /// Follow an inline-JS template operand back to its `&str` literal value.
    fn peer_inline_template(&mut self, op: mir::Operand<'tcx>) -> Result<String, ()> {
        let op = self.strip_deref(op)?;
        match op {
            mir::Operand::Constant(c) => self.const_str_value(c.const_),
            mir::Operand::Copy(p) | mir::Operand::Move(p) => {
                if p.projection.is_empty() {
                    match self.find_assign(p.local)? {
                        Some(mir::Rvalue::Use(inner, _)) => self.peer_inline_template(inner),
                        Some(mir::Rvalue::Ref(_, _, inner)) => {
                            self.peer_inline_template(mir::Operand::Copy(inner))
                        }
                        Some(rv) => {
                            self.err(format!(
                                "inline-JS template must be a string literal, found {rv:?}"
                            ));
                            Err(())
                        }
                        None => {
                            self.err(
                                "inline-JS template must be a string literal".to_string(),
                            );
                            Err(())
                        }
                    }
                } else {
                    self.err("inline-JS template must be a plain string literal".to_string());
                    Err(())
                }
            }
            mir::Operand::RuntimeChecks(_) => {
                self.err("bad inline-JS template".to_string());
                Err(())
            }
        }
    }

    /// Read the value of a `&str`/`str` constant.
    fn const_str_value(&mut self, c: mir::Const<'tcx>) -> Result<String, ()> {
        use mir::ConstValue;
        let ty = self.subst(c.ty());
        let is_str = matches!(ty.kind(), ty::Str)
            || matches!(ty.kind(), ty::Ref(_, inner, _) if matches!(inner.kind(), ty::Str));
        if !is_str {
            self.err(format!("inline-JS template must be `&str`, found {ty}"));
            return Err(());
        }
        match c {
            mir::Const::Val(ConstValue::Slice { alloc_id, meta }, _) => {
                let bytes = self.read_alloc(alloc_id, 0, meta)?;
                String::from_utf8(bytes).map_err(|_| {
                    self.err("inline-JS template is not valid UTF-8".to_string());
                })
            }
            mir::Const::Unevaluated(uc, _) => {
                let args = tyutil::subst_unchecked(self.tcx, self.args, uc.args);
                let uc = mir::UnevaluatedConst {
                    def: uc.def,
                    args,
                    promoted: uc.promoted,
                };
                match self
                    .tcx
                    .const_eval_resolve(mono_env(), uc, self.cur_span)
                {
                    Ok(ConstValue::Slice { alloc_id, meta }) => {
                        let bytes = self.read_alloc(alloc_id, 0, meta)?;
                        String::from_utf8(bytes).map_err(|_| {
                            self.err("inline-JS template is not valid UTF-8".to_string());
                        })
                    }
                    _ => {
                        self.err("inline-JS template must be a string literal".to_string());
                        Err(())
                    }
                }
            }
            _ => {
                // Last resort: render as a JS literal and unquote it.
                let ccx = self.ccx();
                match constval::render_mir_const(&ccx, c) {
                    Ok(lit) => unquote_js_string(&lit).ok_or_else(|| {
                        self.err("inline-JS template must be a string literal".to_string());
                    }),
                    Err(e) => {
                        self.err(e);
                        Err(())
                    }
                }
            }
        }
    }

    /// Map an `Option` operand to a JS null-or-value expression for
    /// generated call shapes (template imports, inline splices):
    /// `None` becomes `undefined`, `Some` unwraps (cells idempotently).
    /// Returns `None` when `ty` is not an `Option`. The operand is always
    /// a MIR place or constant (side-effect-free), so mentioning it twice
    /// is sound.
    fn unwrap_option_splice(&mut self, expr: &str, ty: Ty<'tcx>) -> Option<String> {
        let inner = self.option_payload(ty)?;
        let (_, none_d) = self.opt_discrs(ty).ok()?;
        let mut it = self.subst(inner);
        loop {
            match it.kind() {
                ty::Ref(_, i, _) | ty::RawPtr(i, _) => it = *i,
                _ => break,
            }
        }
        let is_string = matches!(it.kind(), ty::Str)
            || matches!(repr_of(self.tcx, it), Repr::BufStr);
        let val = if is_string {
            format!("__str_raw({expr}.v[0])")
        } else {
            format!("{expr}.v[0]")
        };
        Some(format!("({expr}.t==={none_d}?undefined:{val})"))
    }

    /// `Some(T)`'s payload type when `ty` (through references) is
    /// `Option<T>`; `None` otherwise (including `Result`, which foreign
    /// calls never auto-wrap — throwing JS is an open item).
    fn option_payload(&mut self, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
        let t = self.subst(ty);
        if !tyutil::is_option_ty(self.tcx, t) {
            return None;
        }
        let mut inner = t;
        loop {
            match inner.kind() {
                ty::Ref(_, i, _) | ty::RawPtr(i, _) => inner = *i,
                _ => break,
            }
        }
        match inner.kind() {
            ty::Adt(_, args) => args.iter().find_map(|a| match a.kind() {
                ty::GenericArgKind::Type(t) => Some(self.subst(t)),
                _ => None,
            }),
            _ => None,
        }
    }

    /// Payload value expression for a `Some`/`Ok` cell: owned strings are
    /// cell-wrapped, anything else passes through.
    fn payload_wrap(&self, v: &str, inner: Ty<'tcx>) -> String {
        let inner = self.subst(inner);
        if matches!(repr_of(self.tcx, inner), Repr::BufStr) {
            format!("[{v}]")
        } else {
            v.to_string()
        }
    }

    /// `Some(...)` expression for a JS value that may be null/undefined.
    fn some_or_none(&mut self, v: &str, opt_ty: Ty<'tcx>) -> Result<String, ()> {
        let inner = self.option_payload(opt_ty).ok_or_else(|| {
            self.err(format!("expected Option<T>, found {opt_ty}"));
        })?;
        let (some_d, none_d) = self.opt_discrs(opt_ty)?;
        let some_val = self.payload_wrap(v, inner);
        Ok(format!(
            "(({v}===null||{v}===undefined)?{{t:{none_d},v:[]}}:{{t:{some_d},v:[{some_val}]}})"
        ))
    }

    /// Wrap a foreign-JS call result per the destination type. `Result<T,
    /// E>` captures a thrown exception as `Err` (evaluated once, via an
    /// IIFE whose `try` also applies the `Option`/cell rules to the `Ok`
    /// payload); `Option<T>` maps null/undefined to `None`; owned `String`
    /// arrives raw and is cell-wrapped; anything else passes through.
    fn wrap_js_return(&mut self, call: String, dest_ty: Ty<'tcx>) -> Result<String, ()> {
        let mut dt = self.subst(dest_ty);
        loop {
            match dt.kind() {
                ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => dt = *inner,
                _ => break,
            }
        }
        if tyutil::is_result_ty(self.tcx, dt) {
            let (ok_d, err_d) = self.res_discrs(dt)?;
            let mut ts = match dt.kind() {
                ty::Adt(_, args) => args.iter().filter_map(|a| match a.kind() {
                    ty::GenericArgKind::Type(t) => Some(self.subst(t)),
                    _ => None,
                }),
                _ => {
                    self.err(format!("bad Result type {dt}"));
                    return Err(());
                }
            };
            let (ok_t, err_t) = match (ts.next(), ts.next()) {
                (Some(a), Some(b)) => (a, b),
                _ => {
                    self.err(format!("bad Result type {dt}"));
                    return Err(());
                }
            };
            // The `Ok` payload reuses the value rules (including nested
            // `Option`); the `Err` payload is the caught exception object
            // (stringified when a `String` is expected).
            let ok_val = if self.option_payload(ok_t).is_some() {
                self.some_or_none("__v", ok_t)?
            } else {
                self.payload_wrap("__v", ok_t)
            };
            let err_inner = self.subst(err_t);
            let err_val = if matches!(repr_of(self.tcx, err_inner), Repr::BufStr) {
                "[String(__e)]".to_string()
            } else {
                "__e".to_string()
            };
            return Ok(format!(
                "(()=>{{try{{const __v={call};return {{t:{ok_d},v:[{ok_val}]}};}}catch(__e){{return {{t:{err_d},v:[{err_val}]}};}}}})()"
            ));
        }
        if self.option_payload(dt).is_some() {
            // Evaluate the call once: bind, then map null to `None`.
            return Ok(format!(
                "(()=>{{const __v={call};return {}}})()",
                self.some_or_none("__v", dt)?
            ));
        }
        if matches!(repr_of(self.tcx, dt), Repr::BufStr) {
            Ok(format!("[{call}]"))
        } else {
            Ok(call)
        }
    }
}

// ---------------------------------------------------------------------------
// Shims
// ---------------------------------------------------------------------------

impl<'tcx> Emitter<'tcx> {
    /// Wrap a JS string in the mutable 1-cell holder used for `String`.
    fn scell(&self, e: &str) -> String {
        format!("[{e}]")
    }
    /// Read the string value out of a `String` cell (no-op for `&str`).
    fn sval(&self, e: &str, ty: Ty<'tcx>) -> String {
        if matches!(repr_of(self.tcx, ty), Repr::BufStr) {
            format!("{e}[0]")
        } else {
            e.to_string()
        }
    }
    /// `true` if a value of type `ty` is a `String` cell, looking through
    /// references and `Box` (whose representations are transparent).
    fn is_str_cell_ty(&mut self, ty: Ty<'tcx>) -> bool {
        let mut t = self.subst(ty);
        loop {
            match t.kind() {
                ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => t = *inner,
                ty::Adt(def, args) if def.is_box() || def.is_box() => t = args.type_at(0),
                _ => break,
            }
        }
        matches!(repr_of(self.tcx, t), Repr::BufStr)
    }

    /// (Some_or_Ok, None_or_Err) discriminants for an `Option`/`Result`
    /// enum type, tolerating transparent references.
    fn opt_discrs(&mut self, ty: Ty<'tcx>) -> Result<(u128, u128), ()> {
        let mut ty = ty;
        loop {
            match ty.kind() {
                ty::Ref(_, inner, _) => {
                    ty = *inner;
                }
                _ => break,
            }
        }
        let is_result = self.is_result_adt(ty);
        if is_result {
            let ok = self.enum_discr_by_name(ty, "Ok")?;
            let err = self.enum_discr_by_name(ty, "Err")?;
            Ok((ok, err))
        } else {
            let none = self.enum_discr_by_name(ty, "None")?;
            let some = self.enum_discr_by_name(ty, "Some")?;
            Ok((some, none))
        }
    }

    fn res_discrs(&mut self, ty: Ty<'tcx>) -> Result<(u128, u128), ()> {
        let ok = self.enum_discr_by_name(ty, "Ok")?;
        let err = self.enum_discr_by_name(ty, "Err")?;
        Ok((ok, err))
    }

    /// `true` if the (ref-peeled) type is std `Result` (used by `?` and
    /// Option/Result shims to pick the success discriminant).
    fn is_result_adt(&self, ty: Ty<'tcx>) -> bool {
        let mut ty = ty;
        loop {
            match ty.kind() {
                ty::Ref(_, inner, _) => {
                    ty = *inner;
                }
                _ => break,
            }
        }
        match ty.kind() {
            ty::Adt(def, _) => {
                let p = crate::tyutil::def_path(self.tcx, def.did());
                p == "core::result::Result" || p.ends_with("::result::Result")
            }
            _ => false,
        }
    }

    /// `true` if the (ref-peeled) type is std `Option`.
    fn is_option_adt(&self, ty: Ty<'tcx>) -> bool {
        let mut ty = ty;
        loop {
            match ty.kind() {
                ty::Ref(_, inner, _) => {
                    ty = *inner;
                }
                _ => break,
            }
        }
        match ty.kind() {
            ty::Adt(def, _) => {
                let p = crate::tyutil::def_path(self.tcx, def.did());
                p == "core::option::Option" || p.ends_with("::option::Option")
            }
            _ => false,
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// JS storage shape for uninitialized memory (`Box::new_uninit`).
    /// Leaves are `undefined` (never read before a write on valid paths);
    /// containers mirror the runtime representation so field-projection
    /// writes land correctly. Pointer-like fields (`Box`, references,
    /// `Rc`/`Arc`) are `undefined`: valid code replaces them wholesale
    /// before any read (only reading them first would be UB).
    fn uninit_shape(&mut self, ty: Ty<'tcx>) -> Result<String, ()> {
        let ty = self.subst(ty);
        match ty.kind() {
            ty::Bool
            | ty::Char
            | ty::Int(_)
            | ty::Uint(_)
            | ty::Float(_)
            | ty::Str
            | ty::Never
            | ty::Closure(..)
            | ty::Coroutine(..)
            | ty::CoroutineClosure(..)
            | ty::FnDef(..)
            | ty::FnPtr(..)
            | ty::Ref(..)
            | ty::RawPtr(..)
            | ty::Dynamic(..)
            | ty::Pat(..) => Ok("undefined".to_string()),
            ty::Adt(def, args) if def.is_box() => Ok("undefined".to_string()),
            ty::Adt(def, _) if tyutil::is_std_item(self.tcx, def.did()) => {
                let p = tyutil::def_path(self.tcx, def.did());
                if p.ends_with("rc::Rc") || p.ends_with("sync::Arc") {
                    return Ok("undefined".to_string());
                }
                if p.ends_with("string::String") || p == "String" {
                    return Ok("[\"\"\"]".to_string());
                }
                if p.ends_with("vec::Vec") || p == "Vec" {
                    return Ok("[]".to_string());
                }
                self.uninit_adt_shape(ty)
            }
            ty::Adt(..) => self.uninit_adt_shape(ty),
            ty::Array(elem, len) => {
                let ccx = self.ccx();
                let n = constval::eval_usize(&ccx, *len).map_err(|m| {
                    self.err(m);
                })?;
                if n > 256 {
                    self.err(format!(
                        "uninitialized array of {n} elements is not supported"
                    ));
                    return Err(());
                }
                let e = self.uninit_shape(*elem)?;
                Ok(format!("[{}]", vec![e; n as usize].join(",")))
            }
            ty::Slice(_) => Ok("[]".to_string()),
            ty::Tuple(tys) => {
                let mut out = Vec::with_capacity(tys.len());
                for t in tys.iter() {
                    out.push(self.uninit_shape(t)?);
                }
                Ok(format!("[{}]", out.join(",")))
            }
            _ => {
                self.err(format!("uninitialized storage for {ty} is not supported"));
                Err(())
            }
        }
    }

    /// Storage shape for struct/union/enum aggregates (see `uninit_shape`).
    fn uninit_adt_shape(&mut self, ty: Ty<'tcx>) -> Result<String, ()> {
        match ty.kind() {
            ty::Adt(def, args) => {
                if def.is_enum() {
                    // Best effort: first variant's discriminant, no payload.
                    // Reads before a write are UB; writes replace or extend.
                    let d = crate::tyutil::discriminant_value(
                        self.tcx,
                        ty,
                        rustc_abi::VariantIdx::from_u32(0),
                    )
                    .unwrap_or(0);
                    return Ok(format!("{{t:{d},v:[]}}"));
                }
                let variant = def.non_enum_variant();
                let mut out = Vec::with_capacity(variant.fields.len());
                for f in variant.fields.iter() {
                    let ft = tyutil::subst_unchecked(
                        self.tcx,
                        *args,
                        self.tcx.type_of(f.did).skip_binder(),
                    );
                    out.push(self.uninit_shape(self.subst(ft))?);
                }
                Ok(format!("[{}]", out.join(",")))
            }
            _ => {
                self.err(format!("uninitialized storage for {ty} is not supported"));
                Err(())
            }
        }
    }

    fn shim_expr(
        &mut self,
        op: Shim,
        callee_args: ty::GenericArgsRef<'tcx>,
        arg_ops: &[Spanned<mir::Operand<'tcx>>],
        args: &[String],
        tys: &[Ty<'tcx>],
        dest_ty: Ty<'tcx>,
    ) -> Result<String, ()> {
        use Shim::*;
        // Helper to fetch positional args.
        let a = |i: usize| {
            args.get(i)
                .cloned()
                .unwrap_or_else(|| "undefined".to_string())
        };
        let e = match op {
            VecNew | VecWithCapacity => "[]".to_string(),
            VecPush => format!("{}.push({})", a(0), a(1)),
            VecPop => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!("({0}.length===0?__none({dn}):__some({ds},{0}.pop()))", a(0))
            }
            VecLen => format!("{}.length", a(0)),
            VecIsEmpty => format!("({}.length===0)", a(0)),
            VecClear => format!("({}.length=0)", a(0)),
            VecTruncate => format!("({0}.length=Math.min({0}.length,{1}))", a(0), a(1)),
            VecRemove => format!("{0}.splice(__idx({0},{1}),1)[0]", a(0), a(1)),
            VecInsert => {
                let (v, i, x) = (a(0), a(1), a(2));
                format!("(({i}<0||{i}>{v}.length?__panic(\"insertion index (is \"+{i}+\") should be <= len (is \"+{v}.length+\")\"):{v}.splice({i},0,{x})))")
            }
            VecFirst => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!("({0}.length===0?__none({dn}):__some({ds},{0}[0]))", a(0))
            }
            VecLast => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!("({0}.length===0?__none({dn}):__some({ds},{0}[{0}.length-1]))", a(0))
            }
            VecGet => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let (v, i) = (a(0), a(1));
                format!("(({i}<0||{i}>={v}.length?__none({dn}):__some({ds},{v}[{i}])))")
            }
            VecClone => format!("{}.slice()", a(0)),
            VecReserve => "undefined".to_string(),
            VecCapacity => format!("{}.length", a(0)),
            VecSwapRemove => {
                let (v, i) = (a(0), a(1));
                format!("({v}.length===1?{v}.pop():{v}.splice(__idx({v},{i}),1,{v}.pop())[0])")
            }
            VecRetain => {
                // args: [vec_ref, closure]
                let pred_ty = tys.get(1).copied().unwrap_or(dest_ty);
                let pred_fn = self.closure_name_of_ty(pred_ty)?;
                format!("__vec_retain({},{},{})", a(0), a(1), pred_fn)
            }
            VecIntoIter => format!("__it_new({0})", a(0)),
            VecAsSlice => a(0),
            VecFromElem => format!("Array({}).fill({})", a(1), a(0)),
            VecFromArray => format!("{}.slice()", a(0)),
            VecAppend => format!("(__vec_extend_slice({0},{1}),{1}.length=0)", a(0), a(1)),
            VecSplitOff => {
                let (v, at) = (a(0), a(1));
                format!("(({at}<0||{at}>{v}.length?__panic(\"split_off index (is \"+{at}+\") should be <= len (is \"+{v}.length+\")\"):{v}.splice({at}))")
            }
            VecExtendFromSlice => format!("__vec_extend_slice({},{})", a(0), a(1)),
            VecExtendFromWithin => {
                format!("__vec_extend_slice({0},{0}.slice({1}[0],{1}[1]))", a(0), a(1))
            }
            VecIndex | SliceIndex | VecIndexMut | SliceIndexMut => {
                self.slice_index(&a(0), &a(1), tys.get(1).copied())?
            }
            SliceLen => format!("{}.length", a(0)),
            SliceIsEmpty => format!("({}.length===0)", a(0)),
            SliceGet => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let (v, i) = (a(0), a(1));
                format!("(({i}<0||{i}>={v}.length?__none({dn}):__some({ds},{v}[{i}])))")
            }
            SliceFirst => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!("({0}.length===0?__none({dn}):__some({ds},{0}[0]))", a(0))
            }
            SliceLast => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!("({0}.length===0?__none({dn}):__some({ds},{0}[{0}.length-1]))", a(0))
            }
            SliceIter | SliceIterMut => format!("__it_new({})", a(0)),
            SliceSplitAt => {
                let (v, i) = (a(0), a(1));
                format!("(({i}<0||{i}>{v}.length?__panic(\"mid index \"+{i}+\" exceeds length \"+{v}.length):[{v}.slice(0,{i}),{v}.slice({i})])")
            }
            SliceToVec => format!("{}.slice()", a(0)),
            SliceReverse => format!("{}.reverse()", a(0)),
            IterCount => format!("__it_count({})", a(0)),
            IterLast => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let it = a(0);
                format!("(()=>{{const __v=__it_last({it});return __v===undefined?__none({dn}):__some({ds},__v);}})()")
            }
            IterNth => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let it = a(0);
                format!("(()=>{{const __v=__it_nth({it},{});return __v===undefined?__none({dn}):__some({ds},__v);}})()", a(1))
            }
            IterPosition => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!("(()=>{{const __p=__it_position({0}, {f}, {1});return __p===null?__none({dn}):__some({ds},__p);}})()", a(0), a(1))
            }
            IterFind => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("(()=>{{const __v=__it_find({0}, {f}, {1});return __v===undefined?__none(0):__some(1,__v);}})()", a(0), a(1))
            }
            IterAny => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("__it_any({0}, {f}, {1})", a(0), a(1))
            }
            IterAll => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("__it_all({0}, {f}, {1})", a(0), a(1))
            }
            IterFold => {
                let f = self.closure_name_of_ty(tys.get(2).copied().unwrap_or(dest_ty))?;
                format!("__it_fold({0}, {1}, {f}, {2})", a(0), a(1), a(2))
            }
            IterForEach => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("__it_for_each({0}, {f}, {1})", a(0), a(1))
            }
            IterSum => {
                let zero = "0".to_string();
                format!("__it_sum({}, {zero})", a(0))
            }
            IterProduct => format!("__it_product({}, 1)", a(0)),
            IterMin => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!("(()=>{{const __m=__it_min({}, []);return __m===undefined?__none({dn}):__some({ds},__m);}})()", a(0))
            }
            IterMax => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!("(()=>{{const __m=__it_max({}, []);return __m===undefined?__none({dn}):__some({ds},__m);}})()", a(0))
            }
            IterMinBy => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("__it_min_by({0}, {f}, {1})", a(0), a(1))
            }
            IterMaxBy => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("__it_max_by({0}, {f}, {1})", a(0), a(1))
            }
            IterMap => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("__it_map({0}, {f}, {1})", a(0), a(1))
            }
            IterFilter => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("__it_filter({0}, {f}, {1})", a(0), a(1))
            }
            IterFilterMap => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("__it_filter_map({0}, {f}, {1})", a(0), a(1))
            }
            IterFlatMap => {
                let f = self.closure_name_of_ty(tys.get(1).copied().unwrap_or(dest_ty))?;
                format!("__it_flat_map({0}, {f}, {1})", a(0), a(1))
            }
            IterFlatten => {
                // Elements may be Options/Results (`.t`/`.v`) or plain arrays.
                let mut et = tys[0];
                if let ty::Ref(_, i, _) = et.kind() {
                    et = *i;
                }
                let present = match et.kind() {
                    ty::Adt(_, a) => {
                        let e = self.subst(a.type_at(0));
                        let mut e = e;
                        if let ty::Ref(_, i, _) = e.kind() {
                            e = *i;
                        }
                        if self.opt_discrs(e).is_ok() {
                            self.opt_discrs(e).ok().map(|(p, _)| p)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                match present {
                    Some(d) => format!("__it_flatten({}, {d})", a(0)),
                    None => format!("__it_flatten({})", a(0)),
                }
            }
            IterRev => format!("__it_rev({})", a(0)),
            IterEnumerate => format!("__it_enumerate({})", a(0)),
            IterZip => format!("__it_zip({}, {})", a(0), a(1)),
            IterChain => format!("__it_chain({}, {})", a(0), a(1)),
            IterTake => format!("__it_take({}, {})", a(0), a(1)),
            IterSkip => format!("__it_skip({}, {})", a(0), a(1)),
            IterStepBy => format!("__it_step_by({}, {})", a(0), a(1)),
            IterCloned => format!("__it_cloned({})", a(0)),
            IterCopied => format!("__it_copied({})", a(0)),
            IterEq => format!("__deep_eq({}, {})", a(0), a(1)),
            CloneFn => format!("__clone({})", a(0)),
            IntoFn => a(0),
            // Integer `pow` family: the power is computed exactly in BigInt
            // and then narrowed, which is correct through 128 bits (and
            // wraps, matching `-C overflow-checks=off`). `Self` comes from
            // the destination type: these methods are non-generic, so the
            // instance's generic args are empty.
            IntPow | IntPowWrapping => {
                let (b, e) = (a(0), a(1));
                let ty = self.subst(dest_ty);
                let narrow = self.narrow_big("__v", ty)?;
                format!("(()=>{{const __v=BigInt({b})**BigInt({e});return {narrow};}})()")
            }
            IntPowChecked => {
                let (b, e) = (a(0), a(1));
                let ty = self.pow_self_ty(dest_ty, true)?;
                let narrow = self.narrow_big("__v", ty)?;
                let (lo, hi) = self.int_bounds_lit(ty)?;
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!(
                    "(()=>{{const __v=BigInt({b})**BigInt({e});return (__v<{lo}||__v>{hi})?__none({dn}):__some({ds},{narrow});}})()"
                )
            }
            IntPowOverflowing => {
                let (b, e) = (a(0), a(1));
                let ty = self.pow_self_ty(dest_ty, false)?;
                let narrow = self.narrow_big("__v", ty)?;
                let (lo, hi) = self.int_bounds_lit(ty)?;
                format!(
                    "(()=>{{const __v=BigInt({b})**BigInt({e});return [{narrow},(__v<{lo}||__v>{hi})];}})()"
                )
            }
            IntPowSaturating => {
                let (b, e) = (a(0), a(1));
                let ty = self.subst(dest_ty);
                let narrow = self.narrow_big("__v", ty)?;
                let (lo, hi) = self.int_bounds_lit(ty)?;
                let (lov, hiv) = self.int_extrema_lit(ty)?;
                format!(
                    "(()=>{{const __v=BigInt({b})**BigInt({e});return __v>{hi}?{hiv}:(__v<{lo}?{lov}:{narrow});}})()"
                )
            }
            AsRefFn | AsMutFn | BorrowFn => a(0),
            MustUseFn => a(0),
            ToStringFn => self.scell(&format!("String({})", a(0))),
            RangeIncNew => format!("[{0},{1},1]", a(0), a(1)),
            RangeIncContains => format!("(({1}[0]<={0}&&{0}<={1}[1]))", a(0), a(1)),
            WriteFmtTrait => {
                let fmt = self.peer_format_args(1, arg_ops)?.expr();
                format!("({0}[0]+={fmt})", a(0))
            }
            OptUnwrap => {
                let x = a(0);
                let (on, _off) = self.opt_discrs(tys[0])?;
                format!("(()=>{{if({x}.t!=={on}) __panic(\"called `Option::unwrap()` on a `None` value\");return {x}.v[0];}})()")
            }
            ResUnwrap => {
                let x = a(0);
                let (ok, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{{if({x}.t!=={ok}) __panic(\"called `Result::unwrap()` on an `Err` value\");return {x}.v[0];}})()")
            }
            OptExpect | ResExpect => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{{if({x}.t!=={on}) __panic({});return {x}.v[0];}})()", a(1))
            }
            OptIsSome => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("({x}.t==={on})")
            }
            OptIsNone => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("({x}.t!=={on})")
            }
            ResIsOk => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("({x}.t==={on})")
            }
            ResIsErr => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("({x}.t!=={on})")
            }
            OptMap => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (on, off) = self.opt_discrs(tys[0])?;
                format!("__opt_map({x},{on},{off},{fnm},{fe})")
            }
            OptAndThen => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (on, off) = self.opt_discrs(tys[0])?;
                format!("__opt_and_then({x},{on},{off},{fnm},{fe})")
            }
            // `Option::take`: replace the place with None, return the old.
            OptTake => {
                let x = a(0);
                let (_, none) = self.opt_discrs(tys[0])?;
                format!("(()=>{{const __t={x};{x}=__none({none});return __t;}})()")
            }
            OptMapOr => {
                let (x, d, fe) = (a(0), a(1), a(2));
                let fnm = self.closure_name_of_ty(tys[2])?;
                let (on, off) = self.opt_discrs(tys[0])?;
                format!("__opt_map_or({x},{on},{off},{d},{fnm},{fe})")
            }
            OptMapOrElse => {
                let (x, dfe) = (a(0), a(1));
                let dfn = self.closure_name_of_ty(tys[1])?;
                let fe = a(2);
                let fnm = self.closure_name_of_ty(tys[2])?;
                let (on, off) = self.opt_discrs(tys[0])?;
                format!("__opt_map_or_else({x},{on},{off},{dfn},{dfe},{fnm},{fe})")
            }
            OptFilter => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (on, off) = self.opt_discrs(tys[0])?;
                format!("__opt_filter({x},{on},{off},{fnm},{fe})")
            }
            OptUnwrapOr => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("__opt_unwrap_or({x},{on},{})", a(1))
            }
            OptUnwrapOrElse => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{x}.t==={on}?{x}.v[0]:{fnm}({fe},{x}.v[0]))()")
            }
            OptUnwrapOrDefault => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{{if({x}.t!=={on}) return __default_of({}); return {x}.v[0];}})()", a(1))
            }
            OptOkOr => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                let (ok, err) = self.res_discrs(dest_ty)?;
                format!("(()=>{x}.t==={on}?__ok({ok},{x}.v[0]):__err({err},{}))()", a(1))
            }
            OptOkOrElse => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (on, _) = self.opt_discrs(tys[0])?;
                let (ok, err) = self.res_discrs(dest_ty)?;
                format!("(()=>{x}.t==={on}?__ok({ok},{x}.v[0]):__err({err},{fnm}({fe})))()")
            }
            OptOrElse => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{x}.t==={on}?{x}:{fnm}({fe}))()")
            }
            OptAnd => {
                let (x, y) = (a(0), a(1));
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("__opt_and({x},{y},{on})")
            }
            OptOr => {
                let (x, y) = (a(0), a(1));
                let (on, off) = self.opt_discrs(tys[0])?;
                format!("__opt_or({x},{y},{on},{off})")
            }
            OptXor => {
                let (x, y) = (a(0), a(1));
                let (on, off) = self.opt_discrs(tys[0])?;
                format!("__opt_xor({x},{y},{on},{off})")
            }
            OptInspect => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("__opt_inspect({x},{on},{fnm},{fe})")
            }
            OptCopied | ResCopied => a(0),
            OptCloned => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{x}.t==={on}?{{t:{on},v:[__clone({x}.v[0])]}}:{x})()")
            }
            OptAsRef | OptAsDeref | ResAsRef => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{x}.t==={on}?{{t:{on},v:[{x}.v[0]]}}:{x})()")
            }
            OptContains => {
                let x = a(0);
                let (on, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{x}.t==={on}?__deep_eq({x}.v[0],{}):false)()", a(1))
            }
            ResContains => {
                let x = a(0);
                let (ok, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{x}.t==={ok}?__deep_eq({x}.v[0],{}):false)()", a(1))
            }
            // `?` desugar: `Try::branch(x)` splits a Result/Option into a
            // ControlFlow (Continue(value) | Break(residual-as-value)). The
            // MIR's subsequent match on ControlFlow runs through the normal
            // SwitchInt emission; `TryFromResidual` below completes the exit.
            TryBranch => {
                let x = a(0);
                let in_ty = tys.first().copied().unwrap_or(dest_ty);
                let (good, _) = self.opt_discrs(in_ty)?;
                let cont = self.enum_discr_by_name(dest_ty, "Continue")?;
                let brk = self.enum_discr_by_name(dest_ty, "Break")?;
                format!("({x}.t==={good}?{{t:{cont},v:[{x}.v[0]]}}:{{t:{brk},v:[{x}]}})")
            }
            // `FromResidual::from_residual(r)`: for a Result destination the
            // residual already has the output shape (identity); for Option
            // the residual is unit and the output is None.
            TryFromResidual => {
                let r = a(0);
                if self.is_result_adt(dest_ty) {
                    r
                } else if self.is_option_adt(dest_ty) {
                    let (_, none) = self.opt_discrs(dest_ty)?;
                    format!("__none({none})")
                } else {
                    self.err(format!(
                        "unsupported `?` destination type {dest_ty} (only Result/Option)"
                    ));
                    return Err(());
                }
            }
            ResMap => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (ok, err) = self.opt_discrs(tys[0])?;
                format!("__res_map({x},{ok},{err},{fnm},{fe})")
            }
            ResMapErr => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (ok, err) = self.opt_discrs(tys[0])?;
                format!("__res_map_err({x},{ok},{err},{fnm},{fe})")
            }
            ResAndThen => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (ok, err) = self.opt_discrs(tys[0])?;
                format!("__res_and_then({x},{ok},{err},{fnm},{fe})")
            }
            ResOrElse => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (ok, err) = self.opt_discrs(tys[0])?;
                format!("__res_or_else({x},{ok},{err},{fnm},{fe})")
            }
            ResUnwrapOr => {
                let x = a(0);
                let (ok, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{x}.t==={ok}?{x}.v[0]:{})()", a(1))
            }
            ResUnwrapOrElse => {
                let (x, fe) = (a(0), a(1));
                let fnm = self.closure_name_of_ty(tys[1])?;
                let (ok, _) = self.opt_discrs(tys[0])?;
                format!("(()=>{x}.t==={ok}?{x}.v[0]:{fnm}({fe},{x}.v[0]))()")
            }
            ResErr => {
                let x = a(0);
                let (ok, _) = self.opt_discrs(tys[0])?;
                let (some, none) = self.opt_discrs(dest_ty)?;
                format!("(()=>{x}.t==={ok}?__none({none}):__some({some},{x}.v[0]))()")
            }
            ResOk => {
                let x = a(0);
                let (ok, _) = self.opt_discrs(tys[0])?;
                let (some, none) = self.opt_discrs(dest_ty)?;
                format!("(()=>{x}.t==={ok}?__some({some},{x}.v[0]):__none({none}))()")
            }
            ResCloned => {
                let x = a(0);
                let (ok, _) = self.opt_discrs(tys[0])?;
                let (some, none) = self.opt_discrs(dest_ty)?;
                format!("(()=>{x}.t==={ok}?{{t:{some},v:[__clone({x}.v[0])]}}:{{t:{none},v:[]}})()")
            }
            IterCollect => {
                let it_expr = a(0);
                let cnt = format!("__it_collect({it_expr})");
                match dest_ty.kind() {
                    ty::Adt(def, _) if !def.is_enum()
                        && tyutil::is_std_item(self.tcx, def.did()) =>
                    {
                        let p = def_path(self.tcx, def.did());
                        if p.ends_with("string::String") || p == "String" {
                            // Elements may be chars (code points) or str
                            // pieces: `__str_join` decodes both.
                            self.scell(&format!("__str_join({cnt})"))
                        } else {
                            cnt
                        }
                    }
                    ty::Adt(..) => {
                        // Result / Option / user enum: build the payload then rewrap.
                        let (ok, err) = self.res_discrs(dest_ty)?;
                        // `collect::<Result<V,E>>` needs per-item error handling.
                        let payload = format!("__it_collect_result({}, {})", it_expr, err);
                        format!("(()=>{{const __r={payload};return __r.err!==undefined?__err({err},__r.err):__ok({ok},__r.v);}})()")
                    }
                    ty::Tuple(..) => {
                        // e.g. collect::<(Vec, Vec)>
                        cnt
                    }
                    _ => cnt,
                }
            }
            SliceSort => format!("__slice_sort_inplace({0})", a(0)),
            SliceConcat => format!("[].concat(...{})", a(0)),
            SliceJoin => format!("__str_cell({0}.join({1}))", a(0), a(1)),
            SliceCopyFromSlice => self.copy_from_slice(arg_ops, &args)?,
            SliceContains => format!("__slice_contains({},{})", a(0), a(1)),
            SliceStartsWith => format!("__slice_starts_with({},{})", a(0), a(1)),
            SliceEndsWith => format!("__slice_ends_with({},{})", a(0), a(1)),
            IterNext => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!("(()=>{{const __r=__it_next_raw({});return __r===null?__none({dn}):__some({ds},__r.v);}})()", a(0))
            }
            IterNextBack => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let it = a(0);
                format!("(()=>{{const __r=__it_next_back({it});return __r===null?__none({dn}):__some({ds},__r.v);}})()")
            }
            IterLen => format!("__it_len({})", a(0)),
            IterSizeHint => {
                // (usize, Option<usize>) with upper = Some(remaining).
                let upper_ty = match dest_ty.kind() {
                    ty::Tuple(tys) if tys.len() == 2 => tys[1],
                    _ => {
                        self.err("bad size_hint return type".to_string());
                        return Err(());
                    }
                };
                let (ds, _dn) = self.opt_discrs(upper_ty)?;
                let it = a(0);
                format!("[__it_rem({it}),__some({ds},__it_rem({it}))]")
            }
            IterAsSlice => format!("__it_slice({})", a(0)),
            CharsNext => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let it = a(0);
                format!("({it}[1]<0||{it}[1]>={it}[2]?__none({dn}):__some({ds},{it}[0][{it}[1]++]))")
            }
            BytesNext => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let it = a(0);
                format!("({it}[1]<{it}[0].length?__some({ds},{it}[0][{it}[1]++]):__none({dn}))")
            }
            StringNew => r#"[""]"#.to_string(),
            StringFrom => self.scell(&a(0)),
            StringPushStr => format!("({}[0]+={})", a(0), a(1)),
            StringPush => format!("({}[0]+=String.fromCodePoint({}))", a(0), a(1)),
            StringLen => format!("__str_len({}[0])", a(0)),
            StringIsEmpty => format!("({}[0].length===0)", a(0)),
            StringAsStr => format!("{}[0]", a(0)),
            StringClone => format!("{}[0]", a(0)),
            StringClear => format!("({}[0]=\"\")", a(0)),
            StringTruncate => format!("({}[0]=__str_truncate({}[0],{}))", a(0), a(0), a(1)),
            StringPop => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let v = a(0);
                format!("(()=>{{const __cs=[...{v}[0]];const __c=__cs.pop();{v}[0]=__cs.join(\"\");return __c===undefined?__none({dn}):__some({ds},__c);}})()")
            }
            StringAdd => self.scell(&format!("({}[0]+{}[0])", a(0), a(1))),
            StrToOwned => self.scell(&a(0)),
            // `&str` / `String` -> `PointRef::Named`: a PointRef cell
            // `{t:0,v:[String-cell]}` wrapping a `&str` value inside a
            // String cell (`[&str]` = a(0)).
            PointRefFromStr => format!("[{{t:0,v:[[{}]]}}]", a(0)),
            // `(f64, f64)` -> `PointRef::Coords`: a PointRef cell
            // `{t:1,v:[f64,f64]}`.
            PointRefFromTuple => format!("[{{t:1,v:[{},{}]}}]", a(0), a(1)),
            StrLen => format!("__str_len({})", a(0)),
            StrIsEmpty => format!("({}.length===0)", a(0)),
            StrAsBytes => {
                match tys.first().copied() {
                    Some(t) if self.is_str_cell_ty(t) => format!("__str_bytes({}[0])", a(0)),
                    _ => format!("__str_bytes({})", a(0)),
                }
            }
            // `chars()` yields code points (the backend's `char` repr is a
            // number). The quad must have 4 elements: a 3-element one would
            // be read back as a range by `__it_norm`.
            StrChars => format!(
                "(({s})=>{{const __c=[...Array.isArray(__s)?__s[0]:__s].map((__ch)=>__ch.codePointAt(0));return [__c,0,__c.length,1];}})({})",
                a(0),
                s = "__s",
            ),
            StrSplitAt => format!("__str_split_at_chk({},{})", a(0), a(1)),
            StrGet => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let (v, r) = (a(0), a(1));
                let rt = tys.get(1).copied();
                let (lo, hi) = self.str_range_bounds(&r, rt)?;
                format!("(()=>{{const __gr=__str_get_range({v},{lo},{hi});return __gr.ok?__some({ds},__gr.val):__none({dn});}})()")
            }
            StrIndex => {
                let (v, r) = (a(0), a(1));
                self.str_index(v, r, tys.get(1).copied())?
            }
            // Owned-`String` results live in 1-cells (see `scell`).
            StrRepeat => self.scell(&format!("{}.repeat({})", a(0), a(1))),
            StrToUpper => self.scell(&format!("{}.toUpperCase()", a(0))),
            StrToLower => self.scell(&format!("{}.toLowerCase()", a(0))),
            StrTrim => format!("{}.trim()", a(0)),
            StrContains => format!("{}.includes({})", a(0), self.str_pattern(&a(1), tys.get(1).copied())?),
            StrStartsWith => format!("{}.startsWith({})", a(0), self.str_pattern(&a(1), tys.get(1).copied())?),
            StrEndsWith => format!("{}.endsWith({})", a(0), self.str_pattern(&a(1), tys.get(1).copied())?),
            StrFind => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let (v, pat) = (a(0), self.str_pattern(&a(1), tys.get(1).copied())?);
                format!("(()=>{{const __i=__str_find({v},{pat});return __i<0?__none({dn}):__some({ds},__i);}})()")
            },
            StrBytes => format!("[__str_bytes({}),0]", a(0)),
            StrParse => self.parse_expr(callee_args, &a(0), dest_ty)?,
            BoxNew | BoxLeak => a(0),
            // Uninitialized box used by the `vec![a, b, c]` lowering:
            // `Box::new_uninit()` materializes shape-correct storage whose
            // payload is written through field projections, then read back
            // by `box_assume_init_into_vec_unsafe` (`Box<[T]>::into_vec`
            // shares the shape: the payload array is the vec).
            BoxNewUninit => {
                let payload = match self.subst(dest_ty).kind() {
                    ty::Adt(def, args) if def.is_box() => args.type_at(0),
                    _ => {
                        self.err(format!(
                            "Box::new_uninit with non-box destination {dest_ty}"
                        ));
                        return Err(());
                    }
                };
                self.uninit_shape(payload)?
            }
            BoxAssumeInitIntoVec => format!("{}[1][0]", a(0)),
            VecSetLen => {{
                // `len <= capacity` is guaranteed by the source; no-op.
                let _ = a(1);
                "undefined".to_string()
            }},
            BoxDeref | BoxDerefMut | RcDeref | ArcDeref => {
                // `&String` deref yields the plain string value.
                match tys.first().copied() {
                    Some(t) if self.is_str_cell_ty(t) => format!("{}[0]", a(0)),
                    _ => a(0),
                }
            }
            RcNew | RcClone | ArcNew | ArcClone => a(0),
            CellNew => format!("[{}]", a(0)),
            CellGet => format!("{}[0]", a(0)),
            CellSet => format!("({}[0]={})", a(0), a(1)),
            CellReplace => {
                format!("(()=>{{const __o={0}[0];{0}[0]={1};return __o;}})()", a(0), a(1))
            }
            RefCellNew => format!("[[0],[{}]]", a(0)),
            RefCellBorrow | RefCellBorrowMut | RefCellGetMut => format!("{}[1][0]", a(0)),
            RefDeref | RefMutDeref | RefMutDerefMut | RefClone => a(0),
            MemSwap => {
                format!("(()=>{{const __t={0};{0}={1};{1}=__t;}})()", a(0), a(1))
            }
            MemReplace => {
                format!("(()=>{{const __t={0};{0}={1};return __t;}})()", a(0), a(1))
            }
            MemForget => "undefined".to_string(),
            PtrNull => "null".to_string(),
            PtrIsNull => format!("({}===null)", a(0)),
            HintBlackBox | HintLikely => a(0),
            HintSpinLoop => "undefined".to_string(),
            CharLenUtf8 => format!("(({0})<128?1:(({0})<2048?2:(({0})<65536?3:4)))", a(0)),
            // Char predicates: delegate to the Unicode-aware runtime helpers.
            CharIsLowercase => format!("__char_is_lower({})", a(0)),
            CharIsUppercase => format!("__char_is_upper({})", a(0)),
            CharIsAlphabetic => format!("__char_is_alpha({})", a(0)),
            CharIsNumeric => format!("__char_is_numeric({})", a(0)),
            CharIsAlphanumeric => format!("__char_is_alnum({})", a(0)),
            CharIsWhitespace => format!("__char_is_ws({})", a(0)),
            CharIsControl => format!("__char_is_ctrl({})", a(0)),
            CharIsAsciiLowercase => format!("((({0})>=97&&({0})<=122))", a(0)),
            CharIsAsciiUppercase => format!("((({0})>=65&&({0})<=90))", a(0)),
            CharIsAsciiDigit => format!("__char_is_ascii_digit({})", a(0)),
            CharIsAsciiAlphabetic => format!("__char_is_ascii_letter({})", a(0)),
            CharIsAsciiAlphanumeric => {
                format!("(__char_is_ascii_digit({0})||__char_is_ascii_letter({0}))", a(0))
            }
            CharToAsciiLowercase => {
                format!("((({0})>=65&&({0})<=90)?({0})+32:({0}))", a(0))
            }
            CharToAsciiUppercase => {
                format!("((({0})>=97&&({0})<=122)?({0})-32:({0}))", a(0))
            }
            StrMatches => {
                // `s.matches(pat)`: an iterator over the matches. Only the
                // char/&str patterns are supported.
                let s = self.sval(&a(0), tys[0]);
                let p = self.sval(&a(1), tys[1]);
                format!("__it_new(__str_matches({s},{p}))")
            }
            FloatClamp => {
                // args: [value, min, max]. NaN bounds panic; a NaN value
                // passes through (both comparisons are false).
                format!("__fclamp({},{},{})", a(0), a(1), a(2))
            }
            // ---- HashMap (JS `Map` emulation; see `classify_map`) ----
            MapNew => "__map_new()".to_string(),
            MapInsert => {
                // Returns the previous value: `Option<V>`. `__map_set`
                // wraps the stored value in `[k, v]`; `__p[1]` is `v`.
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                format!(
                    "(()=>{{const __p=__map_set({},{},{});return __p===undefined?__none({dn}):__some({ds},__p[1]);}})()",
                    a(0),
                    a(1),
                    a(2)
                )
            }
            MapGet | MapGetMut | MapRemove => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let op = match op {
                    Shim::MapGet | Shim::MapGetMut => format!("__map_get({},{})", a(0), a(1)),
                    _ => format!("__map_del({},{})", a(0), a(1)),
                };
                format!("(()=>{{const __p={op};return __p===undefined?__none({dn}):__some({ds},__p[1]);}})()")
            }
            MapContainsKey => format!("__map_has({},{})", a(0), a(1)),
            MapLen => format!("__map_len({})", a(0)),
            MapIsEmpty => format!("({}.size===0)", a(0)),
            MapIter => format!("__it_new(__map_items({}))", a(0)),
            MapKeys => format!("__it_new(__map_items({}).map(__k=>__k[0]))", a(0)),
            MapValues => format!("__it_new(__map_items({}).map(__k=>__k[1]))", a(0)),
            StructuralEq | StructuralNe => {
                // Operands may mix `String` cells and `&str`; normalize cells
                // to their string before the structural comparison.
                let mut norm = |e: String, t: Option<Ty<'tcx>>| match t {
                    Some(t) if self.is_str_cell_ty(t) => format!("{e}[0]"),
                    _ => e,
                };
                let (e0, e1) = (
                    norm(a(0), tys.first().copied()),
                    norm(a(1), tys.get(1).copied()),
                );
                if matches!(op, Shim::StructuralNe) {
                    format!("(!__deep_eq({e0},{e1}))")
                } else {
                    format!("__deep_eq({e0},{e1})")
                }
            }
            PrimEq => format!("({}==={})", a(0), a(1)),
            PrimNe => format!("({}!=={})", a(0), a(1)),
            PrimAdd => {
                let (l, r) = (a(0), a(1));
                let t0 = tys.first().copied().unwrap_or(dest_ty);
                // `String + &str`: concatenate the cell contents. The rhs
                // may be a raw `&str` (not a cell), so unwrap per-side.
                if self.is_str_cell_ty(t0) {
                    let lv = format!("{l}[0]");
                    let rv = match tys.get(1).copied() {
                        Some(t) if self.is_str_cell_ty(t) => format!("{r}[0]"),
                        _ => r,
                    };
                    self.scell(&format!("({lv}+{rv})"))
                } else {
                    let lty = self.prim_self_ty(tys)?;
                    self.binop_expr(mir::BinOp::Add, l, r, lty, dest_ty)?
                }
            }
            PrimSub => {
                let (l, r) = (a(0), a(1));
                let lty = self.prim_self_ty(tys)?;
                self.binop_expr(mir::BinOp::Sub, l, r, lty, dest_ty)?
            }
            PrimMul => {
                let (l, r) = (a(0), a(1));
                let lty = self.prim_self_ty(tys)?;
                self.binop_expr(mir::BinOp::Mul, l, r, lty, dest_ty)?
            }
            PrimDiv => {
                let (l, r) = (a(0), a(1));
                let lty = self.prim_self_ty(tys)?;
                self.binop_expr(mir::BinOp::Div, l, r, lty, dest_ty)?
            }
            PrimRem => {
                let (l, r) = (a(0), a(1));
                let lty = self.prim_self_ty(tys)?;
                self.binop_expr(mir::BinOp::Rem, l, r, lty, dest_ty)?
            }
            PrimBitAnd => {
                let (l, r) = (a(0), a(1));
                let lty = self.prim_self_ty(tys)?;
                self.binop_expr(mir::BinOp::BitAnd, l, r, lty, dest_ty)?
            }
            PrimBitOr => {
                let (l, r) = (a(0), a(1));
                let lty = self.prim_self_ty(tys)?;
                self.binop_expr(mir::BinOp::BitOr, l, r, lty, dest_ty)?
            }
            PrimBitXor => {
                let (l, r) = (a(0), a(1));
                let lty = self.prim_self_ty(tys)?;
                self.binop_expr(mir::BinOp::BitXor, l, r, lty, dest_ty)?
            }
            PrimShl => {
                let (l, r) = (a(0), a(1));
                let lty = self.prim_self_ty(tys)?;
                self.binop_expr(mir::BinOp::Shl, l, r, lty, dest_ty)?
            }
            PrimShr => {
                let (l, r) = (a(0), a(1));
                let lty = self.prim_self_ty(tys)?;
                self.binop_expr(mir::BinOp::Shr, l, r, lty, dest_ty)?
            }
            PrimNeg => {
                let e = a(0);
                let ty = self.prim_self_ty(tys)?;
                self.unop_expr(mir::UnOp::Neg, e, ty)?
            }
            PrimNot => {
                let e = a(0);
                let ty = self.prim_self_ty(tys)?;
                self.unop_expr(mir::UnOp::Not, e, ty)?
            }
            IntMin => format!("(({0})<({1})?({0}):({1}))", a(0), a(1)),
            IntMax => format!("(({0})>({1})?({0}):({1}))", a(0), a(1)),
            IntClamp => format!("(({0})<({1})?({1}):(({0})>({2})?({2}):({0})))", a(0), a(1), a(2)),
            IntAbs => format!("(({0})<0?-({0}):({0}))", a(0)),
            IntWrappingAdd => {
                let (l, r) = (a(0), a(1));
                self.wrap_add(&l, &r, dest_ty)?
            }
            IntWrappingSub => {
                let (l, r) = (a(0), a(1));
                self.wrap_sub(&l, &r, dest_ty)?
            }
            IntSaturatingAdd => {
                let (l, r) = (a(0), a(1));
                self.sat_add(&l, &r, dest_ty)?
            }
            IntSaturatingSub => {
                let (l, r) = (a(0), a(1));
                self.sat_sub(&l, &r, dest_ty)?
            }
            IntCheckedAdd => {
                let (x, y) = (a(0), a(1));
                let ty = self.prim_self_ty(tys)?;
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                match repr_of(self.tcx, ty) {
                    Repr::Num(k) => {
                        let bits = num_bits(k);
                        let signed = self.num_signed(k);
                        let val = self.wrap_num("__c[0]".to_string(), k);
                        format!("(()=>{{const __c=__checked_add_num({x},{y},{bits},{signed});return __c[1]?__none({dn}):__some({ds},{val});}})()")
                    }
                    Repr::Big(k) => {
                        let bits = k.bits();
                        let signed = k.signed();
                        format!("(()=>{{const __c=__checked_add_big({x},{y},{bits},{signed});return __c[1]?__none({dn}):__some({ds},__c[0]);}})()")
                    }
                    _ => {
                        self.err(format!("checked_add on {ty} is not supported"));
                        return Err(());
                    }
                }
            }
            IntCheckedDiv => {
                let (x, y) = (a(0), a(1));
                let ty = self.prim_self_ty(tys)?;
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                match repr_of(self.tcx, ty) {
                    Repr::Num(k) => {
                        let signed = self.num_signed(k);
                        let (lo, _) = self.int_bounds(k);
                        let overflow = if signed {
                            format!("({x}==={lo}&&{y}===-1)")
                        } else {
                            "false".to_string()
                        };
                        let val = self.wrap_num(format!("Math.trunc({x}/{y})"), k);
                        format!("(()=>(({y}===0||{overflow})?__none({dn}):__some({ds},{val})))()")
                    }
                    Repr::Big(k) => {
                        let signed = k.signed();
                        let lo = big_int_lo(k);
                        let overflow = if signed {
                            format!("({x}==={lo}&&{y}===-1n)")
                        } else {
                            "false".to_string()
                        };
                        let val = self.wrap_big(format!("({x}/{y})"), k);
                        format!("(()=>(({y}===0n||{overflow})?__none({dn}):__some({ds},{val})))()")
                    }
                    _ => {
                        self.err(format!("checked_div on {ty} is not supported"));
                        return Err(());
                    }
                }
            }
            IntWrappingMul => {
                let (l, r) = (a(0), a(1));
                self.wrap_mul(&l, &r, dest_ty)?
            }
            IntWrappingNeg => {
                let x = a(0);
                self.wrap_neg(&x, dest_ty)?
            }
            IntWrappingShl => {
                let (l, r) = (a(0), a(1));
                self.wrap_shl(&l, &r, dest_ty)?
            }
            IntWrappingShr => {
                let (l, r) = (a(0), a(1));
                self.wrap_shr(&l, &r, dest_ty)?
            }
            IntSaturatingMul => {
                let (l, r) = (a(0), a(1));
                self.sat_mul(&l, &r, dest_ty)?
            }
            IntCheckedSub => {
                let (x, y) = (a(0), a(1));
                let ty = self.prim_self_ty(tys)?;
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                match repr_of(self.tcx, ty) {
                    Repr::Num(k) => {
                        let bits = num_bits(k);
                        let signed = self.num_signed(k);
                        let val = self.wrap_num("__c[0]".to_string(), k);
                        format!("(()=>{{const __c=__checked_sub_num({x},{y},{bits},{signed});return __c[1]?__none({dn}):__some({ds},{val});}})()")
                    }
                    Repr::Big(k) => {
                        let bits = k.bits();
                        let signed = k.signed();
                        format!("(()=>{{const __c=__checked_sub_big({x},{y},{bits},{signed});return __c[1]?__none({dn}):__some({ds},__c[0]);}})()")
                    }
                    _ => {
                        self.err(format!("checked_sub on {ty} is not supported"));
                        return Err(());
                    }
                }
            }
            IntCheckedMul => {
                let (x, y) = (a(0), a(1));
                let ty = self.prim_self_ty(tys)?;
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                match repr_of(self.tcx, ty) {
                    Repr::Num(k) => {
                        let bits = num_bits(k);
                        let signed = self.num_signed(k);
                        let val = self.wrap_num("__c[0]".to_string(), k);
                        format!("(()=>{{const __c=__checked_mul_num({x},{y},{bits},{signed});return __c[1]?__none({dn}):__some({ds},{val});}})()")
                    }
                    Repr::Big(k) => {
                        let bits = k.bits();
                        let signed = k.signed();
                        format!("(()=>{{const __c=__checked_mul_big({x},{y},{bits},{signed});return __c[1]?__none({dn}):__some({ds},__c[0]);}})()")
                    }
                    _ => {
                        self.err(format!("checked_mul on {ty} is not supported"));
                        return Err(());
                    }
                }
            }
            IntCheckedRem => {
                // `checked_rem` only fails on a zero divisor (the remainder
                // itself never overflows: `MIN % -1 == 0` fits).
                let (x, y) = (a(0), a(1));
                let ty = self.prim_self_ty(tys)?;
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                match repr_of(self.tcx, ty) {
                    Repr::Num(k) => {
                        let val = self.wrap_num(format!("({x}%{y})"), k);
                        format!("(()=>(({y}===0)?__none({dn}):__some({ds},{val})))()")
                    }
                    Repr::Big(k) => {
                        let val = self.wrap_big(format!("({x}%{y})"), k);
                        format!("(()=>(({y}===0n)?__none({dn}):__some({ds},{val})))()")
                    }
                    _ => {
                        self.err(format!("checked_rem on {ty} is not supported"));
                        return Err(());
                    }
                }
            }
            IntCheckedNeg => {
                // Signed: `None` only for `MIN`; unsigned: `None` unless zero.
                let x = a(0);
                let ty = self.prim_self_ty(tys)?;
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                match repr_of(self.tcx, ty) {
                    Repr::Num(k) => {
                        let val = self.wrap_num(format!("-({x})"), k);
                        if self.num_signed(k) {
                            let (lo, _) = self.int_bounds(k);
                            format!("(()=>(({x}==={lo})?__none({dn}):__some({ds},{val})))()")
                        } else {
                            format!("(()=>(({x}===0)?__some({ds},{val}):__none({dn})))()")
                        }
                    }
                    Repr::Big(k) => {
                        let val = self.wrap_big(format!("-({x})"), k);
                        if k.signed() {
                            let lo = big_int_lo(k);
                            format!("(()=>(({x}==={lo})?__none({dn}):__some({ds},{val})))()")
                        } else {
                            format!("(()=>(({x}===0n)?__some({ds},{val}):__none({dn})))()")
                        }
                    }
                    _ => {
                        self.err(format!("checked_neg on {ty} is not supported"));
                        return Err(());
                    }
                }
            }
            IntOverflowingAdd => {
                let (l, r) = (a(0), a(1));
                self.ov_add(&l, &r, dest_ty)?
            }
            IntOverflowingSub => {
                let (l, r) = (a(0), a(1));
                self.ov_sub(&l, &r, dest_ty)?
            }
            IntOverflowingMul => {
                let (l, r) = (a(0), a(1));
                self.ov_mul(&l, &r, dest_ty)?
            }
            IntOverflowingNeg => {
                // Signed: overflows only on `MIN`; unsigned: on any nonzero.
                let x = a(0);
                let ty = self.prim_self_ty(tys)?;
                match repr_of(self.tcx, ty) {
                    Repr::Num(k) => {
                        let w = self.wrap_num(format!("-({x})"), k);
                        if self.num_signed(k) {
                            let (lo, _) = self.int_bounds(k);
                            format!("[{w},({x}==={lo})]")
                        } else {
                            format!("[{w},({x}!==0)]")
                        }
                    }
                    Repr::Big(k) => {
                        let w = self.wrap_big(format!("-({x})"), k);
                        if k.signed() {
                            let lo = big_int_lo(k);
                            format!("[{w},({x}==={lo})]")
                        } else {
                            format!("[{w},({x}!==0n)]")
                        }
                    }
                    _ => {
                        self.err(format!("overflowing_neg on {ty} is not supported"));
                        return Err(());
                    }
                }
            }
            StrReplace => {
                let n = self.str_pattern(&a(1), tys.get(1).copied())?;
                let r = match tys.get(2).copied() {
                    Some(t) if self.is_str_cell_ty(t) => format!("{}[0]", a(2)),
                    _ => a(2),
                };
                self.scell(&format!("({}).split({n}).join({r})", a(0)))
            }
            StrStripPrefix => {
                let (ds, dn) = self.opt_discrs(dest_ty)?;
                let n = self.str_pattern(&a(1), tys.get(1).copied())?;
                let h = a(0);
                format!("(({h}).startsWith({n})?__some({ds},({h}).slice(({n}).length)):__none({dn}))")
            }
            StrTrimStartMatches => {
                let n = self.str_pattern(&a(1), tys.get(1).copied())?;
                format!("__str_trim_start_matches({},({n}))", a(0))
            }
            CharEncodeUtf8 => {
                format!("__char_encode_utf8_into({},{})", a(0), a(1))
            }
            StringFromChar => self.scell(&format!("String.fromCodePoint({})", a(0))),
            SpecExtend => format!("__vec_extend_iter({},{})", a(0), a(1)),
            CatchUnwind => {
                // args: [closure]. dest: Result<R, Box<dyn Any + Send>>.
                let (dok, derr) = self.res_discrs(dest_ty)?;
                let f_ty = self.subst(tys.first().copied().unwrap_or(dest_ty));
                // Canonical `catch_unwind(AssertUnwindSafe(f))`: the closure
                // env is the wrapper's field 0.
                let env = match self.unwind_safe_inner(f_ty) {
                    Some(_) => format!("{}[0]", a(0)),
                    None => a(0),
                };
                let f = self.closure_name_of_ty(f_ty)?;
                format!(
                    "(()=>{{try{{return __ok({dok},{f}({env}));}}catch(__e){{return __err({derr},__e);}}}})()"
                )
            }
            FmtEstimatedCapacity => "0".to_string(),
            FmtFromStr => {
                let t = match arg_ops.first() {
                    Some(sp) => sp.node.clone(),
                    None => {
                        self.err("from_str without argument".to_string());
                        return Err(());
                    }
                };
                // No placeholders: the payload is already the whole text.
                let bytes = self.template_bytes(t)?;
                self.scell(&constval::js_string(&bytes))
            }
            FmtFormat => {
                let fmt = self.peer_format_args(0, arg_ops)?.expr();
                self.scell(&fmt)
            }
            FmtWriteFmt => {
                // args: [&mut String, format_args]
                let fmt = self.peer_format_args(1, arg_ops)?.expr();
                format!("({0}[0]+={fmt})", a(0))
            }
            IoPrint => {
                let fmt = self.peer_format_args(0, arg_ops)?.expr();
                format!("console.log({fmt})")
            }
        };
        Ok(e)
    }

    /// `str::parse::<F>`: F from callee args; dest is Result<F, E>.
    fn parse_expr(
        &mut self,
        callee_args: ty::GenericArgsRef<'tcx>,
        s: &str,
        dest_ty: Ty<'tcx>,
    ) -> Result<String, ()> {
        let target = callee_args.type_at(0);
        let target = self.subst(target);
        let (dok, derr) = self.res_discrs(dest_ty)?;
        // Error payload type: dest Err variant field 0.
        let err_ty = match dest_ty.kind() {
            ty::Adt(def, args) if def.is_enum() => {
                let mut found = None;
                for (v, _) in def.discriminants(self.tcx) {
                    if def.variant(v).name.as_str() == "Err" {
                        let f = &def.variant(v).fields[FieldIdx::from_usize(0)];
                        found = Some(self.subst(tyutil::subst_unchecked(
                            self.tcx,
                            args,
                            self.tcx.type_of(f.did).skip_binder(),
                        )));
                        break;
                    }
                }
                match found {
                    Some(t) => t,
                    None => {
                        self.err("bad Result type".to_string());
                        return Err(());
                    }
                }
            }
            _ => {
                self.err(format!("parse must return Result, found {dest_ty}"));
                return Err(());
            }
        };
        // Build the error value for a failing parse of this target type.
        let err_val = self.parse_err_value(target, err_ty)?;
        match tyutil::repr_of(self.tcx, target) {
            Repr::Num(k) => {
                let (lo, hi) = self.int_bounds(k);
                let wrap = self.wrap_num("__v".to_string(), k);
                Ok(format!(
                    "(()=>{{const __r=__str_parse_int({s});if(!__r.ok)return __err({derr},{err_val});const __v=__r.val;if(__v<{lo}||__v>{hi})return __err({derr},{err_val});return __ok({dok},{wrap});}})()"
                ))
            }
            Repr::Big(k) => {
                let wrap = self.wrap_big("__v".to_string(), k);
                // Range check via BigInt bounds.
                let (lo, hi) = match k {
                    BigKind::I64 => ("-9223372036854775808n", "9223372036854775807n"),
                    BigKind::U64 => ("0n", "18446744073709551615n"),
                    BigKind::I128 => {
                        ("-170141183460469231731687303715884105728n", "170141183460469231731687303715884105727n")
                    }
                    BigKind::U128 => {
                        ("0n", "340282366920938463463374607431768211455n")
                    }
                };
                Ok(format!(
                    "(()=>{{const __r=__str_parse_int({s});if(!__r.ok)return __err({derr},{err_val});const __v=BigInt(__r.val);if(__v<{lo}||__v>{hi})return __err({derr},{err_val});return __ok({dok},{wrap});}})()"
                ))
            }
            Repr::Float => {
                let v = self.maybe_fround("__r.val".to_string(), target);
                Ok(format!(
                    "(()=>{{const __r=__str_parse_float({s});if(!__r.ok)return __err({derr},{err_val});return __ok({dok},{v});}})()"
                ))
            }
            Repr::Bool => Ok(format!(
                "(()=>{{const __r=__str_parse_bool({s});if(!__r.ok)return __err({derr},{err_val});return __ok({dok},__r.val);}})()"
            )),
            _ => {
                self.err(format!("parse to {target} is not supported"));
                Err(())
            }
        }
    }

    /// Build a best-effort parse-error value of type `err_ty`.
    fn parse_err_value(&mut self, target: Ty<'tcx>, err_ty: Ty<'tcx>) -> Result<String, ()> {
        // ParseIntError { kind: IntErrorKind }, ParseFloatError { kind }, ParseBoolError (unit).
        match err_ty.kind() {
            ty::Adt(def, args) => {
                let p = def_path(self.tcx, def.did());
                if p.contains("ParseBoolError") {
                    return Ok("[]".to_string());
                }
                // Struct with single `kind` field which is an enum.
                let variant = def.non_enum_variant();
                if variant.fields.len() != 1 {
                    self.err(format!("bad parse error type {err_ty}"));
                    return Err(());
                }
                let f = &variant.fields[FieldIdx::from_usize(0)];
                let kind_ty = self.subst(tyutil::subst_unchecked(
                    self.tcx,
                    args,
                    self.tcx.type_of(f.did).skip_binder(),
                ));
                // Pick a plausible kind discriminant by name.
                let is_float = matches!(tyutil::repr_of(self.tcx, target), Repr::Float);
                let want = if is_float { "Invalid" } else { "InvalidDigit" };
                let d = self
                    .enum_discr_by_name(kind_ty, want)
                    .or_else(|_| self.enum_discr_by_name(kind_ty, "Empty"))?;
                Ok(format!("[{{t:{d},v:[]}}]"))
            }
            ty::Tuple(tys) if tys.is_empty() => Ok("[]".to_string()),
            _ => {
                self.err(format!("bad parse error type {err_ty}"));
                Err(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Panics
// ---------------------------------------------------------------------------

impl<'tcx> Emitter<'tcx> {
    fn panic_expr(
        &mut self,
        k: PanicKind,
        arg_ops: &[Spanned<mir::Operand<'tcx>>],
        args: &[String],
    ) -> Result<String, ()> {
        let a = |i: usize| args.get(i).cloned().unwrap_or_else(|| "\"?\"".to_string());
        let e = match k {
            PanicKind::Str | PanicKind::Nounwind => format!("__panic({})", a(0)),
            PanicKind::Fmt => {
                let op0 = arg_ops.first().map(|s| &s.node);
                match op0 {
                    Some(op) => match self.peer_args_value(op.clone()) {
                        Ok(fmt) => format!("__panic({})", fmt.expr()),
                        Err(_) => return Err(()),
                    },
                    None => {
                        self.err("panic_fmt without arguments".to_string());
                        return Err(());
                    }
                }
            }
            PanicKind::BoundsCheck => {
                format!(
                    "__panic(\"index out of bounds: the len is \"+{}+\" but the index is \"+{})",
                    a(1),
                    a(0)
                )
            }
            PanicKind::AssertFailed => "__panic(\"assertion failed\")".to_string(),
            PanicKind::Misaligned => "__panic(\"misaligned pointer dereference\")".to_string(),
            PanicKind::Generic => "__panic(\"explicit panic\")".to_string(),
        };
        Ok(e)
    }
}

// ---------------------------------------------------------------------------
// Intrinsics
// ---------------------------------------------------------------------------

impl<'tcx> Emitter<'tcx> {
    fn intrinsic_expr(
        &mut self,
        op: Intrinsic,
        callee_args: ty::GenericArgsRef<'tcx>,
        args: &[String],
        tys: &[Ty<'tcx>],
        dest_ty: Ty<'tcx>,
    ) -> Result<String, ()> {
        use Intrinsic::*;
        let a = |i: usize| {
            args.get(i)
                .cloned()
                .unwrap_or_else(|| "undefined".to_string())
        };
        let t0 = callee_args
            .iter()
            .find_map(|arg| match arg.kind() {
                ty::GenericArgKind::Type(ty) => Some(ty),
                _ => None,
            })
            .unwrap_or(self.tcx.types.unit);
        let e = match op {
            Unreachable => "__unreachable()".to_string(),
            Abort => "__abort()".to_string(),
            SizeOf => match self.layout_size(t0) {
                Ok(n) => n.to_string(),
                Err(_) => return Err(()),
            },
            AlignOf => match self.layout_align(t0) {
                Ok(n) => n.to_string(),
                Err(_) => return Err(()),
            },
            NeedsDrop => {
                if t0.needs_drop(self.tcx, mono_env()) {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            VariantCount => match t0.kind() {
                ty::Adt(def, _) => def.variants().len().to_string(),
                _ => {
                    self.err(format!("variant_count of {t0}"));
                    return Err(());
                }
            },
            DiscriminantValue => format!("{}.t", a(0)),
            PtrGuaranteedCmp => format!("(({})===({})?1:0)", a(0), a(1)),
            WrappingAdd | UncheckedAdd => self.wrap_add(&a(0), &a(1), dest_ty)?,
            WrappingSub | UncheckedSub => self.wrap_sub(&a(0), &a(1), dest_ty)?,
            WrappingMul | UncheckedMul => self.wrap_mul(&a(0), &a(1), dest_ty)?,
            SaturatingAdd => self.sat_add(&a(0), &a(1), dest_ty)?,
            SaturatingSub => self.sat_sub(&a(0), &a(1), dest_ty)?,
            SaturatingMul => self.sat_mul(&a(0), &a(1), dest_ty)?,
            UncheckedNeg => self.wrap_neg(&a(0), dest_ty)?,
            UncheckedShl => self.wrap_shl(&a(0), &a(1), dest_ty)?,
            UncheckedShr => self.wrap_shr(&a(0), &a(1), dest_ty)?,
            UncheckedDiv => self.wrap_div(&a(0), &a(1), dest_ty)?,
            UncheckedRem => self.wrap_rem(&a(0), &a(1), dest_ty)?,
            IntegerMin => format!("(({0})<({1})?({0}):({1}))", a(0), a(1)),
            IntegerMax => format!("(({0})>({1})?({0}):({1}))", a(0), a(1)),
            AddWithOverflow => self.ov_add(&a(0), &a(1), dest_ty)?,
            SubWithOverflow => self.ov_sub(&a(0), &a(1), dest_ty)?,
            MulWithOverflow => self.ov_mul(&a(0), &a(1), dest_ty)?,
            RotateLeft => self.rot(&a(0), &a(1), dest_ty, true)?,
            RotateRight => self.rot(&a(0), &a(1), dest_ty, false)?,
            Ctpop => self.ctpop(&a(0), dest_ty)?,
            Ctlz => self.ctlz(&a(0), dest_ty)?,
            Cttz => self.cttz(&a(0), dest_ty)?,
            Bswap => self.bswap(&a(0), dest_ty)?,
            Bitreverse => self.bitreverse(&a(0), dest_ty)?,
            Float(f) => self.float_expr(f, args, dest_ty)?,
            FloatArith(fa) => self.float_arith_expr(fa, args)?,
            Offset => {
                self.err("pointer offset is not supported".to_string());
                return Err(());
            }
            Transmute => {
                self.err("transmute is not supported".to_string());
                return Err(());
            }
            DropInPlace => {
                // Pointee must not have user destructors (same rule as Drop).
                let pt = self.subst(t0);
                self.check_significant_drop(pt)?;
                "undefined".to_string()
            }
            UbChecks => {
                if self.tcx.sess.ub_checks() {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            OverflowChecks => {
                if self.tcx.sess.overflow_checks() {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Likely | BlackBox => a(0),
            CallerLocation => "[\"unknown\",0,0]".to_string(),
            ConstEvalSelect => {
                // args: (tuple_of_args, const_fn, runtime_fn). Call runtime_fn(...tuple).
                if args.len() != 3 {
                    self.err("bad const_eval_select shape".to_string());
                    return Err(());
                }
                // Third operand must be a function item.
                let rfn = self.fn_operand_value(tys[2], &args[2])?;
                format!("{rfn}(...{})", a(0))
            }
            PtrMetadata => match tys.first().map(|t| repr_of(self.tcx, *t)) {
                Some(Repr::Array) => format!("({}.length)", a(0)),
                Some(Repr::Str) | Some(Repr::BufStr) => format!("__str_len({})", a(0)),
                _ => {
                    self.err("pointer metadata is not supported here".to_string());
                    return Err(());
                }
            },
            MoveValInit => format!("({}={})", a(0), a(1)),
            Fence => "undefined".to_string(),
            IsValStaticallyKnown => "false".to_string(),
            AssertZeroValid => "undefined".to_string(),
            UnsupportedInt(msg) => {
                self.err(msg.to_string());
                return Err(());
            }
        };
        Ok(e)
    }

    /// A function-typed operand rendered as a callable reference (for
    /// `const_eval_select`'s runtime fn).
    fn fn_operand_value(&mut self, ty: Ty<'tcx>, expr: &str) -> Result<String, ()> {
        match ty.kind() {
            ty::FnDef(did, fargs) => {
                let fargs = subst_args(self.tcx, self.args, fargs.skip_binder());
                self.fn_value_of(*did, fargs)
            }
            _ => Ok(expr.to_string()),
        }
    }

    fn layout_size(&mut self, ty: Ty<'tcx>) -> Result<u64, ()> {
        let ty = self.subst(ty);
        match self.tcx.layout_of(mono_env().as_query_input(ty)) {
            Ok(layout) => Ok(layout.size.bytes()),
            Err(_) => {
                self.err(format!("cannot compute size of {ty}"));
                Err(())
            }
        }
    }

    fn layout_align(&mut self, ty: Ty<'tcx>) -> Result<u64, ()> {
        let ty = self.subst(ty);
        match self.tcx.layout_of(mono_env().as_query_input(ty)) {
            Ok(layout) => Ok(layout.align.abi.bytes()),
            Err(_) => {
                self.err(format!("cannot compute alignment of {ty}"));
                Err(())
            }
        }
    }

    // arithmetic builders dispatching on the destination representation
    fn arith2(
        &mut self,
        l: &str,
        r: &str,
        dest_ty: Ty<'tcx>,
        num: fn(&Self, NumKind, &str, &str) -> String,
        big: fn(&Self, BigKind, &str, &str) -> String,
    ) -> Result<String, ()> {
        match repr_of(self.tcx, dest_ty) {
            Repr::Num(k) => Ok(num(self, k, l, r)),
            Repr::Big(k) => Ok(big(self, k, l, r)),
            _ => {
                self.err(format!("arithmetic on {dest_ty} is not supported"));
                Err(())
            }
        }
    }

    fn wrap_add(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        self.arith2(
            l,
            r,
            d,
            |s, k, l, r| s.wrap_num(format!("({l}+{r})"), k),
            |s, k, l, r| s.wrap_big(format!("({l}+{r})"), k),
        )
    }
    fn wrap_sub(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        self.arith2(
            l,
            r,
            d,
            |s, k, l, r| s.wrap_num(format!("({l}-{r})"), k),
            |s, k, l, r| s.wrap_big(format!("({l}-{r})"), k),
        )
    }
    fn wrap_mul(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        self.arith2(
            l,
            r,
            d,
            |s, k, l, r| s.wrap_num(format!("({l}*{r})"), k),
            |s, k, l, r| s.wrap_big(format!("({l}*{r})"), k),
        )
    }
    fn wrap_neg(&mut self, x: &str, d: Ty<'tcx>) -> Result<String, ()> {
        match repr_of(self.tcx, d) {
            // Parenthesize: a const-folded negative receiver (`-5`) would
            // otherwise emit the invalid prefix-decrement `--5`.
            Repr::Num(k) => Ok(self.wrap_num(format!("-({x})"), k)),
            Repr::Big(k) => Ok(self.wrap_big(format!("-({x})"), k)),
            _ => {
                self.err(format!("negation of {d} is not supported"));
                Err(())
            }
        }
    }
    fn wrap_shl(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        match repr_of(self.tcx, d) {
            Repr::Num(k) => Ok(self.wrap_num(format!("({l}<<{r})"), k)),
            // Shift amounts are plain numbers in JS, so mask with a number.
            Repr::Big(k) => Ok(self.wrap_big(format!("({l}<<BigInt(({r})&{}))", k.bits() - 1), k)),
            _ => {
                self.err(format!("shift of {d} is not supported"));
                Err(())
            }
        }
    }
    fn wrap_shr(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        match repr_of(self.tcx, d) {
            Repr::Num(k) => {
                if self.num_signed(k) {
                    Ok(self.wrap_num(format!("({l}>>{r})"), k))
                } else {
                    Ok(format!("({l}>>>{r})"))
                }
            }
            Repr::Big(k) => {
                if k.signed() {
                    Ok(self.wrap_big(format!("({l}>>BigInt(({r})&{}))", k.bits() - 1), k))
                } else {
                    Ok(format!("__lshr_big({l},{r},{})", k.bits()))
                }
            }
            _ => {
                self.err(format!("shift of {d} is not supported"));
                Err(())
            }
        }
    }
    fn wrap_div(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        match repr_of(self.tcx, d) {
            Repr::Num(k) => Ok(self.wrap_num(format!("Math.trunc({l}/{r})"), k)),
            Repr::Big(k) => Ok(self.wrap_big(format!("({l}/{r})"), k)),
            _ => {
                self.err(format!("division of {d} is not supported"));
                Err(())
            }
        }
    }
    fn wrap_rem(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        match repr_of(self.tcx, d) {
            Repr::Num(k) => Ok(self.wrap_num(format!("({l}%{r})"), k)),
            Repr::Big(k) => Ok(self.wrap_big(format!("({l}%{r})"), k)),
            _ => {
                self.err(format!("remainder of {d} is not supported"));
                Err(())
            }
        }
    }
    fn ov_add(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        match d.kind() {
            ty::Tuple(tys) if tys.len() == 2 => {}
            _ => {
                self.err("bad with-overflow return type".to_string());
                return Err(());
            }
        }
        // Element type of the tuple.
        let ety = match d.kind() {
            ty::Tuple(tys) => tys[0],
            _ => unreachable!(),
        };
        match repr_of(self.tcx, ety) {
            Repr::Num(k) => Ok(self.with_overflow_num("|", "+", l, r, k)),
            Repr::Big(k) => Ok(self.with_overflow_big("+", l, r, k)),
            _ => {
                self.err(format!("checked arithmetic on {d} is not supported"));
                Err(())
            }
        }
    }
    fn ov_sub(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        let ety = match d.kind() {
            ty::Tuple(tys) if tys.len() == 2 => tys[0],
            _ => {
                self.err("bad with-overflow return type".to_string());
                return Err(());
            }
        };
        match repr_of(self.tcx, ety) {
            Repr::Num(k) => Ok(self.with_overflow_num("|", "-", l, r, k)),
            Repr::Big(k) => Ok(self.with_overflow_big("-", l, r, k)),
            _ => {
                self.err(format!("checked arithmetic on {d} is not supported"));
                Err(())
            }
        }
    }
    fn ov_mul(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        let ety = match d.kind() {
            ty::Tuple(tys) if tys.len() == 2 => tys[0],
            _ => {
                self.err("bad with-overflow return type".to_string());
                return Err(());
            }
        };
        match repr_of(self.tcx, ety) {
            Repr::Num(k) => Ok(self.with_overflow_num("|", "*", l, r, k)),
            Repr::Big(k) => Ok(self.with_overflow_big("*", l, r, k)),
            _ => {
                self.err(format!("checked arithmetic on {d} is not supported"));
                Err(())
            }
        }
    }

    fn sat_add(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        self.sat_op(l, r, d, "+")
    }
    fn sat_sub(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        self.sat_op(l, r, d, "-")
    }
    fn sat_mul(&mut self, l: &str, r: &str, d: Ty<'tcx>) -> Result<String, ()> {
        self.sat_op(l, r, d, "*")
    }
    fn sat_op(&mut self, l: &str, r: &str, d: Ty<'tcx>, op: &str) -> Result<String, ()> {
        match repr_of(self.tcx, d) {
            Repr::Num(k) => {
                let (lo, hi) = self.int_bounds(k);
                Ok(format!("Math.min(Math.max(({l}{op}{r}),{lo}),{hi})"))
            }
            Repr::Big(k) => {
                let b = k.bits();
                let (lo, hi) = if k.signed() {
                    (
                        format!("-(1n<<{}n)", b - 1),
                        format!("((1n<<{}n)-1n)", b - 1),
                    )
                } else {
                    ("0n".to_string(), format!("((1n<<{b}n)-1n)"))
                };
                Ok(format!(
                    "(()=>{{const __s=({l}{op}{r});return __s<{lo}?{lo}:(__s>{hi}?{hi}:__s);}})()"
                ))
            }
            _ => {
                self.err(format!("saturating arithmetic on {d} is not supported"));
                Err(())
            }
        }
    }

    fn int_bits(&mut self, ty: Ty<'tcx>) -> Result<(u32, bool), ()> {
        match repr_of(self.tcx, ty) {
            Repr::Num(k) => {
                let b = match k {
                    NumKind::I8 | NumKind::U8 => 8,
                    NumKind::I16 | NumKind::U16 => 16,
                    NumKind::I32 | NumKind::U32 | NumKind::Usize | NumKind::Isize => 32,
                    NumKind::Char => 32,
                };
                Ok((b, self.num_signed(k)))
            }
            Repr::Big(k) => Ok((k.bits(), k.signed())),
            _ => {
                self.err(format!("bit operation on {ty} is not supported"));
                Err(())
            }
        }
    }

    fn rot(&mut self, x: &str, n: &str, d: Ty<'tcx>, left: bool) -> Result<String, ()> {
        let (bits, signed) = self.int_bits(d)?;
        if bits == 32 {
            return Ok(if left {
                format!("__rotl32({x},{n})")
            } else {
                format!("__rotr32({x},{n})")
            });
        }
        match repr_of(self.tcx, d) {
            Repr::Big(k) => Ok(if left {
                format!("__rotl_big({x},{n},{})", k.bits())
            } else {
                format!("__rotr_big({x},{n},{})", k.bits())
            }),
            _ => {
                // Narrow ints: rotate within width.
                let m = (1u64 << bits) - 1;
                let (op1, op2) = if left { ("<<", ">>>") } else { (">>>", "<<") };
                let _ = signed;
                Ok(format!("((((({x}&{m}){op1}({n}&{bits_minus}))|(({x}&{m}){op2}({bits}-(({n})&{bits_minus}))))&{m})", bits_minus = bits - 1, bits = bits))
            }
        }
    }

    fn ctpop(&mut self, x: &str, d: Ty<'tcx>) -> Result<String, ()> {
        match repr_of(self.tcx, d) {
            Repr::Num(_) => Ok(format!("__popcnt32({x})")),
            Repr::Big(_) => Ok(format!("__popcnt_big({x})")),
            _ => {
                self.err(format!("ctpop of {d} is not supported"));
                Err(())
            }
        }
    }
    fn ctlz(&mut self, x: &str, d: Ty<'tcx>) -> Result<String, ()> {
        match repr_of(self.tcx, d) {
            Repr::Num(_) => Ok(format!("Math.clz32({x})")),
            Repr::Big(k) => Ok(format!("__ctlz_big({x},{})", k.bits())),
            _ => {
                self.err(format!("ctlz of {d} is not supported"));
                Err(())
            }
        }
    }
    fn cttz(&mut self, x: &str, d: Ty<'tcx>) -> Result<String, ()> {
        match repr_of(self.tcx, d) {
            Repr::Num(_) => Ok(format!("(({x}===0)?32:(31-Math.clz32({x}&-{x})))")),
            Repr::Big(k) => Ok(format!("__cttz_big({x},{})", k.bits())),
            _ => {
                self.err(format!("cttz of {d} is not supported"));
                Err(())
            }
        }
    }
    fn bswap(&mut self, x: &str, d: Ty<'tcx>) -> Result<String, ()> {
        let (bits, _) = self.int_bits(d)?;
        match bits {
            8 => Ok(format!("({x})")),
            16 => Ok(format!("((((({x})&255)<<8)|((({x})>>>8)&255)))")),
            32 => Ok(format!("__bswap32({x})")),
            _ => Ok(format!("__bswap_big({x},{bits})")),
        }
    }
    fn bitreverse(&mut self, x: &str, d: Ty<'tcx>) -> Result<String, ()> {
        let (bits, _) = self.int_bits(d)?;
        match bits {
            8 => Ok(format!("__bitrev32({x})>>>24")),
            16 => Ok(format!("__bitrev32({x})>>>16")),
            32 => Ok(format!("__bitrev32({x})")),
            _ => Ok(format!("__bitrev_big({x},{bits})")),
        }
    }

    fn float_expr(&mut self, f: FloatOp, args: &[String], dest_ty: Ty<'tcx>) -> Result<String, ()> {
        use FloatOp::*;
        let a = |i: usize| args.get(i).cloned().unwrap_or_else(|| "0".to_string());
        let e = match f {
            Sqrt => format!("Math.sqrt({})", a(0)),
            Sin => format!("Math.sin({})", a(0)),
            Cos => format!("Math.cos({})", a(0)),
            Tan => format!("Math.tan({})", a(0)),
            Asin => format!("Math.asin({})", a(0)),
            Acos => format!("Math.acos({})", a(0)),
            Atan => format!("Math.atan({})", a(0)),
            Atan2 => format!("Math.atan2({},{})", a(0), a(1)),
            Sinh => format!("Math.sinh({})", a(0)),
            Cosh => format!("Math.cosh({})", a(0)),
            Tanh => format!("Math.tanh({})", a(0)),
            Asinh => format!("Math.asinh({})", a(0)),
            Acosh => format!("Math.acosh({})", a(0)),
            Atanh => format!("Math.atanh({})", a(0)),
            Exp => format!("Math.exp({})", a(0)),
            Exp2 => format!("Math.pow(2,{})", a(0)),
            Log => format!("Math.log({})", a(0)),
            Log2 => format!("Math.log2({})", a(0)),
            Log10 => format!("Math.log10({})", a(0)),
            Powf => format!("Math.pow({},{})", a(0), a(1)),
            Powi => format!("Math.pow({},{})", a(0), a(1)),
            Floor => format!("Math.floor({})", a(0)),
            Ceil => format!("Math.ceil({})", a(0)),
            Trunc => format!("Math.trunc({})", a(0)),
            // Rust rounds half away from zero; Math.round rounds half up.
            Round => format!("__fround_away({})", a(0)),
            RoundTiesEven => format!("__round_ties_even({})", a(0)),
            Fract => format!("({0}-Math.trunc({0}))", a(0)),
            Abs => format!("Math.abs({})", a(0)),
            // IEEE 754-2019 minimumNumber/maximumNumber: a lone NaN
            // yields the other argument (Math.min/max propagate NaN).
            Min => format!("__fmin({},{})", a(0), a(1)),
            Max => format!("__fmax({},{})", a(0), a(1)),
            // Propagating minimum/maximum: any NaN yields NaN, and -0.0 is
            // ordered below +0.0.
            Minimum => format!("__fminimum({},{})", a(0), a(1)),
            Maximum => format!("__fmaximum({},{})", a(0), a(1)),
            MulAdd => format!("({0}*{1}+{2})", a(0), a(1), a(2)),
            Fma => format!("({0}*{1}+{2})", a(0), a(1), a(2)),
            Copysign => format!("__copysign({},{})", a(0), a(1)),
            Pow => format!("Math.pow({},{})", a(0), a(1)),
            Log1p => format!("Math.log1p({})", a(0)),
            ExpM1 => format!("Math.expm1({})", a(0)),
            Hypot => format!("Math.hypot({},{})", a(0), a(1)),
            Cbrt => format!("Math.cbrt({})", a(0)),
            IsNan => format!("Number.isNaN({})", a(0)),
            IsFinite => format!("Number.isFinite({})", a(0)),
            IsInfinite => format!("({0}===Infinity||{0}===-Infinity)", a(0)),
            IsSignPositive => format!("({0}>0||({0}===0&&1/{0}===Infinity))", a(0)),
            IsSignNegative => format!("({0}<0||({0}===0&&1/{0}===-Infinity))", a(0)),
            // NaN falls through to `x` (stays NaN); ±0 fall through to `x`.
            Signum => format!("({0}>0?1:{0}<0?-1:{0})", a(0)),
            Recip => format!("(1/{})", a(0)),
            ToDegrees => format!("({}*180/Math.PI)", a(0)),
            ToRadians => format!("({}*Math.PI/180)", a(0)),
            LogBase => format!("(Math.log({0})/Math.log({1}))", a(0), a(1)),
        };
        Ok(self.maybe_fround(e, dest_ty))
    }

    fn float_arith_expr(&mut self, fa: FloatArithOp, args: &[String]) -> Result<String, ()> {
        let a = |i: usize| args.get(i).cloned().unwrap_or_else(|| "0".to_string());
        Ok(match fa {
            FloatArithOp::Add => format!("({}+{})", a(0), a(1)),
            FloatArithOp::Sub => format!("({}-{})", a(0), a(1)),
            FloatArithOp::Mul => format!("({}*{})", a(0), a(1)),
            FloatArithOp::Div => format!("({}/{})", a(0), a(1)),
            FloatArithOp::Rem => format!("({}%{})", a(0), a(1)),
            FloatArithOp::Neg => format!("(-{})", a(0)),
        })
    }

    /// Drop-glue significance check shared by `Drop` and `drop_in_place`.
    /// For user types the glue itself is emittable (it is a normal function
    /// body), so this only rejects the cases with no JS equivalent.
    fn check_significant_drop(&mut self, ty: Ty<'tcx>) -> Result<(), ()> {
        if ty.has_significant_drop(self.tcx, mono_env()) {
            let (path, in_std) = match ty.kind() {
                ty::Adt(def, _) => (
                    def_path(self.tcx, def.did()),
                    tyutil::is_std_item(self.tcx, def.did()),
                ),
                _ => (format!("{ty}"), false),
            };
            if recognize::noop_drop(&path, in_std) {
                return Ok(());
            }
            self.err(format!(
                "cannot drop {ty}: user destructors are not supported \
                 (deallocation itself is a no-op under GC)"
            ));
            return Err(());
        }
        Ok(())
    }

    /// Resolve `<T as Drop>::drop` for a local type (its destructor).
    fn drop_fn_instance(&mut self, ty: Ty<'tcx>) -> Option<ty::Instance<'tcx>> {
        let trait_did = self.tcx.lang_items().drop_trait()?;
        let method = self
            .tcx
            .associated_items(trait_did)
            .filter_by_name_unhygienic(rustc_span::symbol::sym::drop)
            .next()?;
        let args = self.tcx.mk_args(&[ty.into()]);
        match ty::Instance::try_resolve(self.tcx, mono_env(), method.def_id, args) {
            Ok(Some(inst)) => Some(inst),
            _ => None,
        }
    }

    /// `destructor(value);` for `drop(place)`, or `None` when the value
    /// needs no glue at all. The caller emits the control flow.
    fn emit_drop_call(&mut self, place: mir::Place<'tcx>) -> Result<Option<String>, ()> {
        let ty = self.place_ty(place)?;
        if !ty.has_significant_drop(self.tcx, mono_env()) {
            return Ok(None);
        }
        let (path, in_std) = match ty.kind() {
            ty::Adt(def, _) => (
                def_path(self.tcx, def.did()),
                tyutil::is_std_item(self.tcx, def.did()),
            ),
            _ => (format!("{ty}"), false),
        };
        if recognize::noop_drop(&path, in_std) {
            return Ok(None);
        }
        let Some(glue) = self.drop_fn_instance(ty) else {
            // No `Drop` impl exists: the significance is field-driven only
            // (`Box`/`Vec` deallocation, refcount ticks), which are all
            // no-ops under GC. There is no user code to run.
            return Ok(None);
        };
        let name = fn_name(self.tcx, &self.crate_name, &glue);
        let (expr, _) = self.place_expr(place)?;
        // The destructor borrows, and references are transparent here.
        Ok(Some(format!("{name}({expr});\n")))
    }
}

// ---------------------------------------------------------------------------
// format_args! peering (fmt::Arguments::new template decoding)
// ---------------------------------------------------------------------------

/// A peered `fmt::Arguments` value.
pub(crate) enum PeerFmt {
    /// Bytecode template + rendered `[desc, val, kind]` arg exprs, from
    /// `Arguments::new`.
    Bytes(String, Vec<String>),
    /// A JS string expression holding the already-rendered text, from
    /// `Arguments::from_str_nonconst` (rustc const-folds `write!`/`panic!`
    /// whose template has no formatting options and whose args are all
    /// literals).
    Lit(String),
}

impl PeerFmt {
    /// The JS expression evaluating to the formatted string.
    fn expr(&self) -> String {
        match self {
            PeerFmt::Bytes(bytes, args) => {
                format!("__fmt_template({bytes},[{}])", args.join(","))
            }
            PeerFmt::Lit(s) => s.clone(),
        }
    }
}

impl<'tcx> Emitter<'tcx> {
    /// Find the unique `local = <rvalue>` assignment in the body (clone).
    /// Returns `Ok(None)` if there is no plain assignment (the value may
    /// come from a `Call` terminator — see `find_call_to`).
    fn find_assign(&mut self, local: mir::Local) -> Result<Option<mir::Rvalue<'tcx>>, ()> {
        let body = self.body.clone();
        let mut found = None;
        for bb in body.basic_blocks.iter_enumerated() {
            for stmt in &bb.1.statements {
                if let mir::StatementKind::Assign(place_rv) = &stmt.kind {
                    let (place, rv): &(mir::Place<'tcx>, mir::Rvalue<'tcx>) = &**place_rv;
                    if place.local == local && place.projection.is_empty() {
                        if found.is_some() {
                            self.err("format argument assigned twice".to_string());
                            return Err(());
                        }
                        found = Some(rv.clone());
                    }
                }
            }
        }
        Ok(found)
    }

    /// Find the `Call` terminator writing to `local`, if any.
    fn find_call_to(
        &mut self,
        local: mir::Local,
    ) -> Result<Option<(mir::Operand<'tcx>, Vec<mir::Operand<'tcx>>)>, ()> {
        let body = self.body.clone();
        for bb in body.basic_blocks.iter_enumerated() {
            if let Some(term) = &bb.1.terminator {
                if let mir::TerminatorKind::Call {
                    func,
                    args,
                    destination,
                    ..
                } = &term.kind
                {
                    if destination.local == local && destination.projection.is_empty() {
                        let a = args.iter().map(|s| s.node.clone()).collect();
                        return Ok(Some((func.clone(), a)));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Peer `Arguments::new(template_op, args_op)` → (bytes literal, arg exprs).
    fn peer_format_args(
        &mut self,
        arg_idx: usize,
        arg_ops: &[Spanned<mir::Operand<'tcx>>],
    ) -> Result<PeerFmt, ()> {
        let op = arg_ops.get(arg_idx).map(|s| &s.node);
        let op = match op {
            Some(o) => o.clone(),
            None => {
                self.err("format call without arguments".to_string());
                return Err(());
            }
        };
        self.peer_args_value(op)
    }

    /// Follow an `Arguments` value back to its `Arguments::new` call.
    fn peer_args_value(&mut self, op: mir::Operand<'tcx>) -> Result<PeerFmt, ()> {
        let op = self.strip_deref(op)?;
        match op {
            mir::Operand::Copy(p) | mir::Operand::Move(p) if p.projection.is_empty() => {
                if let Some((func, call_args)) = self.find_call_to(p.local)? {
                    return self.peer_new_call(&func, &call_args);
                }
                match self.find_assign(p.local)? {
                    Some(mir::Rvalue::Use(inner, _)) => self.peer_args_value(inner),
                    Some(rv) => {
                        self.err(format!("cannot peer format args through {rv:?}"));
                        Err(())
                    }
                    None => {
                        self.err("format arguments have no visible construction".to_string());
                        Err(())
                    }
                }
            }
            _ => {
                self.err("format arguments must be a plain local".to_string());
                Err(())
            }
        }
    }

    /// Handle the constructing call: `Arguments::new` or `from_str`.
    fn peer_new_call(
        &mut self,
        func: &mir::Operand<'tcx>,
        call_args: &[mir::Operand<'tcx>],
    ) -> Result<PeerFmt, ()> {
        let target = self.resolve_target(func)?;
        match target {
            CallTarget::FmtNew => {
                if call_args.len() != 2 {
                    self.err("bad Arguments::new shape".to_string());
                    return Err(());
                }
                let bytes = self.template_bytes(call_args[0].clone())?;
                let fargs = self.peer_arg_array(call_args[1].clone())?;
                return Ok(PeerFmt::Bytes(
                    format!(
                        "[{}]",
                        bytes
                            .iter()
                            .map(|b| b.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    ),
                    fargs,
                ));
            }
            CallTarget::Shim {
                op: Shim::FmtFromStr,
                ..
            } => {
                let t = match call_args.first() {
                    Some(o) => o.clone(),
                    None => {
                        self.err("from_str without argument".to_string());
                        return Err(());
                    }
                };
                // `from_str` has no arguments, so its payload is already the
                // fully rendered text: emit it as a plain string literal
                // rather than as bytecode.
                let bytes = self.template_bytes(t)?;
                Ok(PeerFmt::Lit(constval::js_string(&bytes)))
            }
            CallTarget::Direct { name } => {
                // Must be `core::fmt::Arguments::new`.
                let is_new = name.contains("Arguments") || self.is_arguments_new(func);
                if !is_new {
                    self.err("format arguments must come from format_args!".to_string());
                    return Err(());
                }
                if call_args.len() != 2 {
                    self.err("bad Arguments::new shape".to_string());
                    return Err(());
                }
                let bytes = self.template_bytes(call_args[0].clone())?;
                let fargs = self.peer_arg_array(call_args[1].clone())?;
                Ok(PeerFmt::Bytes(
                    format!(
                        "[{}]",
                        bytes
                            .iter()
                            .map(|b| b.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    ),
                    fargs,
                ))
            }
            _ => {
                self.err("format arguments must come from format_args!".to_string());
                Err(())
            }
        }
    }

    /// `true` if `func` names `core::fmt::Arguments::new`.
    fn is_arguments_new(&mut self, func: &mir::Operand<'tcx>) -> bool {
        match func {
            mir::Operand::Constant(c) => match self.subst(c.const_.ty()).kind() {
                ty::FnDef(did, _) => {
                    crate::tyutil::ends_seg(&def_path(self.tcx, *did), &["Arguments", "new"])
                }
                _ => false,
            },
            _ => false,
        }
    }

    /// Resolve the `&[Argument; M]` operand into per-arg `[desc, val, kind]` exprs.
    fn peer_arg_array(&mut self, op: mir::Operand<'tcx>) -> Result<Vec<String>, ()> {
        let op = self.strip_deref(op)?;
        match op {
            mir::Operand::Copy(p) | mir::Operand::Move(p) if p.projection.is_empty() => {
                match self.find_assign(p.local)? {
                    Some(mir::Rvalue::Ref(_, _, inner)) => {
                        self.peer_arg_array(mir::Operand::Copy(inner))
                    }
                    Some(mir::Rvalue::Use(inner, _)) => self.peer_arg_array(inner),
                    Some(mir::Rvalue::Aggregate(box_kind, elems)) => match *box_kind {
                        mir::AggregateKind::Array(_) => {
                            let mut out = Vec::with_capacity(elems.len());
                            for el in elems.iter() {
                                out.push(self.peer_arg_elem(el.clone())?);
                            }
                            Ok(out)
                        }
                        _ => {
                            self.err("bad format args array".to_string());
                            Err(())
                        }
                    },
                    Some(rv) => {
                        self.err(format!("cannot peer format args through {rv:?}"));
                        Err(())
                    }
                    None => {
                        self.err("format args array has no visible construction".to_string());
                        Err(())
                    }
                }
            }
            _ => {
                self.err("format args array must be a plain local".to_string());
                Err(())
            }
        }
    }

    /// One `Argument::new_*(value)` element → `[desc, val, kind]` expr.
    fn peer_arg_elem(&mut self, op: mir::Operand<'tcx>) -> Result<String, ()> {
        match op {
            mir::Operand::Copy(p) | mir::Operand::Move(p) if p.projection.is_empty() => {
                match self.find_call_to(p.local)? {
                    Some((func, call_args)) => {
                        // NOTE: do not resolve_target here: ArgNew ctors
                        // intentionally fail resolution (peer-only).
                        self.peer_arg_new(&func, &call_args)
                    }
                    None => {
                        self.err("format arg has no visible construction".to_string());
                        Err(())
                    }
                }
            }
            _ => {
                self.err("format arg must be a plain local".to_string());
                Err(())
            }
        }
    }

    fn peer_arg_new(
        &mut self,
        func: &mir::Operand<'tcx>,
        call_args: &[mir::Operand<'tcx>],
    ) -> Result<String, ()> {
        let (kind, targ_ty) = match func {
            mir::Operand::Constant(c) => {
                let ty = self.subst(c.const_.ty());
                match ty.kind() {
                    ty::FnDef(did, fargs) => {
                        let path = def_path(self.tcx, *did);
                        let in_std = tyutil::is_std_item(self.tcx, *did);
                        let k = match recognize::classify_arg_new(&path, in_std) {
                            Some(k) => k,
                            None => {
                                self.err(format!("bad fmt Argument constructor {path}"));
                                return Err(());
                            }
                        };
                        let fargs = subst_args(self.tcx, self.args, fargs.skip_binder());
                        // GenericArgs may lead with lifetimes; find the first type.
                        let targ_ty = fargs
                            .iter()
                            .find_map(|a| match a.kind() {
                                ty::GenericArgKind::Type(t) => Some(t),
                                _ => None,
                            })
                            .ok_or_else(|| {
                                self.err("fmt Argument constructor has no type param".to_string());
                            })?;
                        (k, targ_ty)
                    }
                    _ => {
                        self.err("bad fmt Argument constructor".to_string());
                        return Err(());
                    }
                }
            }
            _ => {
                self.err("fmt Argument constructor must be a constant".to_string());
                return Err(());
            }
        };
        let targ_ty = self.subst(targ_ty);
        // The value operand is `&T`; for `String` that is a cell.
        let mut is_cell = false;
        let val_op = match call_args.first().cloned() {
            Some(o) => {
                let t = self.operand_ty(&o);
                if let Ok(t) = t {
                    is_cell = self.is_str_cell_ty(t);
                }
                o
            }
            None => {
                self.err("fmt Argument without value".to_string());
                return Err(());
            }
        };
        let (val, _) = self.operand_expr(&val_op)?;
        let val = if is_cell { format!("{val}[0]") } else { val };
        let kind_n = match kind {
            ArgNewKind::Display => 0,
            ArgNewKind::Debug => 1,
            ArgNewKind::Noop => 2,
            ArgNewKind::Octal => 3,
            ArgNewKind::LowerHex => 4,
            ArgNewKind::UpperHex => 5,
            ArgNewKind::Pointer => 6,
            ArgNewKind::Binary => 7,
            ArgNewKind::LowerExp => 8,
            ArgNewKind::UpperExp => 9,
            ArgNewKind::Count => {
                // Indirect width/precision: raw number in the args array.
                return Ok(val);
            }
        };
        // Custom Display impls cannot be structural: diagnose.
        if matches!(kind, ArgNewKind::Display) && self.is_custom_display(targ_ty) {
            self.err(format!(
                "Display formatting for {targ_ty} needs the Formatter runtime (not supported)"
            ));
            return Err(());
        }
        let desc = self.fmt_desc(targ_ty)?;
        Ok(format!("[{desc},{val},{kind_n}]"))
    }

    /// `true` for non-primitive Display (user structs/enums need Formatter).
    fn is_custom_display(&self, ty: Ty<'tcx>) -> bool {
        match repr_of(self.tcx, ty) {
            Repr::Num(_) | Repr::Big(_) | Repr::Float | Repr::Bool | Repr::Str | Repr::BufStr => {
                false
            }
            Repr::Transparent => match ty.kind() {
                ty::Ref(_, inner, _) => self.is_custom_display(*inner),
                ty::Adt(def, args) if def.is_box() => self.is_custom_display(args.type_at(0)),
                _ => true,
            },
            _ => true,
        }
    }

    /// JS type-descriptor literal for structural Debug (see runtime.js).
    fn fmt_desc(&mut self, ty: Ty<'tcx>) -> Result<String, ()> {
        let mut seen = HashSet::new();
        self.fmt_desc_inner(ty, &mut seen, 0)
    }

    fn fmt_desc_inner(
        &mut self,
        ty: Ty<'tcx>,
        seen: &mut HashSet<DefId>,
        depth: usize,
    ) -> Result<String, ()> {
        if depth > 12 {
            return Ok("{k:'o'}".to_string());
        }
        match repr_of(self.tcx, ty) {
            Repr::Num(k) => {
                let (b, s) = match k {
                    NumKind::I8 | NumKind::U8 => (8, self.num_signed(k)),
                    NumKind::I16 | NumKind::U16 => (16, self.num_signed(k)),
                    _ => (32, self.num_signed(k)),
                };
                Ok(format!("{{k:'n',bits:{b},signed:{s}}}"))
            }
            Repr::Big(k) => Ok(format!("{{k:'n',bits:{},signed:{}}}", k.bits(), k.signed())),
            Repr::Float => {
                let b = if self.is_f32(ty) { 32 } else { 64 };
                Ok(format!("{{k:'f',bits:{b}}}"))
            }
            Repr::Bool => Ok("{k:'n',bits:8,signed:false}".to_string()),
            Repr::Str | Repr::BufStr => Ok("{k:'s'}".to_string()),
            Repr::Array => {
                let inner = match ty.kind() {
                    ty::Array(t, _) | ty::Slice(t) => *t,
                    ty::Adt(_, args) => args.type_at(0),
                    _ => unreachable!(),
                };
                Ok(format!(
                    "{{k:'a',e:{}}}",
                    self.fmt_desc_inner(inner, seen, depth + 1)?
                ))
            }
            Repr::Tuple => {
                let tys: Vec<Ty<'tcx>> = match ty.kind() {
                    ty::Tuple(tys) => tys.to_vec(),
                    ty::Adt(def, args) => {
                        // Tuple struct: check names are positional.
                        let v = def.non_enum_variant();
                        v.fields
                            .iter()
                            .map(|f| {
                                self.subst(tyutil::subst_unchecked(
                                    self.tcx,
                                    args,
                                    self.tcx.type_of(f.did).skip_binder(),
                                ))
                            })
                            .collect()
                    }
                    ty::Closure(_, cargs) => cargs
                        .as_closure()
                        .upvar_tys()
                        .iter()
                        .map(|t| self.subst(t))
                        .collect(),
                    _ => {
                        self.err(format!("bad tuple descriptor for {ty}"));
                        return Err(());
                    }
                };
                let mut ds = Vec::with_capacity(tys.len());
                for t in tys {
                    ds.push(self.fmt_desc_inner(t, seen, depth + 1)?);
                }
                // Name tuple structs for `Foo(a, b)` rendering.
                let name = match ty.kind() {
                    ty::Adt(def, _) => self
                        .tcx
                        .opt_item_name(def.did())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "?".to_string()),
                    _ => "?".to_string(),
                };
                if matches!(ty.kind(), ty::Adt(..)) {
                    Ok(format!("{{k:'T',n:'{name}',e:[{}]}}", ds.join(",")))
                } else {
                    Ok(format!("{{k:'t',e:[{}]}}", ds.join(",")))
                }
            }
            Repr::Enum => {
                let (def, _) = match ty.kind() {
                    ty::Adt(def, args) => (*def, args),
                    _ => unreachable!(),
                };
                if !seen.insert(def.did()) {
                    return Ok("{k:'o'}".to_string());
                }
                let name = self
                    .tcx
                    .opt_item_name(def.did())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "?".to_string());
                let mut vs = vec![];
                for (v, d) in def.discriminants(self.tcx) {
                    let variant = def.variant(v);
                    let is_struct = variant.ctor_kind().is_none();
                    // Tuple variants: no named fields. Struct variants: named.
                    let fs: Vec<String> = if is_struct {
                        variant
                            .fields
                            .iter()
                            .map(|f| format!("'{}'", f.name))
                            .collect()
                    } else {
                        vec![]
                    };
                    vs.push(format!(
                        "{}:['{}',{is_struct},[{}]]",
                        d.val,
                        variant.name,
                        fs.join(",")
                    ));
                }
                seen.remove(&def.did());
                Ok(format!("{{k:'E',n:'{name}',v:{{{}}}}}", vs.join(",")))
            }
            Repr::Transparent => {
                let inner = match ty.kind() {
                    ty::Ref(_, t, _) => *t,
                    ty::Adt(def, args) => {
                        let p = crate::tyutil::def_path(self.tcx, def.did());
                        // Pointer-like wrappers: the pointee is type arg 0.
                        let ptr_like = crate::tyutil::is_box_adt(self.tcx, def.did())
                            || p.ends_with("boxed::Unique")
                            || p.ends_with("ptr::NonNull")
                            || p.ends_with("ptr::Shared")
                            || p.ends_with("cell::Ref")
                            || p.ends_with("cell::RefMut")
                            || p.ends_with("rc::Rc")
                            || p.ends_with("sync::Arc")
                            || matches!(p.as_str(), "Unique" | "Shared" | "Ref" | "RefMut");
                        if ptr_like {
                            args.types().next().unwrap_or_else(|| {
                                crate::tyutil::adt_field_ty(
                                    self.tcx,
                                    ty,
                                    rustc_abi::VariantIdx::from_usize(0),
                                    0,
                                )
                                .unwrap_or(ty)
                            })
                        } else {
                            crate::tyutil::adt_field_ty(
                                self.tcx,
                                ty,
                                rustc_abi::VariantIdx::from_usize(0),
                                0,
                            )
                            .unwrap_or(ty)
                        }
                    }
                    _ => ty,
                };
                self.fmt_desc_inner(inner, seen, depth + 1)
            }
            Repr::Unit => Ok("{k:'u'}".to_string()),
            Repr::RawPtr | Repr::Dyn => Ok("{k:'p'}".to_string()),
            // Opaque JS handles format via `String(handle)`.
            Repr::JsValue | Repr::FnVal => Ok("{k:'o'}".to_string()),
            Repr::Never | Repr::Opaque => {
                self.err(format!("cannot format value of type {ty}"));
                Err(())
            }
        }
    }

    /// Strip a single transparent dereference: `(*p)` where `p: &T`
    /// (or `Box<T>`) holds the same value as `p` in our model.
    fn strip_deref(&mut self, op: mir::Operand<'tcx>) -> Result<mir::Operand<'tcx>, ()> {
        match op {
            mir::Operand::Copy(p) | mir::Operand::Move(p) => {
                if p.projection.len() == 1 && matches!(p.projection[0], mir::ProjectionElem::Deref)
                {
                    let base_ty = match self.local_tys.get(p.local.index()) {
                        Some(t) => *t,
                        None => return Ok(mir::Operand::Copy(p)),
                    };
                    match base_ty.kind() {
                        ty::Ref(..) => {
                            let base = mir::Place {
                                local: p.local,
                                projection: self.tcx.mk_place_elems(&[]),
                            };
                            Ok(mir::Operand::Copy(base))
                        }
                        _ => Ok(mir::Operand::Copy(p)),
                    }
                } else {
                    Ok(mir::Operand::Copy(p))
                }
            }
            _ => Ok(op),
        }
    }

    fn template_bytes(&mut self, op: mir::Operand<'tcx>) -> Result<Vec<u8>, ()> {
        let op = self.strip_deref(op)?;
        match op {
            mir::Operand::Constant(c) => self.const_bytes(c.const_),
            mir::Operand::Copy(p) | mir::Operand::Move(p) => {
                if p.projection.is_empty() {
                    match self.find_assign(p.local)? {
                        Some(mir::Rvalue::Use(inner, _)) => self.template_bytes(inner),
                        Some(mir::Rvalue::Ref(_, _, inner)) => {
                            // A &-typed local holding a reference: the value
                            // is the pointee; look through one more level.
                            self.template_bytes(mir::Operand::Copy(inner))
                        }
                        Some(rv) => {
                            self.err(format!("cannot peer format template through {rv:?}"));
                            Err(())
                        }
                        None => {
                            self.err("format template has no visible construction".to_string());
                            Err(())
                        }
                    }
                } else {
                    self.err(format!(
                        "format template must be a plain local, found {p:?}"
                    ));
                    Err(())
                }
            }
            mir::Operand::RuntimeChecks(_) => {
                self.err("bad format template".to_string());
                Err(())
            }
        }
    }

    fn const_bytes(&mut self, c: mir::Const<'tcx>) -> Result<Vec<u8>, ()> {
        use mir::ConstValue;
        let ty = self.subst(c.ty());
        // Array length for `&[u8; N]` (templates are array refs).
        let array_len = match ty.kind() {
            ty::Ref(_, inner, _) => match inner.kind() {
                ty::Array(_, len) => {
                    let ccx = self.ccx();
                    Some(constval::eval_usize(&ccx, *len).map_err(|m| {
                        self.err(m);
                    })?)
                }
                _ => None,
            },
            ty::Array(_, len) => {
                let ccx = self.ccx();
                Some(constval::eval_usize(&ccx, *len).map_err(|m| {
                    self.err(m);
                })?)
            }
            _ => None,
        };
        // Promoted `&[u8; N]` constants may appear as pointer scalars in any
        // const variant: resolve to the target allocation's bytes.
        if let Some(len) = array_len {
            if let Some((alloc_id, offset)) = self.const_ptr_target(&c) {
                return self.read_alloc(alloc_id, offset, len);
            }
        }
        match c {
            mir::Const::Val(ConstValue::Scalar(mir::interpret::Scalar::Ptr(ptr, _)), _) => {
                match array_len {
                    Some(len) => {
                        let (prov, offset) = ptr.into_raw_parts();
                        self.read_alloc(prov.alloc_id(), offset.bytes(), len)
                    }
                    None => {
                        self.err("bad template constant".to_string());
                        Err(())
                    }
                }
            }
            mir::Const::Val(ConstValue::Slice { alloc_id, meta }, _) => {
                self.read_alloc(alloc_id, 0, meta)
            }
            mir::Const::Val(ConstValue::Indirect { alloc_id, offset }, _) => {
                let len = match ty.kind() {
                    ty::Array(_, len) => {
                        let ccx = self.ccx();
                        constval::eval_usize(&ccx, *len).map_err(|m| {
                            self.err(m);
                        })?
                    }
                    ty::Ref(_, inner, _) => match inner.kind() {
                        ty::Array(_, len) => {
                            let ccx = self.ccx();
                            constval::eval_usize(&ccx, *len).map_err(|m| {
                                self.err(m);
                            })?
                        }
                        _ => {
                            self.err("bad template constant".to_string());
                            return Err(());
                        }
                    },
                    _ => {
                        self.err("bad template constant".to_string());
                        return Err(());
                    }
                };
                self.read_alloc(alloc_id, offset.bytes(), len)
            }
            mir::Const::Ty(_, ct) => {
                let ct = tyutil::subst_unchecked(self.tcx, self.args, ct);
                let vc = match ct.try_to_value() {
                    Some(v) => v,
                    None => {
                        self.err("bad template constant".to_string());
                        return Err(());
                    }
                };
                match vc.try_to_raw_bytes(self.tcx) {
                    Some(b) => Ok(b.to_vec()),
                    None => match vc.try_to_branch() {
                        Some(_) => {
                            // Last resort: render elements as u8s.
                            let ccx = self.ccx();
                            let s = constval::render_mir_const(&ccx, c).map_err(|m| {
                                self.err(m);
                            })?;
                            // Rendered like "[104,105]": parse back numbers.
                            let inner = s.trim().trim_start_matches('[').trim_end_matches(']');
                            inner
                                .split(',')
                                .map(|x| {
                                    x.trim().parse::<u8>().map_err(|_| {
                                        self.err("bad template constant".to_string());
                                    })
                                })
                                .collect::<Result<Vec<_>, _>>()
                        }
                        None => {
                            self.err("bad template constant".to_string());
                            Err(())
                        }
                    },
                }
            }
            mir::Const::Unevaluated(uc, _) => {
                let args = tyutil::subst_unchecked(self.tcx, self.args, uc.args);
                let uc = mir::UnevaluatedConst {
                    def: uc.def,
                    args,
                    promoted: uc.promoted,
                };
                match self.tcx.const_eval_resolve(mono_env(), uc, self.cur_span) {
                    Ok(ConstValue::Slice { alloc_id, meta }) => self.read_alloc(alloc_id, 0, meta),
                    Ok(ConstValue::Indirect { alloc_id, offset }) => {
                        // Re-dispatch as Val.
                        self.const_bytes(mir::Const::Val(
                            ConstValue::Indirect { alloc_id, offset },
                            ty,
                        ))
                    }
                    _ => {
                        self.err("bad template constant".to_string());
                        Err(())
                    }
                }
            }
            _ => {
                self.err("bad template constant".to_string());
                Err(())
            }
        }
    }

    /// If `c` is a pointer scalar, return (alloc_id, offset) of its target.
    fn const_ptr_target(&mut self, c: &mir::Const<'tcx>) -> Option<(mir::interpret::AllocId, u64)> {
        use mir::interpret::Scalar;
        let scalar = match *c {
            mir::Const::Val(mir::ConstValue::Scalar(s), _) => s,
            mir::Const::Ty(_, ct) => {
                let ct = tyutil::subst_unchecked(self.tcx, self.args, ct);
                ct.try_to_value()?.try_to_scalar()?
            }
            _ => return None,
        };
        match scalar {
            Scalar::Ptr(ptr, _) => {
                let (prov, offset) = ptr.into_raw_parts();
                Some((prov.alloc_id(), offset.bytes()))
            }
            _ => None,
        }
    }

    fn read_alloc(
        &mut self,
        alloc_id: mir::interpret::AllocId,
        offset: u64,
        len: u64,
    ) -> Result<Vec<u8>, ()> {
        use mir::interpret::{AllocRange, GlobalAlloc};
        let alloc = match self.tcx.global_alloc(alloc_id) {
            GlobalAlloc::Memory(a) => a,
            _ => {
                self.err("bad template constant".to_string());
                return Err(());
            }
        };
        let range = AllocRange {
            start: rustc_abi::Size::from_bytes(offset),
            size: rustc_abi::Size::from_bytes(len),
        };
        Ok(alloc.inner().get_bytes_unchecked(range).to_vec())
    }
}

// ---------------------------------------------------------------------------
// Statements, terminators, bodies
// ---------------------------------------------------------------------------

impl<'tcx> Emitter<'tcx> {
    /// Pre-scan: destinations of peer-only constructor calls
    /// (`Arguments::new`, `Argument::new_*`, `from_str`). These are consumed
    /// through template peering and must never be materialized.
    fn find_peer_only_calls(&mut self) {
        use crate::tyutil::ends_seg;
        let body = self.body.clone();
        for bb in body.basic_blocks.iter_enumerated() {
            let term = match &bb.1.terminator {
                Some(t) => t,
                None => continue,
            };
            if let mir::TerminatorKind::Call {
                func, destination, ..
            } = &term.kind
            {
                if !destination.projection.is_empty() {
                    continue;
                }
                let did = match func {
                    mir::Operand::Constant(c) => match self.subst(c.const_.ty()).kind() {
                        ty::FnDef(did, _) => *did,
                        ty::Closure(..) => continue,
                        _ => continue,
                    },
                    _ => continue,
                };
                let path = def_path(self.tcx, did);
                let in_std = tyutil::is_std_item(self.tcx, did);
                if !in_std {
                    continue;
                }
                if ends_seg(&path, &["Arguments", "new"])
                    || ends_seg(&path, &["Argument", "new_display"])
                    || ends_seg(&path, &["Argument", "new_debug"])
                    || ends_seg(&path, &["Argument", "new_debug_noop"])
                    || ends_seg(&path, &["Argument", "new_octal"])
                    || ends_seg(&path, &["Argument", "new_lower_hex"])
                    || ends_seg(&path, &["Argument", "new_upper_hex"])
                    || ends_seg(&path, &["Argument", "new_pointer"])
                    || ends_seg(&path, &["Argument", "new_binary"])
                    || ends_seg(&path, &["Argument", "new_lower_exp"])
                    || ends_seg(&path, &["Argument", "new_upper_exp"])
                    || ends_seg(&path, &["Argument", "from_usize"])
                    || ends_seg(&path, &["Arguments", "from_str"])
                    || ends_seg(&path, &["Arguments", "from_str_nonconst"])
                {
                    self.skipped.insert(destination.local);
                }
            }
        }
    }

    fn emit_stmt(&mut self, stmt: &mir::Statement<'tcx>) {
        self.cur_span = stmt.source_info.span;
        match &stmt.kind {
            mir::StatementKind::Assign(place_rv) => {
                let (place, rv): &(mir::Place<'tcx>, mir::Rvalue<'tcx>) = &**place_rv;
                // Track mutable reference bindings: `local = &mut place` gives
                // `local` the value of the place, so later `*local = x` must
                // store to the tracked place instead of the local.
                if let mir::Rvalue::Ref(_, mir::BorrowKind::Mut { .. }, ref_place) = rv {
                    let (ref_expr, _ref_ty) = match self.place_expr(*ref_place) {
                        Ok(v) => v,
                        Err(_) => return,
                    };
                    self.ref_locals.insert(place.local, ref_expr);
                }
                // Rewrite a Deref of a known mutable-reference local to its
                // actual tracked location (`*c = x` -> field = x).
                let (lexpr, lty) = match self.place_expr(*place) {
                    Ok(v) => v,
                    Err(_) => return,
                };
                let lexpr = if place.projection.len() == 1
                    && matches!(place.projection[0], mir::ProjectionElem::Deref)
                    && self.ref_locals.contains_key(&place.local)
                {
                    self.ref_locals[&place.local].clone()
                } else {
                    lexpr
                };
                let mut rexpr = match self.rvalue_expr(rv, lty) {
                    Ok(v) => v,
                    Err(_) => return,
                };
                rexpr = self.maybe_fround(rexpr, lty);
                self.out.push_str(&format!("{lexpr}={rexpr};\n"));
            }
            mir::StatementKind::SetDiscriminant {
                place,
                variant_index,
            } => {
                let (e, ty) = match self.place_expr(**place) {
                    Ok(v) => v,
                    Err(_) => return,
                };
                let d = match discriminant_value(self.tcx, ty, *variant_index) {
                    Some(d) => d,
                    None => {
                        self.err(format!("bad SetDiscriminant on {ty}"));
                        return;
                    }
                };
                self.out.push_str(&format!("{e}.t={d};\n"));
            }
            mir::StatementKind::Intrinsic(intr) => match &**intr {
                mir::NonDivergingIntrinsic::Assume(_) => {}
                mir::NonDivergingIntrinsic::CopyNonOverlapping(_) => {
                    self.err("copy_nonoverlapping is not supported".to_string());
                }
            },
            mir::StatementKind::StorageLive(_) => {}
            mir::StatementKind::StorageDead(local) => {
                self.ref_locals.remove(local);
            }
            mir::StatementKind::FakeRead(_)
            | mir::StatementKind::AscribeUserType(_, _)
            | mir::StatementKind::Coverage(_)
            | mir::StatementKind::ConstEvalCounter
            | mir::StatementKind::Nop
            | mir::StatementKind::PlaceMention(_)
            | mir::StatementKind::BackwardIncompatibleDropHint { .. } => {}
        }
    }

    fn emit_term(&mut self, term: &mir::Terminator<'tcx>) {
        self.cur_span = term.source_info.span;
        match &term.kind {
            mir::TerminatorKind::Goto { target } => {
                self.out
                    .push_str(&format!("bb={};continue;\n", target.index()));
            }
            mir::TerminatorKind::SwitchInt { discr, targets } => {
                let (e, ty) = match self.operand_expr(discr) {
                    Ok(v) => v,
                    Err(_) => return,
                };
                if matches!(repr_of(self.tcx, self.subst(ty)), Repr::JsValue) {
                    self.err(format!(
                        "cannot match on opaque JS handle {ty} (compare it with == instead)"
                    ));
                    return;
                }
                let is_big = matches!(repr_of(self.tcx, ty), Repr::Big(_));
                let is_bool = matches!(repr_of(self.tcx, ty), Repr::Bool);
                self.out.push_str(&format!("switch({e}){{\n"));
                for (v, t) in targets.iter() {
                    let lit = if is_bool {
                        (v != 0).to_string()
                    } else if is_big {
                        format!("{v}n")
                    } else {
                        v.to_string()
                    };
                    self.out
                        .push_str(&format!("case {lit}:bb={};continue;\n", t.index()));
                }
                self.out.push_str(&format!(
                    "default:bb={};continue;\n}}\n",
                    targets.otherwise().index()
                ));
            }
            mir::TerminatorKind::Return => {
                self.out.push_str("return M[0];\n");
            }
            mir::TerminatorKind::Unreachable => {
                self.out.push_str("throw __unreachable();\n");
            }
            mir::TerminatorKind::Drop { place, target, .. } => {
                let code = match self.emit_drop_call(*place) {
                    Ok(c) => c,
                    Err(_) => return,
                };
                match code {
                    Some(c) => {
                        self.out.push_str(&c);
                        self.out
                            .push_str(&format!("bb={};continue;\n", target.index()));
                    }
                    None => self
                        .out
                        .push_str(&format!("bb={};continue;\n", target.index())),
                }
            }
            mir::TerminatorKind::Call {
                func,
                args,
                destination,
                target,
                ..
            } => {
                if std::env::var("JS_DEBUG").is_ok() {
                    use std::io::Write;
                    std::io::stderr()
                        .write_all(
                            format!(
                                "emit_term: CALL {} => {}\n",
                                self.tcx.def_path_str(self.instance.def_id()),
                                self.tcx.def_path_str(self.instance.def_id())
                            )
                            .as_bytes(),
                        )
                        .ok();
                }
                if destination.projection.is_empty() && self.skipped.contains(&destination.local) {
                    // Peer-only constructor (Arguments::new, Argument::new_*):
                    // consumed via peering, never materialized.
                    if let Some(bb) = target {
                        self.out.push_str(&format!("bb={};continue;\n", bb.index()));
                    } else {
                        self.out.push_str("throw __unreachable();\n");
                    }
                    return;
                }
                let tgt = match self.resolve_target(func) {
                    Ok(t) => t,
                    Err(_) => return,
                };
                let dest_ty = match self.place_ty(*destination) {
                    Ok(t) => t,
                    Err(_) => return,
                };
                let mut call = match self.call_expr(&tgt, args, dest_ty) {
                    Ok(c) => c,
                    Err(_) => return,
                };
                call = self.maybe_fround(call, dest_ty);
                match target {
                    Some(bb) => {
                        if self.assign_call(call, *destination, dest_ty).is_err() {
                            return;
                        }
                        self.out.push_str(&format!("bb={};continue;\n", bb.index()));
                    }
                    None => {
                        self.out
                            .push_str(&format!("{call};\nthrow __unreachable();\n"));
                    }
                }
            }
            mir::TerminatorKind::TailCall { func, args, .. } => {
                let tgt = match self.resolve_target(func) {
                    Ok(t) => t,
                    Err(_) => return,
                };
                let ret_ty = self.local_tys[0];
                let call = match self.call_expr(&tgt, args, ret_ty) {
                    Ok(c) => c,
                    Err(_) => return,
                };
                self.out.push_str(&format!("return {call};\n"));
            }
            mir::TerminatorKind::Assert {
                cond,
                expected,
                msg,
                target,
                ..
            } => {
                let check = match self.assert_check(cond, *expected, msg) {
                    Ok(c) => c,
                    Err(_) => return,
                };
                self.out.push_str(&check);
                self.out
                    .push_str(&format!("bb={};continue;\n", target.index()));
            }
            mir::TerminatorKind::UnwindResume => {
                self.out.push_str("throw __unreachable();\n");
            }
            mir::TerminatorKind::UnwindTerminate(_) => {
                self.out.push_str("throw __abort(\"terminated\");\n");
            }
            mir::TerminatorKind::FalseEdge { real_target, .. }
            | mir::TerminatorKind::FalseUnwind { real_target, .. } => {
                self.out
                    .push_str(&format!("bb={};continue;\n", real_target.index()));
            }
            mir::TerminatorKind::Yield { .. } | mir::TerminatorKind::CoroutineDrop => {
                self.err("async coroutines are not supported".to_string());
            }
            mir::TerminatorKind::InlineAsm { .. } => {
                self.err("inline assembly is not supported".to_string());
            }
        }
    }

    fn assert_msg(&mut self, msg: &mir::AssertMessage<'tcx>) -> String {
        match msg {
            mir::AssertKind::BoundsCheck { len, index } => {
                let l = self
                    .operand_expr(len)
                    .map(|v| v.0)
                    .unwrap_or_else(|_| "?".to_string());
                let i = self
                    .operand_expr(index)
                    .map(|v| v.0)
                    .unwrap_or_else(|_| "?".to_string());
                format!(
                    "__panic(\"index out of bounds: the len is \"+{l}+\" but the index is \"+{i})"
                )
            }
            mir::AssertKind::Overflow(op, _, _) => {
                let name = match op {
                    mir::BinOp::Add => "add",
                    mir::BinOp::Sub => "sub",
                    mir::BinOp::Mul => "mul",
                    mir::BinOp::Shl => "shl",
                    mir::BinOp::Shr => "shr",
                    mir::BinOp::Div => "divide",
                    mir::BinOp::Rem => "remainder",
                    _ => "arithmetic",
                };
                format!("__panic(\"attempt to {name} with overflow\")")
            }
            mir::AssertKind::OverflowNeg(_) => {
                "__panic(\"attempt to negate with overflow\")".to_string()
            }
            mir::AssertKind::DivisionByZero(_) => {
                "__panic(\"attempt to divide by zero\")".to_string()
            }
            mir::AssertKind::RemainderByZero(_) => {
                "__panic(\"attempt to calculate the remainder with a divisor of zero\")".to_string()
            }
            mir::AssertKind::MisalignedPointerDereference { .. } => {
                // Vacuous: there are no addresses in the object model.
                return String::new();
            }
            mir::AssertKind::NullPointerDereference | mir::AssertKind::NullReferenceConstructed => {
                return String::new();
            }
            _ => "__panic(\"assertion failed\")".to_string(),
        }
    }

    /// `dest = call;` (or a bare `call;` for unit results), shared by the
    /// state-machine and the structured emitters.
    fn assign_call(
        &mut self,
        call: String,
        destination: mir::Place<'tcx>,
        dest_ty: Ty<'tcx>,
    ) -> Result<(), ()> {
        if matches!(repr_of(self.tcx, dest_ty), Repr::Unit) {
            self.out.push_str(&format!("{call};\n"));
            return Ok(());
        }
        let (dexpr, _) = self.place_expr(destination)?;
        self.out.push_str(&format!("{dexpr}={call};\n"));
        Ok(())
    }

    /// `if (cond !== expected) throw <msg>;` — empty string for the vacuous
    /// assertions (alignment/pointer checks have no meaning here).
    fn assert_check(
        &mut self,
        cond: &mir::Operand<'tcx>,
        expected: bool,
        msg: &mir::AssertMessage<'tcx>,
    ) -> Result<String, ()> {
        let m = self.assert_msg(msg);
        if m.is_empty() {
            return Ok(String::new());
        }
        let (e, _) = self.operand_expr(cond)?;
        Ok(format!("if(({e})!=={expected})throw {m};\n"))
    }

    /// The effect part of one terminator, without any control flow. Used by
    /// the structured emitter: the branches and jumps come from the plan.
    fn emit_term_effect(&mut self, term: &mir::Terminator<'tcx>) -> Result<(), ()> {
        self.cur_span = term.source_info.span;
        match &term.kind {
            // Pure control flow: no runtime effect in this backend
            // (cleanup/unwind edges are dropped; the real edge is structural).
            mir::TerminatorKind::Goto { .. }
            | mir::TerminatorKind::SwitchInt { .. }
            | mir::TerminatorKind::FalseEdge { .. }
            | mir::TerminatorKind::FalseUnwind { .. } => Ok(()),
            mir::TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => {
                // Peer-only constructor (Arguments::new, Argument::new_*):
                // consumed via peering, never materialized.
                if destination.projection.is_empty() && self.skipped.contains(&destination.local) {
                    return Ok(());
                }
                let tgt = self.resolve_target(func)?;
                let dest_ty = self.place_ty(*destination)?;
                let call = self.call_expr(&tgt, args, dest_ty)?;
                let call = self.maybe_fround(call, dest_ty);
                self.assign_call(call, *destination, dest_ty)
            }
            mir::TerminatorKind::TailCall { func, args, .. } => {
                let tgt = self.resolve_target(func)?;
                let ret_ty = self.local_tys[0];
                let call = self.call_expr(&tgt, args, ret_ty)?;
                self.out.push_str(&format!("return {call};\n"));
                Ok(())
            }
            mir::TerminatorKind::Drop { place, .. } => {
                let call = self.emit_drop_call(*place)?;
                if let Some(c) = call {
                    self.out.push_str(&c);
                }
                Ok(())
            }
            mir::TerminatorKind::Assert {
                cond,
                expected,
                msg,
                ..
            } => {
                let check = self.assert_check(cond, *expected, msg)?;
                self.out.push_str(&check);
                Ok(())
            }
            mir::TerminatorKind::Return => {
                self.out.push_str("return M[0];\n");
                Ok(())
            }
            mir::TerminatorKind::Unreachable | mir::TerminatorKind::UnwindResume => {
                self.out.push_str("throw __unreachable();\n");
                Ok(())
            }
            mir::TerminatorKind::UnwindTerminate(_) => {
                self.out.push_str("throw __abort(\"terminated\");\n");
                Ok(())
            }
            _ => Err(()),
        }
    }

    /// The `SwitchInt` discriminator operand of a block (cloned, so it does
    /// not borrow the body while the emitter runs).
    fn switch_discr(&mut self, bb: usize) -> Result<mir::Operand<'tcx>, ()> {
        let Some(term) = &self.body.basic_blocks[mir::BasicBlock::from_usize(bb)].terminator else {
            return Err(());
        };
        let mir::TerminatorKind::SwitchInt { discr, .. } = &term.kind else {
            return Err(());
        };
        Ok(discr.clone())
    }

    /// Discriminator of a block's `SwitchInt` as a JS expression, plus its
    /// type (which decides how the case values are rendered).
    fn switch_parts(&mut self, bb: usize) -> Result<(String, Ty<'tcx>), ()> {
        let discr = self.switch_discr(bb)?;
        let (e, ty) = self.operand_expr(&discr)?;
        // Opaque JS handles have no Rust-visible discriminant.
        if matches!(repr_of(self.tcx, self.subst(ty)), Repr::JsValue) {
            self.err(format!(
                "cannot match on opaque JS handle {ty} (compare it with == instead)"
            ));
            return Err(());
        }
        Ok((e, ty))
    }

    /// A `SwitchInt` value rendered as a JS literal.
    fn switch_lit(&self, v: u128, ty: Ty<'tcx>) -> String {
        switch_lit_of(self.tcx, v, ty)
    }

    /// Render a structured plan, `labels` holding the JS loop labels of the
    /// enclosing loops (innermost last).
    fn render_plan(
        &mut self,
        nodes: &[crate::structure::Node],
        labels: &mut Vec<String>,
    ) -> Result<(), ()> {
        use crate::structure::Node;
        for n in nodes {
            if self.poisoned {
                return Err(());
            }
            match n {
                Node::Block(bb) => {
                    let data = self.body.basic_blocks[mir::BasicBlock::from_usize(*bb)].clone();
                    for stmt in &data.statements {
                        self.emit_stmt(stmt);
                        if self.poisoned {
                            return Err(());
                        }
                    }
                    let Some(term) = &data.terminator else {
                        return Err(());
                    };
                    self.emit_term_effect(term)?;
                }
                Node::Branch {
                    bb,
                    arms,
                    otherwise,
                } => {
                    // The header block's statements compute the discriminant
                    // (e.g. `x > y` for `if`): they run once on entry, like
                    // a Block, and must not be dropped.
                    {
                        let data = self.body.basic_blocks[mir::BasicBlock::from_usize(*bb)].clone();
                        for stmt in &data.statements {
                            self.emit_stmt(stmt);
                            if self.poisoned {
                                return Err(());
                            }
                        }
                    }
                    let (discr, ty) = self.switch_parts(*bb)?;
                    let tcx = self.tcx;
                    let cond = |v: u128| {
                        let lit = switch_lit_of(tcx, v, ty);
                        format!("({discr}==={lit})")
                    };
                    if arms.len() <= 2 {
                        // `if` / `else if` / `else` chain: each arm tests one
                        // discriminant value, `otherwise` is the final else.
                        // (1 arm = `if`; 2 arms = `if`/`else if` -- never a
                        // `switch`, and never a dangling `else`.)
                        let mut first = true;
                        for (v, body) in arms.iter() {
                            self.out.push_str(&format!(
                                "{}if({}){{\n",
                                if first { "" } else { "else " },
                                cond(*v)
                            ));
                            first = false;
                            self.render_plan(body, labels)?;
                            self.out.push_str("}\n");
                        }
                        if !otherwise.is_empty() {
                            self.out.push_str("else{\n");
                            self.render_plan(otherwise, labels)?;
                            self.out.push_str("}\n");
                        }
                    } else {
                        self.out.push_str(&format!("switch({discr}){{\n"));
                        for (v, body) in arms.iter() {
                            let lit = switch_lit_of(self.tcx, *v, ty);
                            self.out.push_str(&format!("case {lit}:{{\n"));
                            self.render_plan(body, labels)?;
                            self.out.push_str("break;\n}\n");
                        }
                        self.out.push_str("default:{\n");
                        self.render_plan(otherwise, labels)?;
                        self.out.push_str("break;\n}\n}\n");
                    }
                }
                Node::Loop { bb, body } => {
                    // `loop` (Goto header) and `while`/`for` (SwitchInt
                    // header) both lower to `L: while (true)` with the header
                    // block's statements at the top of every iteration: they
                    // compute the loop test and must run before it, on every
                    // pass including the first. A `while`-style header then
                    // breaks out when its exit arm is selected.
                    let label = format!("L{}", self.loop_seq);
                    self.loop_seq += 1;
                    let data = self.body.basic_blocks[mir::BasicBlock::from_usize(*bb)].clone();
                    // Single-successor headers (`loop`, or a call/drop
                    // header whose decision branch sits deeper in the body)
                    // have no test; a `SwitchInt` header breaks out on its
                    // exit arm.
                    let single_succ = matches!(
                        data.terminator.as_ref().map(|t| &t.kind),
                        Some(
                            mir::TerminatorKind::Goto { .. }
                                | mir::TerminatorKind::Drop { .. }
                                | mir::TerminatorKind::Call { .. }
                                | mir::TerminatorKind::Assert { .. }
                                | mir::TerminatorKind::FalseEdge { .. }
                                | mir::TerminatorKind::FalseUnwind { .. }
                        )
                    );
                    let test = if single_succ {
                        None
                    } else {
                        Some(self.loop_test(*bb)?)
                    };
                    labels.push(label.clone());
                    self.out.push_str(&format!("{label}: while (true){{\n"));
                    for stmt in &data.statements {
                        self.emit_stmt(stmt);
                        if self.poisoned {
                            return Err(());
                        }
                    }
                    if single_succ {
                        // The header's own effect (call assignment, drop,
                        // assert check) runs each iteration; `Goto` has none.
                        if let Some(term) = &data.terminator {
                            self.emit_term_effect(term)?;
                        }
                    }
                    if let Some(test) = test {
                        self.out
                            .push_str(&format!("if (!({test})) break {label};\n"));
                    }
                    self.render_plan(body, labels)?;
                    self.out.push_str("}\n");
                    labels.pop();
                }
                Node::Jump { depth, is_break } => {
                    let Some(label) = labels.get(labels.len() - *depth).cloned() else {
                        return Err(());
                    };
                    self.out.push_str(&format!(
                        "{} {label};\n",
                        if *is_break { "break" } else { "continue" }
                    ));
                }
                Node::Stop => {}
            }
        }
        Ok(())
    }

    /// Loop condition for a header block: the `SwitchInt` test that keeps the
    /// iteration inside the loop body.
    fn loop_test(&mut self, bb: usize) -> Result<String, ()> {
        let kind = {
            let Some(term) = &self.body.basic_blocks[mir::BasicBlock::from_usize(bb)].terminator
            else {
                return Err(());
            };
            match &term.kind {
                mir::TerminatorKind::Goto { .. } => None,
                mir::TerminatorKind::SwitchInt { discr, targets } => Some((
                    discr.clone(),
                    targets
                        .iter()
                        .map(|(v, t)| (v, t.index()))
                        .collect::<Vec<_>>(),
                )),
                _ => return Err(()),
            }
        };
        let Some((discr, targets)) = kind else {
            return Ok("true".to_string());
        };
        let (e, ty) = self.operand_expr(&discr)?;
        let mut body_val = None;
        let mut outside = 0;
        for (v, t) in targets {
            match &self.structure {
                Some(st) if st.contains_in_loop(bb, t) => {
                    if body_val.is_none() {
                        body_val = Some(v);
                    }
                }
                _ => outside += 1,
            }
        }
        let Some(body_val) = body_val else {
            return Err(());
        };
        if outside != 1 {
            return Err(());
        }
        let lit = switch_lit_of(self.tcx, body_val, ty);
        Ok(format!("{e}==={lit}"))
    }

    /// Try to emit `body` with structured control flow. On failure nothing of
    /// the attempt is kept, so the caller can fall back to the state machine.
    fn emit_structured(&mut self, body: &mir::Body<'tcx>) -> bool {
        let path = self.tcx.def_path_str(self.instance.def_id());
        if std::env::var("JS_DEBUG").is_ok() {
            use std::io::Write;
            std::io::stderr()
                .write_all(format!("emit_structured: START {}\n", path).as_bytes())
                .ok();
        }
        let t_an = std::time::Instant::now();
        let timing = std::env::var("JS_TIMING").is_ok();
        let Some(st) = crate::structure::Structure::analyze(body) else {
            if timing {
                eprintln!(
                    "[js-timing] structured {path} analyze FAILED elapsed={:.3}ms",
                    t_an.elapsed().as_secs_f64() * 1000.0
                );
            }
            if std::env::var("JS_DEBUG").is_ok() {
                use std::io::Write;
                std::io::stderr()
                    .write_all(format!("emit_structured: analyze none {}\n", path).as_bytes())
                    .ok();
            }
            return false;
        };
        if timing {
            eprintln!(
                "[js-timing] structured {path} analyze OK elapsed={:.3}ms",
                t_an.elapsed().as_secs_f64() * 1000.0
            );
        }
        let budget = 4 * st.block_count() + 64;
        let t_pl = std::time::Instant::now();
        let Some(nodes) = crate::structure::plan(body, &st, budget) else {
            if timing {
                eprintln!(
                    "[js-timing] structured {path} plan FAILED elapsed={:.3}ms",
                    t_pl.elapsed().as_secs_f64() * 1000.0
                );
            }
            if std::env::var("JS_DEBUG").is_ok() {
                use std::io::Write;
                std::io::stderr()
                    .write_all(format!("emit_structured: plan none {}\n", path).as_bytes())
                    .ok();
            }
            return false;
        };
        let mark = self.out.len();
        let saved_loop = self.loop_seq;
        self.loop_seq = 0;
        self.structure = Some(st);
        self.quiet = true;
        if timing {
            eprintln!(
                "[js-timing] structured {path} plan OK elapsed={:.3}ms",
                t_pl.elapsed().as_secs_f64() * 1000.0
            );
        }
        let mut labels = Vec::new();
        let t_re = std::time::Instant::now();
        let ok = self.render_plan(&nodes, &mut labels).is_ok() && !self.poisoned;
        if timing {
            eprintln!(
                "[js-timing] structured {path} render ok={ok} elapsed={:.3}ms",
                t_re.elapsed().as_secs_f64() * 1000.0
            );
        }
        if std::env::var("JS_DEBUG").is_ok() {
            use std::io::Write;
            std::io::stderr()
                .write_all(format!("emit_structured: RENDER {} => {}\n", path, ok).as_bytes())
                .ok();
        }
        self.quiet = false;
        self.structure = None;
        self.loop_seq = saved_loop;
        if !ok {
            self.out.truncate(mark);
            self.poisoned = false;
            if std::env::var("JS_STRUCT_DEBUG").is_ok() {
                match &self.last_err {
                    Some(e) => eprintln!("STRUCT-FALLBACK: render failed: {e}"),
                    None => eprintln!("STRUCT-FALLBACK: render returned Err"),
                }
            }
        }
        ok
    }

    fn emit_body(&mut self, body: &mir::Body<'tcx>) {
        let t_fn_body = std::time::Instant::now();
        if body.spread_arg.is_some() {
            self.err("variadic functions are not supported".to_string());
            return;
        }
        // Coroutine bodies (async) are rejected via Yield, but a body that is
        // itself a coroutine has no Callable MIR shape we support.
        if body.coroutine.is_some() {
            self.err("async coroutines are not supported".to_string());
            return;
        }
        let name = match crate::g_fn_names().lock().unwrap().get(&self.instance) {
            Some(n) => n.clone(),
            None => {
                self.err("internal: missing function name".to_string());
                return;
            }
        };
        let mut params = vec![];
        let mut frame = vec!["undefined".to_string()];
        for i in 0..body.arg_count {
            params.push(format!("p{i}", i = i + 1));
            frame.push(format!("p{}", i + 1));
        }
        self.out
            .push_str(&format!("function {name}({}){{\n", params.join(",")));
        // One object per frame: `M[0]` is the return place, `M[1..]` the
        // arguments; every other slot is `undefined` until written (exactly
        // the semantics of the old per-local `let` declarations).
        self.out
            .push_str(&format!("const M=[{}];\n", frame.join(",")));
        if std::env::var("JS_STRUCT_DEBUG").is_ok() {
            eprintln!("STRUCT-BODY {name}");
        }
        if std::env::var("JS_DEBUG").is_ok() {
            use std::io::Write;
            std::io::stderr()
                .write_all(format!("emit_body: START {}\n", name).as_bytes())
                .ok();
        }
        if self.emit_structured(body) {
            self.out.push_str("}\n");
            if std::env::var("JS_DEBUG").is_ok() {
                use std::io::Write;
                std::io::stderr()
                    .write_all(format!("emit_body: structured OK {}\n", name).as_bytes())
                    .ok();
            }
            return;
        }
        if std::env::var("JS_TIMING").is_ok() {
            eprintln!(
                "[js-timing] fallback {name} switch-bb blocks={} elapsed={:.3}ms",
                body.basic_blocks.len(),
                t_fn_body.elapsed().as_secs_f64() * 1000.0
            );
        }
        self.out.push_str("let bb=0;\nwhile(true){switch(bb){\n");
        for (i, data) in body.basic_blocks.iter_enumerated() {
            // Unwind/cleanup blocks are never entered: this backend has no
            // real unwinding (drops are handled by the GC), and the assert
            // terminators panic inline. Emitting them would turn the MIR
            // `resume`/drop trampolines into live JS.
            if data.is_cleanup {
                continue;
            }
            self.out.push_str(&format!("case {}:{{\n", i.index()));
            for stmt in &data.statements {
                self.emit_stmt(stmt);
                if self.poisoned {
                    return;
                }
            }
            if let Some(term) = &data.terminator {
                self.emit_term(term);
                if self.poisoned {
                    return;
                }
            } else {
                self.err("basic block without terminator".to_string());
                return;
            }
            self.out.push_str("break;\n}\n");
        }
        self.out.push_str("}\n}\n}\n");
    }
}

// ---------------------------------------------------------------------------
// Range-based indexing helpers
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum RangeKind {
    Range,
    RangeFrom,
    RangeTo,
    RangeFull,
    RangeInclusive,
}

impl<'tcx> Emitter<'tcx> {
    fn range_kind(&mut self, ty: Option<Ty<'tcx>>) -> Result<RangeKind, ()> {
        let ty = match ty {
            Some(t) => self.subst(t),
            None => {
                self.err("missing index type".to_string());
                return Err(());
            }
        };
        match ty.kind() {
            ty::Adt(def, _) if tyutil::is_std_item(self.tcx, def.did()) => {
                let p = def_path(self.tcx, def.did());
                let name = p.rsplit("::").next().unwrap_or("");
                match name {
                    "Range" => Ok(RangeKind::Range),
                    "RangeFrom" => Ok(RangeKind::RangeFrom),
                    "RangeTo" => Ok(RangeKind::RangeTo),
                    "RangeFull" => Ok(RangeKind::RangeFull),
                    "RangeInclusive" => Ok(RangeKind::RangeInclusive),
                    _ => {
                        self.err(format!("unsupported index type {ty}"));
                        Err(())
                    }
                }
            }
            _ => {
                self.err(format!("unsupported index type {ty}"));
                Err(())
            }
        }
    }

    /// Slice/Vec indexing: usize → bounds-checked element; ranges → sub-slice.
    fn slice_index(
        &mut self,
        base: &str,
        idx: &str,
        idx_ty: Option<Ty<'tcx>>,
    ) -> Result<String, ()> {
        let idx_ty = idx_ty.map(|t| self.subst(t));
        if matches!(idx_ty.map(|t| repr_of(self.tcx, t)), Some(Repr::Num(_))) {
            return Ok(format!("{base}[__idx({base},{idx})]"));
        }
        match self.range_kind(idx_ty)? {
            RangeKind::Range => Ok(format!("{base}.slice({idx}[0],{idx}[1])")),
            RangeKind::RangeFrom => Ok(format!("{base}.slice({idx}[0])")),
            RangeKind::RangeTo => Ok(format!("{base}.slice(0,{idx}[0])")),
            RangeKind::RangeFull => Ok(format!("{base}.slice()")),
            RangeKind::RangeInclusive => Ok(format!("{base}.slice({idx}[0],{idx}[1]+1)")),
        }
    }

    /// `&s[range]` with native-like panics.
    fn str_index(
        &mut self,
        base: String,
        idx: String,
        idx_ty: Option<Ty<'tcx>>,
    ) -> Result<String, ()> {
        let idx_ty = idx_ty.map(|t| self.subst(t));
        if matches!(idx_ty.map(|t| repr_of(self.tcx, t)), Some(Repr::Num(_))) {
            self.err("cannot index str by integer".to_string());
            return Err(());
        }
        match self.range_kind(idx_ty)? {
            RangeKind::Range => Ok(format!("__str_index({base},{idx}[0],{idx}[1])")),
            RangeKind::RangeFrom => Ok(format!("__str_index_from({base},{idx}[0])")),
            RangeKind::RangeTo => Ok(format!("__str_index_to({base},{idx}[0])")),
            RangeKind::RangeFull => Ok(base),
            RangeKind::RangeInclusive => Ok(format!("__str_index({base},{idx}[0],{idx}[1]+1)")),
        }
    }

    /// `(lo_expr, hi_expr)` byte bounds for `str::get`-style helpers.
    fn str_range_bounds(
        &mut self,
        idx: &str,
        idx_ty: Option<Ty<'tcx>>,
    ) -> Result<(String, String), ()> {
        let idx_ty = idx_ty.map(|t| self.subst(t));
        if matches!(idx_ty.map(|t| repr_of(self.tcx, t)), Some(Repr::Num(_))) {
            self.err("cannot index str by integer".to_string());
            return Err(());
        }
        match self.range_kind(idx_ty)? {
            RangeKind::Range => Ok((format!("{idx}[0]"), format!("{idx}[1]"))),
            RangeKind::RangeFrom => Ok((format!("{idx}[0]"), "9007199254740991".to_string())),
            RangeKind::RangeTo => Ok(("0".to_string(), format!("{idx}[0]"))),
            RangeKind::RangeFull => Ok(("0".to_string(), "9007199254740991".to_string())),
            RangeKind::RangeInclusive => Ok((format!("{idx}[0]"), format!("{idx}[1]+1"))),
        }
    }

    /// Pattern operand (`&str`, `char`) → JS string expression.
    fn str_pattern(&mut self, pat: &str, pat_ty: Option<Ty<'tcx>>) -> Result<String, ()> {
        let ty = match pat_ty {
            Some(t) => self.subst(t),
            None => {
                self.err("missing pattern type".to_string());
                return Err(());
            }
        };
        match ty.kind() {
            ty::Ref(_, inner, _) if matches!(inner.kind(), ty::Str) => Ok(pat.to_string()),
            ty::Str => Ok(pat.to_string()),
            _ if matches!(repr_of(self.tcx, ty), Repr::BufStr) => Ok(pat.to_string()),
            _ if matches!(repr_of(self.tcx, ty), Repr::Num(tyutil::NumKind::Char)) => {
                Ok(format!("String.fromCodePoint({pat})"))
            }
            _ => {
                self.err(format!("string pattern of type {ty} is not supported"));
                Err(())
            }
        }
    }

    /// `slice::copy_from_slice`: peer the destination place so in-place
    /// writes hit the original array instead of a slice copy.
    fn copy_from_slice(
        &mut self,
        arg_ops: &[Spanned<mir::Operand<'tcx>>],
        args: &[String],
    ) -> Result<String, ()> {
        let src = args.get(1).cloned().unwrap_or_else(|| "[]".to_string());
        let dst_op = match arg_ops.first() {
            Some(sp) => sp.node.clone(),
            None => {
                self.err("copy_from_slice without destination".to_string());
                return Err(());
            }
        };
        let dst_place = match dst_op {
            mir::Operand::Copy(p) | mir::Operand::Move(p) => p,
            _ => {
                self.err("copy_from_slice destination must be a place".to_string());
                return Err(());
            }
        };
        // If the destination is `_ = base[range]`, write into the base.
        if let Some((last, prefix)) = dst_place.projection.split_last() {
            if let mir::ProjectionElem::Index(idx) = *last {
                let idx_ty = self.local_tys.get(idx.index()).copied();
                let base = mir::Place {
                    local: dst_place.local,
                    projection: self.tcx.mk_place_elems(prefix),
                };
                let (bexpr, _) = self.place_expr(base)?;
                let idxe = self.local(idx);
                match self.range_kind(Some(idx_ty.unwrap_or(self.tcx.types.usize))) {
                    Ok(RangeKind::Range) => {
                        return Ok(format!(
                            "__copy_range({bexpr},{idxe}[0],{idxe}[1]-{idxe}[0],{src})"
                        ));
                    }
                    Ok(RangeKind::RangeInclusive) => {
                        return Ok(format!(
                            "__copy_range({bexpr},{idxe}[0],{idxe}[1]+1-{idxe}[0],{src})"
                        ));
                    }
                    Ok(RangeKind::RangeFrom) => {
                        return Ok(format!(
                            "__copy_range({bexpr},{idxe}[0],{bexpr}.length-{idxe}[0],{src})"
                        ));
                    }
                    Ok(RangeKind::RangeTo) => {
                        return Ok(format!("__copy_range({bexpr},0,{idxe}[0],{src})"));
                    }
                    Ok(RangeKind::RangeFull) => {
                        return Ok(format!("__copy_from_slice({bexpr},{src})"));
                    }
                    Err(_) => {}
                }
            }
        }
        let (dexpr, _) = self.place_expr(dst_place)?;
        Ok(format!("__copy_from_slice({dexpr},{src})"))
    }
}

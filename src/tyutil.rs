//! Type model (§4, §5).
//!
//!
//! Representations (all decisions are per *monomorphized* type):
//! - `bool` → JS boolean; `char`/small ints/`usize`/`isize` → JS number.
//! - `i64/u64/i128/u128` → JS BigInt (spec default is a split lo/hi pair;
//!   BigInt is used here for exactness — see `doc/limits.md` note in lib.rs).
//! - `f32/f64` → JS number (`f32` widened; `Math.fround` at casts/stores).
//! - `&str`/`str`/`String` → JS string; `Vec<T>`/`[T]`/`[T; N]` → JS Array.
//! - structs/tuples/unions/`Cell` → JS Array (field order); enums → `{t, v}`.
//! - `&T`/`&mut T`/`Box<T>`/`Rc<T>`/`Arc<T>` → transparent (the inner value).
//! - `RefCell` guards → transparent (borrow flags not enforced, documented).
//! - fn items/closures/fn pointers → JS functions (closures take env first).
//! - `()`/uninhabited-ZSTs → `undefined`.
//! - raw pointers, `dyn Trait`, coroutines, unions with raw guts → diagnosed.

#![allow(dead_code)]

use rustc_abi::{FieldIdx, VariantIdx};
use rustc_hir::attrs::lang_items::LangItem;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, AdtKind, Ty, TyCtxt, Unnormalized};
use rustc_type_ir::{TypeFoldable, TypeFolder, TypeSuperFoldable, TypeVisitableExt};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NumKind {
    I8,
    I16,
    I32,
    U8,
    U16,
    U32,
    Usize,
    Isize,
    Char,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BigKind {
    I64,
    U64,
    I128,
    U128,
}

impl BigKind {
    pub fn bits(self) -> u32 {
        match self {
            BigKind::I64 | BigKind::U64 => 64,
            BigKind::I128 | BigKind::U128 => 128,
        }
    }
    pub fn signed(self) -> bool {
        matches!(self, BigKind::I64 | BigKind::I128)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Repr {
    Bool,
    /// JS number.
    Num(NumKind),
    /// JS BigInt.
    Big(BigKind),
    /// JS number (f32 widened to f64).
    Float,
    /// JS string.
    Str,
    /// JS string (owned `String`; same runtime shape as `Str`).
    BufStr,
    /// JS Array (`Vec`, slice, fixed array).
    Array,
    /// JS Array, positional fields (struct/tuple/closure env/`Cell`).
    Tuple,
    /// `{t: <discriminant value>, v: [...]}`.
    Enum,
    /// JS function value.
    FnVal,
    /// Opaque JS object handle (`#[js::opaque]` struct, e.g. a DOM node).
    /// Passed by reference directly, never serialized; `Clone` is identity,
    /// equality is `===`, `Drop` is a no-op.
    JsValue,
    /// Same value as the pointee/inner (`&T`, `&mut T`, `Box`, `Rc`, `Arc`).
    Transparent,
    /// `undefined` (`()`, ZSTs).
    Unit,
    /// Uninhabited / diverging.
    Never,
    /// Raw pointer (creation/deref diagnosed at use sites).
    RawPtr,
    /// `dyn Trait` (diagnosed).
    Dyn,
    /// Anything else (diagnosed): foreign, coroutine, unresolved params, ...
    Opaque,
}

impl Repr {
    pub fn is_unit_like(self) -> bool {
        matches!(self, Repr::Unit)
    }
}

/// Monomorphization environment for the whole backend: post-typeck,
/// all params concrete, associated types revealed.
pub fn mono_env<'tcx>() -> ty::TypingEnv<'tcx> {
    ty::TypingEnv::fully_monomorphized()
}

/// Soft early-binder substitution that never panics.
///
/// Unlike `EarlyBinder::instantiate` — which panics (tripping rustc's ICE
/// hook, which prints an ICE and writes a `rustc-ice-*.txt` file even when
/// the panic would later be caught) whenever a param index is out of range,
/// the arg kind mismatches, or args are empty while the value has params —
/// this folder replaces `Param(i)` / `ConstParam(i)` with `args[i]` only
/// when that entry exists *and* has the expected kind, and leaves everything
/// else untouched. Regions are left as-is (erased during normalization).
/// Unsubstitutable leftovers surface as `Repr::Opaque` downstream, which
/// callers already diagnose.
struct SoftSubstFolder<'tcx> {
    tcx: TyCtxt<'tcx>,
    args: ty::GenericArgsRef<'tcx>,
}

impl<'tcx> TypeFolder<TyCtxt<'tcx>> for SoftSubstFolder<'tcx> {
    fn cx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx> {
        if !t.has_param() {
            return t;
        }
        match t.kind() {
            ty::Param(p) => match self.args.get(p.index as usize).map(|a| a.kind()) {
                Some(ty::GenericArgKind::Type(sub)) => sub,
                _ => t,
            },
            _ => t.super_fold_with(self),
        }
    }

    fn fold_const(&mut self, c: ty::Const<'tcx>) -> ty::Const<'tcx> {
        match c.kind() {
            ty::ConstKind::Param(p) => match self.args.get(p.index as usize).map(|a| a.kind()) {
                Some(ty::GenericArgKind::Const(sub)) => sub,
                _ => c,
            },
            _ => c.super_fold_with(self),
        }
    }
    // Regions: default no-op (leave as-is; erased by normalization).
}

/// `true` if an `impl Into<T>` / `impl From<T>` display-inner (e.g.
/// `Into<String>`, `From<&str>`) targets a string-like type whose runtime
/// value crosses boundaries as a raw string.
fn is_string_like_target(inner: &str) -> bool {
    let target = inner
        .split_once('<')
        .map(|(_, t)| t.trim_end_matches(|c| c == '>' || c == ']'))
        .unwrap_or("")
        .trim();
    matches!(target, "String" | "str" | "&str")
}

/// Peel `&T`/`&mut T` and transparent box wrappers (`Box`/`Rc`/`Arc`)
/// for equality resolution: `&Box<T> == ...` and `Box<T> == ...` both
/// compare the `T` values (all three wrappers are transparent in this
/// backend), while resolving the wrapper impls would reference core/alloc
/// bodies with no bundle to link. Raw pointers are NOT peeled (address
/// comparison must keep its type).
pub fn peel_eq_self<'tcx>(tcx: TyCtxt<'tcx>, mut ty: Ty<'tcx>) -> Ty<'tcx> {
    loop {
        match ty.kind() {
            ty::Ref(_, inner, _) => {
                ty = *inner;
                continue;
            }
            ty::Adt(def, args) => {
                let transparent_box = def.is_box() || is_box_adt(tcx, def.did());
                let transparent_rc = is_std_item(tcx, def.did()) && {
                    let p = def_path(tcx, def.did());
                    p.ends_with("rc::Rc") || p.ends_with("sync::Arc") || p == "Rc" || p == "Arc"
                };
                if (transparent_box || transparent_rc) && !args.is_empty() {
                    if let Some(t) = args.iter().find_map(|a| match a.kind() {
                        ty::GenericArgKind::Type(t) => Some(t),
                        _ => None,
                    }) {
                        ty = t;
                        continue;
                    }
                }
                return ty;
            }
            _ => return ty,
        }
    }
}

/// Substitute `instance.args` into `ty`, then normalize associated types.
/// Never panics: unsubstitutable params are left as-is, and normalization
/// failure falls back to the substituted (or raw) type.
pub fn subst_ty<'tcx>(tcx: TyCtxt<'tcx>, args: ty::GenericArgsRef<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
    // Synthetic impl-trait params display as `impl Into<T>` / `impl From<T>`
    // and carry an internal impl-trait-subst that cannot be instantiated in
    // isolation. For *string-like* targets the boundary convention passes
    // the raw string value, so concretize the type to `str` (e.g. the param
    // of `point(name: impl Into<String>)`). For any other target (e.g.
    // `impl Into<PointRef>`) the param is left unresolved so callers pass
    // the value through untouched — concretizing those to `str` would
    // miscompile the value (e.g. unwrap a `{t, v}` enum as a string cell).
    if let ty::TyKind::Param(_) = ty.kind() {
        let s = ty.to_string();
        let is_impl_trait = s.starts_with("impl ") && (s.ends_with(']') || s.ends_with('>'));
        let is_self_param = s == "Self" || s.starts_with("Self/");
        if is_impl_trait || is_self_param {
            let inner = if s.ends_with('>') {
                &s[5..s.len() - 1]
            } else if s.ends_with(']') {
                &s[5..s.len() - 2]
            } else {
                // `Self` param of an impl-trait item.
                "Self"
            };
            let trait_name = if is_impl_trait {
                inner.split_once('<').map(|(t, _)| t).unwrap_or("")
            } else {
                "Self"
            };
            if is_impl_trait && !(trait_name == "Into" || trait_name == "From") {
                // Only concrete Into/From params get concretized; other
                // impl-trait params are left unsubstituted below.
            } else if is_self_param || trait_name == "Self" {
                return tcx.types.str_;
            } else if is_impl_trait && is_string_like_target(inner) {
                return tcx.types.str_;
            }
        }
        // A bare param with no args to substitute from cannot be resolved.
        if args.is_empty() {
            return ty;
        }
    } else if !ty.has_param() {
        // Fast path: nothing to substitute; still normalize aliases.
        return match tcx.try_normalize_erasing_regions(mono_env(), Unnormalized::new_wip(ty)) {
            Ok(t) => t,
            Err(_) => ty,
        };
    } else if args.is_empty() {
        return ty;
    }
    let mut folder = SoftSubstFolder { tcx, args };
    let t = ty.fold_with(&mut folder);
    match tcx.try_normalize_erasing_regions(mono_env(), Unnormalized::new_wip(t)) {
        Ok(t) => t,
        Err(_) => t,
    }
}

/// Substitute `outer` into `inner`, leaving unsubstitutable params as-is.
/// `inner` may carry binders whose args are not available in `outer` (e.g. a
/// trait impl's declaration types viewed from an instance with no type
/// params); that simply means this value cannot be substituted, so leaving
/// it unchanged is the correct fallback.
pub fn subst_unchecked<'tcx, T>(tcx: TyCtxt<'tcx>, outer: ty::GenericArgsRef<'tcx>, inner: T) -> T
where
    T: TypeFoldable<TyCtxt<'tcx>>,
{
    if outer.is_empty() {
        return inner;
    }
    let mut folder = SoftSubstFolder { tcx, args: outer };
    inner.fold_with(&mut folder)
}

/// Substitute instance args into generic args (for callee args, consts, ...).
/// Falls back to the inner args unchanged if instantiation panics.
pub fn subst_args<'tcx>(
    tcx: TyCtxt<'tcx>,
    outer: ty::GenericArgsRef<'tcx>,
    inner: ty::GenericArgsRef<'tcx>,
) -> ty::GenericArgsRef<'tcx> {
    subst_unchecked(tcx, outer, inner)
}

/// Substitute free inference params (e.g., the self-type of an `impl Into<T>`
/// method call whose opaque param points at the enclosing function's inference
/// param) against the enclosing function's instance args. `instantiate` alone
/// never touches free params, and `dgs::point(name: impl Into<String>)` leaves
/// `name.into()` with fargs `[ParamTy(0), String]`; substing `ParamTy(0)` with
/// the enclosing args (e.g. `&str`) lets `try_resolve` monomorphize the call.
pub fn subst_free_params<'tcx>(
    tcx: TyCtxt<'tcx>,
    outer: ty::GenericArgsRef<'tcx>,
    inner: ty::GenericArgsRef<'tcx>,
    enclosing_fn_def_id: Option<rustc_hir::def_id::DefId>,
) -> ty::GenericArgsRef<'tcx> {
    if outer.is_empty() {
        return inner;
    }
    let mut concrete = Vec::with_capacity(inner.len());
    for arg in inner.iter() {
        let arg = match arg.kind() {
            ty::GenericArgKind::Type(ty) => {
                let ty = subst_free_param_ty(tcx, outer, ty, enclosing_fn_def_id);
                ty::GenericArg::from(ty)
            }
            _ => arg,
        };
        concrete.push(arg);
    }
    tcx.mk_args(&concrete)
}

fn subst_free_param_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    outer: ty::GenericArgsRef<'tcx>,
    ty: ty::Ty<'tcx>,
    enclosing_fn_def_id: Option<rustc_hir::def_id::DefId>,
) -> ty::Ty<'tcx> {
    if let ty::TyKind::Param(p) = ty.kind() {
        if let Some(fn_def_id) = enclosing_fn_def_id {
            let generics = tcx.generics_of(fn_def_id);
            if (p.index as usize) < generics.own_params.len() {
                let pd = &generics.own_params[p.index as usize];
                if let ty::GenericParamDefKind::Type { synthetic, .. } = pd.kind {
                    if synthetic {
                        // Synthetic impl-trait param: leave the param
                        // unresolved; conversions are handled in
                        // resolve_and_classify.
                        return ty;
                    }
                }
                // A real type param of the enclosing function: fall through
                // to the instance-args lookup below. (Note: `tcx.type_of`
                // must NOT be called on a generic param's DefId — the query
                // ICEs with "unexpected non-type Node::GenericParam".)
            }
        }
    }
    match ty.kind() {
        // `type_at` would panic on a non-type arg (e.g. a lifetime); use the
        // fallible `as_type` projection so this never panics.
        ty::TyKind::Param(p) => outer
            .get(p.index as usize)
            .and_then(|a| a.as_type())
            .unwrap_or(ty),
        _ => ty,
    }
}

use rustc_middle::ty::print::with_no_trimmed_paths;

pub fn def_path(tcx: TyCtxt<'_>, did: DefId) -> String {
    // `def_path_str` honors path trimming (e.g. `String` instead of
    // `alloc::string::String`); disable it for stable table matching.
    normalize_path(&with_no_trimmed_paths!(tcx.def_path_str(did)))
}

/// `true` if `did` is defined in `std`, `core`, or `alloc` (as opposed to
/// user code or third-party crates). Used together with suffix matching so
/// that re-export renderings (`std::vec::...` vs `alloc::vec::...`) and
/// trimmed paths don't matter, while user types with std-like names never
/// match.
pub fn is_std_item(tcx: TyCtxt<'_>, did: DefId) -> bool {
    matches!(tcx.crate_name(did.krate).as_str(), "std" | "core" | "alloc")
}

/// `def_path_str` renders generic arguments (`Argument::<'_>::new_display`,
/// `Vec::<u8>::new`); erase all `<...>` groups so table matching is stable
/// across instantiations. Note this also erases `<impl ...>` headers, so
/// matching must be suffix/segment-based (see `recognize`).
pub fn normalize_path(p: &str) -> String {
    let mut out = String::with_capacity(p.len());
    let mut depth = 0u32;
    for c in p.chars() {
        match c {
            '<' => depth += 1,
            '>' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out
}

/// Match the last `segs.len()` `::` segments of a (normalized) path.
/// Immune to re-export prefixes, trimming, and `OsString`-style collisions.
/// Empty segments (left behind by erased `<...>` groups) are ignored.
pub fn ends_seg(path: &str, segs: &[&str]) -> bool {
    let parts: Vec<&str> = path.split("::").filter(|s| !s.is_empty()).collect();
    if parts.len() < segs.len() {
        return false;
    }
    parts[parts.len() - segs.len()..] == *segs
}

pub fn is_box_adt(tcx: TyCtxt<'_>, did: DefId) -> bool {
    tcx.is_lang_item(did, LangItem::OwnedBox)
}

/// `true` if `did` carries the `#[js::<name>]` tool attribute, whether it is
/// defined in the local crate (HIR) or upstream (crate metadata, e.g. the
/// `js_support` crate). The crate defining the item needs
/// `#![register_tool(js)]`.
pub fn has_js_attr(tcx: TyCtxt<'_>, did: DefId, name: &str) -> bool {
    let path = [
        rustc_span::Symbol::intern("js"),
        rustc_span::Symbol::intern(name),
    ];
    if let Some(local) = did.as_local() {
        let hir_id = tcx.local_def_id_to_hir_id(local);
        if tcx.hir_attrs(hir_id).iter().any(|a| a.path_matches(&path)) {
            return true;
        }
    }
    // Upstream items (and anything HIR misses): encoded attributes.
    // (Bound to a bool first so the query's temporary drops before `path`.)
    let found = tcx.get_attrs_by_path(did, &path).next().is_some();
    found
}

/// `true` if `did` names a `#[js::opaque]` ADT (an opaque JS object handle
/// such as a DOM node).
pub fn is_js_opaque(tcx: TyCtxt<'_>, did: DefId) -> bool {
    has_js_attr(tcx, did, "opaque")
}

/// `true` if `did` names a `#[js::inline]` function stub (a peered inline-JS
/// intrinsic whose first `&str` argument is the template and whose remaining
/// arguments are `$N` splices).
pub fn is_js_inline(tcx: TyCtxt<'_>, did: DefId) -> bool {
    has_js_attr(tcx, did, "inline")
}

/// Peel `&T`/`&mut T` (references are transparent in this backend).
pub fn peel_ref<'tcx>(ty: Ty<'tcx>) -> Ty<'tcx> {
    let mut t = ty;
    loop {
        match t.kind() {
            ty::Ref(_, inner, _) => t = *inner,
            _ => return t,
        }
    }
}

/// `true` if `ty` (through references) is an opaque JS handle.
pub fn is_js_value_ty<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    is_js_value_ty_inner(tcx, ty, 0)
}

fn is_js_value_ty_inner<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, depth: u8) -> bool {
    // Depth-guarded: recursive wrappers (`struct A(Box<A>)`) would cycle.
    if depth > 16 {
        return false;
    }
    match peel_ref(ty).kind() {
        ty::Adt(def, args) => {
            is_js_opaque(tcx, def.did())
                || handle_field_ty_inner(tcx, def, args, depth).is_some()
        }
        _ => false,
    }
}

/// Field type if `def` (with `args`) is a transparent single-field handle
/// wrapper: a non-enum struct with exactly one field whose type is itself
/// a JS handle. Backs structural transparency — a macro-generated `pub
/// struct Element(pub JsValue)` behaves as its handle with no attribute.
/// `None` otherwise (including multi-field `#[js::opaque]` structs, which
/// stay uninspectable objects).
pub fn handle_field_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    def: &ty::AdtDef<'tcx>,
    args: ty::GenericArgsRef<'tcx>,
) -> Option<Ty<'tcx>> {
    handle_field_ty_inner(tcx, def, args, 0)
}

fn handle_field_ty_inner<'tcx>(
    tcx: TyCtxt<'tcx>,
    def: &ty::AdtDef<'tcx>,
    args: ty::GenericArgsRef<'tcx>,
    depth: u8,
) -> Option<Ty<'tcx>> {
    if depth > 16 || def.is_enum() {
        return None;
    }
    // Field count first: no queries for ordinary structs.
    if def.non_enum_variant().fields.len() != 1 {
        return None;
    }
    let f = &def.non_enum_variant().fields[FieldIdx::from_usize(0)];
    let fty = subst_unchecked(tcx, args, tcx.type_of(f.did).skip_binder());
    is_js_value_ty_inner(tcx, fty, depth + 1).then_some(fty)
}

/// `true` if `ty` (through references) is std `Option<_>`.
pub fn is_option_ty<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    match peel_ref(ty).kind() {
        ty::Adt(def, _) => {
            is_std_item(tcx, def.did())
                && {
                    let p = def_path(tcx, def.did());
                    p.ends_with("option::Option") || p == "Option"
                }
        }
        _ => false,
    }
}

/// `true` if `ty` (through references) is std `Result<_, _>`.
pub fn is_result_ty<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    match peel_ref(ty).kind() {
        ty::Adt(def, _) => {
            is_std_item(tcx, def.did())
                && {
                    let p = def_path(tcx, def.did());
                    p.ends_with("result::Result") || p == "Result"
                }
        }
        _ => false,
    }
}

/// Last path segment of an ADT, or `None` for non-ADTs.
fn adt_last_segment<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<String> {
    let ty = match ty.kind() {
        ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => *inner,
        ty::Adt(..) => ty,
        _ => return None,
    };
    let (def, _) = match ty.kind() {
        ty::Adt(def, args) => (*def, *args),
        _ => return None,
    };
    if !is_std_item(tcx, def.did()) {
        return None;
    }
    Some(def_path(tcx, def.did()))
}

/// `true` if a value of this type is already an iterator (or a range), so
/// `into_iter` on it is the identity. Ranges normalize to their 3-element
/// form on first use instead.
pub fn is_iterator_ty<'tcx>(_tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    let Some(p) = adt_last_segment(_tcx, ty) else {
        return false;
    };
    let last = p.rsplit("::").next().unwrap_or("");
    matches!(
        last,
        "Iter"
            | "IterMut"
            | "IntoIter"
            | "Chars"
            | "Bytes"
            | "Lines"
            | "Enumerate"
            | "Map"
            | "Filter"
            | "FilterMap"
            | "FlatMap"
            | "Flatten"
            | "Zip"
            | "Chain"
            | "Take"
            | "Skip"
            | "StepBy"
            | "Rev"
            | "Cloned"
            | "Copied"
            | "Cycle"
            | "Peekable"
            | "Fuse"
            | "Split"
            | "SplitTerminator"
            | "Matches"
            | "Drain"
            | "Inspect"
            | "Scan"
            | "Range"
            | "RangeInclusive"
    )
}

/// Representation decision for a (monomorphized) type.
pub fn repr_of<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Repr {
    match ty.kind() {
        ty::Bool => Repr::Bool,
        ty::Char => Repr::Num(NumKind::Char),
        ty::Int(t) => match t {
            ty::IntTy::I8 => Repr::Num(NumKind::I8),
            ty::IntTy::I16 => Repr::Num(NumKind::I16),
            ty::IntTy::I32 => Repr::Num(NumKind::I32),
            ty::IntTy::I64 => Repr::Big(BigKind::I64),
            ty::IntTy::I128 => Repr::Big(BigKind::I128),
            ty::IntTy::Isize => Repr::Num(NumKind::Isize),
        },
        ty::Uint(t) => match t {
            ty::UintTy::U8 => Repr::Num(NumKind::U8),
            ty::UintTy::U16 => Repr::Num(NumKind::U16),
            ty::UintTy::U32 => Repr::Num(NumKind::U32),
            ty::UintTy::U64 => Repr::Big(BigKind::U64),
            ty::UintTy::U128 => Repr::Big(BigKind::U128),
            ty::UintTy::Usize => Repr::Num(NumKind::Usize),
        },
        ty::Float(_) => Repr::Float,
        ty::Str => Repr::Str,
        ty::Array(..) | ty::Slice(..) => Repr::Array,
        ty::Tuple(..) => Repr::Tuple,
        ty::Adt(def, args) => {
            // Transparent single-field handle wrappers behave as their
            // field (with or without the `#[js::opaque]` mark).
            if let Some(fty) = handle_field_ty(tcx, def, args) {
                return repr_of(tcx, fty);
            }
            // Opaque JS handles win over every other rule (including the
            // ZST/`Unit` fallback below: a field-less `#[js::opaque]` struct
            // is a live object reference, not `undefined`).
            if is_js_opaque(tcx, def.did()) {
                return Repr::JsValue;
            }
            if def.is_box() || is_box_adt(tcx, def.did()) {
                return Repr::Transparent;
            }
            match def.adt_kind() {
                AdtKind::Struct | AdtKind::Union => {
                    let in_std = is_std_item(tcx, def.did());
                    let p = def_path(tcx, def.did());
                    if in_std && (p.ends_with("string::String") || p == "String") {
                        return Repr::BufStr;
                    }
                    if in_std && (p.ends_with("vec::Vec") || p == "Vec") {
                        return Repr::Array;
                    }
                    // Rc/Arc are transparent (refcount ignored: single-threaded GC).
                    if in_std
                        && (p.ends_with("rc::Rc")
                            || p.ends_with("sync::Arc")
                            || p == "Rc"
                            || p == "Arc")
                    {
                        return Repr::Transparent;
                    }
                    // Borrow-guards and value wrappers are transparent.
                    if in_std
                        && (p.ends_with("cell::Ref")
                            || p.ends_with("cell::RefMut")
                            || p.ends_with("boxed::Unique")
                            || p.ends_with("ptr::NonNull")
                            || p.ends_with("ptr::Shared")
                            || p.ends_with("num::Wrapping")
                            || p.ends_with("num::NonZeroI8")
                            || p.ends_with("num::NonZeroI16")
                            || p.ends_with("num::NonZeroI32")
                            || p.ends_with("num::NonZeroI64")
                            || p.ends_with("num::NonZeroI128")
                            || p.ends_with("num::NonZeroIsize")
                            || p.ends_with("num::NonZeroU8")
                            || p.ends_with("num::NonZeroU16")
                            || p.ends_with("num::NonZeroU32")
                            || p.ends_with("num::NonZeroU64")
                            || p.ends_with("num::NonZeroU128")
                            || p.ends_with("num::NonZeroUsize")
                            || p == "Ref"
                            || p == "RefMut"
                            || p == "NonNull"
                            || p == "Shared"
                            || p == "Unique"
                            || p == "Wrapping")
                    {
                        return Repr::Transparent;
                    }
                    Repr::Tuple
                }
                AdtKind::Enum => Repr::Enum,
            }
        }
        ty::Ref(..) => Repr::Transparent,
        ty::RawPtr(..) => Repr::RawPtr,
        ty::FnDef(..) | ty::FnPtr(..) => Repr::FnVal,
        ty::Closure(..) | ty::CoroutineClosure(..) => Repr::Tuple,
        ty::Coroutine(..) => Repr::Opaque,
        ty::Dynamic(..) => Repr::Dyn,
        ty::Never => Repr::Never,
        ty::Foreign(..) => Repr::Opaque,
        ty::Pat(ty, _) => repr_of(tcx, *ty),
        // Param/Infer/Bound/Placeholder/Alias/Error: should not survive
        // substitution; treat as opaque (callers diagnose).
        _ => {
            if ty.is_unit() {
                Repr::Unit
            } else {
                Repr::Opaque
            }
        }
    }
}

/// `true` if the type is a ZST in a way we represent as `undefined`.
pub fn is_zst_like<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    if repr_of(tcx, ty) == Repr::Unit {
        return true;
    }
    matches!(
        ty.kind(),
        ty::FnDef(..) | ty::Closure(..) | ty::CoroutineClosure(..)
    )
}

/// Field type of an ADT field (substituted with the ADT's own args).
pub fn adt_field_ty<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
    variant_idx: VariantIdx,
    field_idx: usize,
) -> Option<Ty<'tcx>> {
    let (def, args) = match ty.kind() {
        ty::Adt(def, args) => (*def, *args),
        _ => return None,
    };
    let f = def
        .variant(variant_idx)
        .fields
        .get(FieldIdx::from_usize(field_idx))?;
    Some(subst_unchecked(tcx, args, tcx.type_of(f.did).skip_binder()))
}

/// Tuple element type.
pub fn tuple_field_ty<'tcx>(ty: Ty<'tcx>, idx: usize) -> Option<Ty<'tcx>> {
    match ty.kind() {
        ty::Tuple(tys) => tys.get(idx).copied(),
        _ => None,
    }
}

/// Upvar type of a closure (args already belong to the closure instance).
pub fn closure_upvar_ty<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, idx: usize) -> Option<Ty<'tcx>> {
    let _ = tcx;
    match ty.kind() {
        ty::Closure(_did, args) => args.as_closure().upvar_tys().get(idx).copied(),
        ty::CoroutineClosure(_did, args) => {
            args.as_coroutine_closure().upvar_tys().get(idx).copied()
        }
        _ => None,
    }
}

/// Discriminant *value* (not variant index) for an enum variant, as u128.
pub fn discriminant_value<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
    variant_idx: VariantIdx,
) -> Option<u128> {
    let (def, _) = match ty.kind() {
        ty::Adt(def, args) if def.is_enum() => (*def, args),
        _ => return None,
    };
    Some(def.discriminant_for_variant(tcx, variant_idx).val)
}

/// All (variant_idx, discriminant value) pairs for an enum type.
pub fn all_discriminants<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Vec<(VariantIdx, u128)>> {
    let (def, _) = match ty.kind() {
        ty::Adt(def, args) if def.is_enum() => (*def, args),
        _ => return None,
    };
    Some(def.discriminants(tcx).map(|(v, d)| (v, d.val)).collect())
}

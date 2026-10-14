//! Library-function recognition (§6.1, applied at emission time).
//!
//! Recognized calls never consult the callee's MIR: the backend owns
//! instruction selection (`v.push(x)` stays `v.push(x)`). Unrecognized
//! upstream functions are emitted from their (monomorphized) MIR, which is
//! what gives the backend breadth over the std API surface. Anything that
//! would touch raw memory, threads, or OS services and is not recognized
//! produces a hard diagnostic at the call site instead of miscompiling.

#![allow(dead_code)]

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, TyCtxt};

use crate::tyutil::{def_path, mono_env};

/// A recognized standard-library operation. Operands are passed separately;
/// each variant documents its MIR signature shape.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shim {
    // --- Vec ---
    VecNew,
    VecWithCapacity,
    VecPush,
    VecPop,
    VecLen,
    VecIsEmpty,
    VecClear,
    VecTruncate,
    VecRemove,
    VecInsert,
    VecFirst,
    VecLast,
    VecGet,
    VecClone,
    VecReserve,
    VecCapacity,
    VecSwapRemove,
    VecRetain,
    VecIntoIter,
    VecAsSlice,
    VecFromElem,
    VecFromArray,
    VecSetLen,
    SliceReverse,
    SliceSort,
    IterCount,
    IterLast,
    IterNth,
    IterPosition,
    IterFind,
    IterAny,
    IterAll,
    IterFold,
    IterForEach,
    IterSum,
    IterProduct,
    IterMin,
    IterMax,
    IterMinBy,
    IterMaxBy,
    IterMap,
    IterFilter,
    IterFilterMap,
    IterFlatMap,
    IterFlatten,
    IterRev,
    IterEnumerate,
    IterZip,
    IterChain,
    IterTake,
    IterSkip,
    IterStepBy,
    IterCloned,
    IterCopied,
    IterCollect,
    IterEq,
    // Option / Result
    OptUnwrap,
    OptIsSome,
    OptIsNone,
    OptMap,
    OptMapOr,
    OptMapOrElse,
    OptAndThen,
    OptOkOr,
    OptOkOrElse,
    OptTake,
    OptOrElse,
    OptUnwrapOr,
    OptUnwrapOrElse,
    OptUnwrapOrDefault,
    OptFilter,
    OptCopied,
    OptCloned,
    OptAsRef,
    OptAsDeref,
    OptContains,
    OptAnd,
    OptOr,
    OptXor,
    OptExpect,
    OptInspect,
    ResMap,
    ResMapErr,
    ResAndThen,
    ResOrElse,
    ResUnwrapOr,
    ResUnwrapOrElse,
    ResErr,
    ResIsOk,
    ResIsErr,
    ResOk,
    ResAsRef,
    ResCloned,
    ResCopied,
    ResUnwrap,
    ResExpect,
    ResContains,
    // `?` desugar: `Try::branch` + `FromResidual::from_residual`.
    TryBranch,
    TryFromResidual,
    // conversions / borrows
    CloneFn,
    IntoFn,
    AsRefFn,
    AsMutFn,
    BorrowFn,
    ToStringFn,
    // dgs PointRef conversions (From<&str>|From<String> for PointRef -> Named,
    // From<(f64,f64)> for PointRef -> Coords).
    PointRefFromStr,
    PointRefFromTuple,
    MustUseFn,
    RangeIncNew,
    RangeIncContains,
    WriteFmtTrait,
    VecExtendFromSlice,
    VecExtendFromWithin,
    VecAppend,
    VecSplitOff,
    VecIndex,
    VecIndexMut,
    // --- &[T] ---
    SliceLen,
    SliceIsEmpty,
    SliceGet,
    SliceFirst,
    SliceLast,
    SliceIter,
    SliceIterMut,
    SliceSplitAt,
    SliceToVec,
    SliceIndex,
    SliceIndexMut,
    SliceConcat,
    SliceJoin,
    SliceCopyFromSlice,
    SliceContains,
    SliceStartsWith,
    SliceEndsWith,
    StrSplitAt,
    StrGet,
    StrIndex,
    StrRepeat,
    StrToUpper,
    StrToLower,
    StrTrim,
    StrContains,
    StrStartsWith,
    StrEndsWith,
    StrFind,
    // --- slice::Iter / vec::IntoIter ---
    IterNext,
    IterNextBack,
    IterLen,
    IterSizeHint,
    IterAsSlice,
    // --- str::Chars / Bytes ---
    CharsNext,
    BytesNext,
    // --- String / str ---
    StringNew,
    StringFrom,
    StringPushStr,
    StringPush,
    StringLen,
    StringIsEmpty,
    StringAsStr,
    StringClone,
    StringClear,
    StringTruncate,
    StringPop,
    StringAdd,
    StrToOwned,
    StrLen,
    StrIsEmpty,
    StrAsBytes,
    StrChars,
    StrMatches,
    StrBytes,
    StrParse,
    // --- integer `pow` family ---
    IntPow,
    IntPowWrapping,
    IntPowChecked,
    IntPowOverflowing,
    IntPowSaturating,
    // --- char predicates / str patterns ---
    CharIsLowercase,
    CharIsUppercase,
    CharIsAlphabetic,
    CharIsNumeric,
    CharIsAlphanumeric,
    CharIsWhitespace,
    CharIsControl,
    CharIsAsciiLowercase,
    CharIsAsciiUppercase,
    CharIsAsciiDigit,
    CharIsAsciiAlphabetic,
    CharIsAsciiAlphanumeric,
    CharToAsciiLowercase,
    CharToAsciiUppercase,
    // --- Box / Rc / Arc ---
    BoxNew,
    BoxLeak,
    BoxNewUninit,
    BoxAssumeInitIntoVec,
    BoxDeref,
    BoxDerefMut,
    RcNew,
    RcClone,
    RcDeref,
    ArcNew,
    ArcClone,
    ArcDeref,
    // --- Cell / RefCell ---
    CellNew,
    CellGet,
    CellSet,
    CellReplace,
    RefCellNew,
    RefCellBorrow,
    RefCellBorrowMut,
    RefCellGetMut,
    RefDeref,
    RefMutDeref,
    RefMutDerefMut,
    RefClone,
    // --- mem / ptr / hint ---
    MemSwap,
    MemReplace,
    MemForget,
    PtrNull,
    PtrIsNull,
    HintBlackBox,
    HintLikely,
    HintSpinLoop,
    CharLenUtf8,
    // --- panic / catch_unwind ---
    CatchUnwind,
    // --- float method with const_assert! body (lowered directly) ---
    FloatClamp,
    // --- structural equality (bodies are byte/pointer shuffles) ---
    StructuralEq,
    StructuralNe,
    // --- primitive equality (inline `===`/`!==`, NaN-correct) ---
    PrimEq,
    PrimNe,
    // --- primitive operator-trait calls (`<i32 as Add>::add`, `!x`, ...) ---
    PrimAdd,
    PrimSub,
    PrimMul,
    PrimDiv,
    PrimRem,
    PrimBitAnd,
    PrimBitOr,
    PrimBitXor,
    PrimShl,
    PrimShr,
    PrimNeg,
    PrimNot,
    // --- integer min/max/clamp/abs (relational ternary: valid for both
    // --- the JS-number and the BigInt representations) ---
    IntMin,
    IntMax,
    IntClamp,
    IntAbs,
    IntWrappingAdd,
    IntWrappingSub,
    IntWrappingMul,
    IntWrappingNeg,
    IntWrappingShl,
    IntWrappingShr,
    IntSaturatingAdd,
    IntSaturatingSub,
    IntSaturatingMul,
    IntCheckedAdd,
    IntCheckedSub,
    IntCheckedMul,
    IntCheckedDiv,
    IntCheckedRem,
    IntCheckedNeg,
    IntOverflowingAdd,
    IntOverflowingSub,
    IntOverflowingMul,
    IntOverflowingNeg,
    // --- str -----
    StrReplace,
    StrStripPrefix,
    StrTrimStartMatches,
    // --- char -----
    CharEncodeUtf8,
    // --- String::from ---
    StringFromChar,
    // --- HashMap (hashbrown bodies are raw-memory; emulate at API level) ---
    MapNew,
    MapInsert,
    MapGet,
    MapGetMut,
    MapContainsKey,
    MapRemove,
    MapLen,
    MapIsEmpty,
    MapIter,
    MapKeys,
    MapValues,
    // --- vec extend specialization ---
    SpecExtend,
    // --- fmt (peer-only constructors handled separately) ---
    FmtEstimatedCapacity,
    FmtFromStr,
    FmtFormat,
    FmtWriteFmt,
    IoPrint,
}

/// Rust-level intrinsics (`core::intrinsics::*`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Intrinsic {
    Unreachable,
    Abort,
    SizeOf,
    AlignOf,
    NeedsDrop,
    VariantCount,
    DiscriminantValue,
    PtrGuaranteedCmp,
    WrappingAdd,
    WrappingSub,
    WrappingMul,
    SaturatingAdd,
    SaturatingSub,
    SaturatingMul,
    UncheckedAdd,
    UncheckedSub,
    UncheckedMul,
    UncheckedNeg,
    UncheckedShl,
    UncheckedShr,
    UncheckedDiv,
    UncheckedRem,
    IntegerMin,
    IntegerMax,
    AddWithOverflow,
    SubWithOverflow,
    MulWithOverflow,
    RotateLeft,
    RotateRight,
    Ctpop,
    Ctlz,
    Cttz,
    Bswap,
    Bitreverse,
    Float(FloatOp),
    Offset,
    Transmute,
    DropInPlace,
    UbChecks,
    OverflowChecks,
    Likely,
    BlackBox,
    CallerLocation,
    ConstEvalSelect,
    PtrMetadata,
    MoveValInit,
    Fence,
    IsValStaticallyKnown,
    AssertZeroValid, // assert_inhabited / assert_zero_valid / ... -> ()
    FloatArith(FloatArithOp),
    UnsupportedInt(&'static str),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FloatArithOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FloatOp {
    Sqrt,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Atan2,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Exp,
    Exp2,
    Log,
    Log2,
    Log10,
    Powf,
    Powi,
    Floor,
    Ceil,
    Trunc,
    Round,
    Fract,
    Abs,
    Min,
    Max,
    Minimum,
    Maximum,
    MulAdd,
    Copysign,
    Pow,
    RoundTiesEven,
    Fma,
    Log1p,
    ExpM1,
    Hypot,
    Cbrt,
    // f32/f64 inherent-method-only (no matching intrinsic spelling).
    IsNan,
    IsFinite,
    IsInfinite,
    IsSignPositive,
    IsSignNegative,
    Signum,
    Recip,
    ToDegrees,
    ToRadians,
    /// `x.log(base)` (two-arg); the unary `ln` maps to `Log`.
    LogBase,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PanicKind {
    Str,
    Fmt,
    BoundsCheck,
    Nounwind,
    AssertFailed,
    Misaligned,
    Generic,
}

/// `core::fmt::rt::Argument::new_*` constructor kinds (peer-only).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ArgNewKind {
    Display,
    Debug,
    Noop,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    Binary,
    LowerExp,
    UpperExp,
    Count,
}

pub enum Callee<'tcx> {
    /// Normal call to an emitted instance.
    Direct(ty::Instance<'tcx>),
    Shim(Shim),
    Intrinsic(Intrinsic),
    /// Tuple-struct / enum-variant constructor used as a value.
    CtorFn,
    /// Call to a `#[js_import]` foreign function.
    JsImport(DefId),
    Panic(PanicKind),
    /// `fmt::Arguments::new` — must be peered, never emitted.
    FmtNew,
    /// `fmt::rt::Argument::new_*` — must be peered, never emitted.
    ArgNew(ArgNewKind),
    /// `<Closure as FnOnce>::call_once` — call the closure body directly
    /// with (env, ...spread-tuple). Carries closure DefId + closure args.
    ClosureOnce(DefId, ty::GenericArgsRef<'tcx>),
    /// Blocklisted: threads, fs, time, HashMap, SIMD, ... (diagnosed).
    Unsupported(&'static str),
    /// Negated inner callee. Used for `PartialEq::ne`, which is virtually
    /// never overridden (`derive(PartialEq)` only generates `eq`, and
    /// clippy lints against overriding `ne`): a `!=` call would otherwise
    /// resolve to core's default `ne` body, which has no bundle to link.
    Not(Box<Callee<'tcx>>),
}

fn ends(p: &str, suffix: &str) -> bool {
    p == suffix || p.ends_with(suffix)
}

/// Like `ends`, but tolerates a trailing `<>` (generic arguments).
fn ends2(p: &str, suffix: &str) -> bool {
    ends(p, suffix) || p.ends_with(&format!("{suffix}<>"))
}

/// Does this DefId name a single-output conversion trait (Into<T> / From<S>)?
fn is_convert_trait(tcx: TyCtxt<'_>, def_id: rustc_hir::def_id::DefId) -> bool {
    let p = crate::tyutil::def_path(tcx, def_id);
    p.ends_with("convert::Into") || p.ends_with("convert::From") || p == "Into" || p == "From"
}

/// If `arg` is an opaque impl-trait param (`impl Into<T>` / `impl From<S>`)
/// whose projection has a single output type, return that output type
/// (the second alias arg for `Into<T>`/`From<S>`). Otherwise None.
fn subst_opaque_param<'tcx>(
    tcx: TyCtxt<'tcx>,
    arg: ty::GenericArg<'tcx>,
) -> Option<ty::GenericArg<'tcx>> {
    let kind = arg.kind();
    let ty::GenericArgKind::Type(ty) = kind else {
        return None;
    };
    match ty.kind() {
        ty::TyKind::Param(_) => None,
        ty::TyKind::Alias(_, alias_ty) => {
            match &alias_ty.kind {
                ty::AliasTyKind::Opaque { def_id } => {
                    // Opaque impl-trait param (e.g. the `impl Into<T>` param of
                    // `point_named`): the hidden type normalizes to the projection
                    // output (`<A as Into<T>>::Output` -> `T`), which is the type
                    // rustc uses to monomorphize `Into::into` at a call site.
                    let hidden = tcx.normalize_erasing_regions(
                        mono_env(),
                        tcx.type_of(*def_id).instantiate_identity(),
                    );
                    let is_conv = |ty: ty::Ty<'tcx>| match ty.kind() {
                        ty::TyKind::Alias(_, a) => match &a.kind {
                            ty::AliasTyKind::Projection { def_id } => {
                                let n = tcx.item_name(*def_id);
                                let t = tcx.parent(*def_id);
                                let trait_name = tcx.item_name(t);
                                n == rustc_span::symbol::sym::Output
                                    && (trait_name == rustc_span::symbol::sym::Into
                                        || trait_name == rustc_span::symbol::sym::From)
                            }
                            _ => false,
                        },
                        _ => false,
                    };
                    if is_conv(hidden) {
                        return Some(alias_ty.args[1]);
                    }
                    None
                }
                ty::AliasTyKind::Projection { .. } => None,
                _ => None,
            }
        }
        ty::TyKind::Error(_) => None,
        _ => None,
    }
}

/// Reasons that are always hard errors. Paths may render with any std
/// re-export prefix (`std::`/`core::`/`alloc::`) or trimmed, so matching is
/// segment-based; `in_std` (defining crate is std/core/alloc) excludes
/// user items with colliding names.
pub fn blocklisted(path: &str, in_std: bool) -> Option<&'static str> {
    if !in_std {
        return None;
    }
    let segs: Vec<&str> = path.split("::").collect();
    let has = |s: &str| segs.iter().any(|x| *x == s);
    if has("thread") {
        return Some("threads are not supported");
    }
    if has("fs") {
        return Some("the filesystem is not supported (same boundary as wasm32-unknown-unknown)");
    }
    if has("net") {
        return Some("networking is not supported");
    }
    if has("process") {
        return Some("processes are not supported");
    }
    if has("env") {
        return Some("environment access is not supported");
    }
    if has("os") {
        return Some("OS services are not supported");
    }
    if has("time") {
        return Some("system time is not supported");
    }
    if has("backtrace") {
        return Some("backtraces are not supported");
    }
    if has("sync") && !path.contains("Arc") {
        return Some("synchronization primitives are not supported");
    }
    if path.contains("collections::hash") || has("hashbrown") {
        return Some(
            "HashMap/HashSet with custom Hash/Eq need a real table runtime (not implemented)",
        );
    }
    if path.contains("collections::btree") {
        return Some("BTreeMap/BTreeSet need an ordered-table runtime (not implemented)");
    }
    if has("arch") || path.contains("simd") {
        return Some("SIMD intrinsics are rejected outright");
    }
    if has("libm") {
        return Some(
            "libm functions are not implemented (use the f32/f64 methods the backend supports)",
        );
    }
    if path.contains("stdin") {
        return Some("stdin is not supported");
    }
    None
}

/// `Into`/`From` dispatch using the *resolved* instance args (`[Self,
/// Target]`). Identity when both types match; otherwise the conversion body
/// is replaced by the representation-level equivalent (a `String` cell wrap
/// for `&str`/`char`, identity for any other convertible pair, whose bodies
/// emit normally). Returns `None` to fall through.
/// `Into`/`From` dispatch using the *resolved* instance's impl trait ref, so
/// the source/target types come from `impl From<Src> for Dst` itself rather
/// than from the call-site args (trait-default calls only carry `[Self]`).
fn conversion_of_inst<'tcx>(
    tcx: TyCtxt<'tcx>,
    inst: ty::Instance<'tcx>,
    path: &str,
    is_js_import: &dyn Fn(&DefId) -> bool,
) -> Option<Result<Callee<'tcx>, String>> {
    use crate::tyutil::ends_seg as es;
    // `def_path` erases `<...>` groups, so `<String as From<&str>>::from`
    // normalizes to `::from` and the trait is unreadable from text. Detect
    // it from the associated item's trait instead.
    let trait_name: Option<String> = match inst.def {
        ty::InstanceKind::Item(did2) => tcx
            .opt_associated_item(did2)
            .and_then(|a| a.trait_item_def_id())
            .map(|t| tcx.parent(t))
            .and_then(|t| tcx.opt_item_name(t))
            .map(|s| s.to_string()),
        _ => None,
    };
    let is_from = es(path, &["From", "from"]) || trait_name.as_deref() == Some("From");
    let is_into = es(path, &["Into", "into"]) || trait_name.as_deref() == Some("Into");
    if !(is_from || is_into) {
        return None;
    }
    // Instance args already carry fully-resolved Self + target (e.g.
    // <String as From<&str>>::from -> [String, &#x27; str]).
    // Const-generic impls (e.g. `impl<T, const N: usize> From<[T; N]> for
    // Vec<T>`, whose args are `[T, N]`) do NOT: arg #1 is a const, so
    // `type_at(1)` would ICE. Only take the fast path when both leading
    // args are types; otherwise read Self/Target from the impl trait ref.
    let first_two_types = inst.args.len() >= 2
        && matches!(inst.args[0].kind(), ty::GenericArgKind::Type(_))
        && matches!(inst.args[1].kind(), ty::GenericArgKind::Type(_));
    let (dst, src) = if first_two_types {
        if is_from {
            (
                crate::tyutil::subst_ty(tcx, inst.args, inst.args.type_at(0)),
                crate::tyutil::subst_ty(tcx, inst.args, inst.args.type_at(1)),
            )
        } else {
            (
                crate::tyutil::subst_ty(tcx, inst.args, inst.args.type_at(1)),
                crate::tyutil::subst_ty(tcx, inst.args, inst.args.type_at(0)),
            )
        }
    } else {
        // Monomorphized impl items (e.g. `<String as From<&str>>::from`)
        // carry only regions in `inst.args`; read Self/Target from the
        // impl's trait ref instead.
        let did2 = match inst.def {
            ty::InstanceKind::Item(did2) => did2,
            _ => return None,
        };
        let parent = tcx.parent(did2);
        if !matches!(tcx.def_kind(parent), rustc_hir::def::DefKind::Impl { .. }) {
            return None;
        }
        let tr = tcx.normalize_erasing_regions(
            mono_env(),
            tcx.impl_trait_ref(parent).instantiate(tcx, inst.args),
        );
        let mut ts = tr.args.iter().filter_map(|a| match a.kind() {
            ty::GenericArgKind::Type(t) => Some(crate::tyutil::subst_ty(tcx, inst.args, t)),
            _ => None,
        });
        let (self_ty, targ_ty) = match (ts.next(), ts.next()) {
            (Some(a), Some(b)) => (a, b),
            _ => return None,
        };
        if is_from {
            (self_ty, targ_ty)
        } else {
            (targ_ty, self_ty)
        }
    };
    // Reflexive impl (`impl From<T> for T`, `impl Into<T> for T`): identity.
    if dst == src {
        return Some(Ok(Callee::Shim(Shim::IntoFn)));
    }
    let is_string_ty = |ty: rustc_middle::ty::Ty<'tcx>| match ty.kind() {
        ty::Adt(def, _) if crate::tyutil::is_std_item(tcx, def.did()) => {
            let p = crate::tyutil::def_path(tcx, def.did());
            p.ends_with("string::String") || p == "String"
        }
        _ => false,
    };
    let dst_string = is_string_ty(dst);
    let src_str = matches!(
        match src.kind() {
            ty::Ref(_, t, _) => *t,
            _ => src,
        }
        .kind(),
        ty::Str
    );
    let src_string = is_string_ty(match src.kind() {
        ty::Ref(_, t, _) => *t,
        _ => src,
    });
    if dst_string && (src_str || src_string) {
        // `&str` / `String` / `&String` -> `String`: the 1-cell repr.
        return Some(Ok(Callee::Shim(Shim::StrToOwned)));
    }
    // dgs PointRef conversions: `&str` / `String` -> `PointRef::Named`,
    // `(f64, f64)` -> `PointRef::Coords`.
    let is_pointref = match dst.kind() {
        ty::Adt(def, ..) if is_js_import(&def.did()) => {
            crate::tyutil::ends_seg(&crate::tyutil::def_path(tcx, def.did()), &["PointRef"])
        }
        _ => false,
    };
    if is_pointref {
        match src.kind() {
            ty::Str => {
                return Some(Ok(Callee::Shim(Shim::PointRefFromStr)));
            }
            ty::Ref(_, t, _) if *t.kind() == ty::Str => {
                return Some(Ok(Callee::Shim(Shim::PointRefFromStr)));
            }
            _ if is_string_ty(src) => {
                return Some(Ok(Callee::Shim(Shim::PointRefFromStr)));
            }
            ty::Tuple(tys)
                if tys.len() == 2
                    && matches!(tys[0].kind(), ty::Float(ty::FloatTy::F64))
                    && matches!(tys[1].kind(), ty::Float(ty::FloatTy::F64)) =>
            {
                return Some(Ok(Callee::Shim(Shim::PointRefFromTuple)));
            }
            _ => {}
        }
    }
    if dst_string && matches!(src.kind(), ty::Char) {
        return Some(Ok(Callee::Shim(Shim::StringFromChar)));
    }
    // `<Vec<T> as From<[T; N]>>::from`: the impl-erased path is `::from`,
    // so dispatch on Self/Target types (both JS arrays: `.slice()` copies).
    // Covers `Vec::from(arr)`, `vec.into()` via `Into`, `From<&[T]>`/`From<&mut [T]>`
    // (peeled to `Slice`), and `From<Vec>`-by-ref (peeled to `Vec`).
    let is_vec_ty = |ty: rustc_middle::ty::Ty<'tcx>| match ty.kind() {
        ty::Adt(def, _) if crate::tyutil::is_std_item(tcx, def.did()) => {
            let p = crate::tyutil::def_path(tcx, def.did());
            p.ends_with("vec::Vec") || p == "Vec"
        }
        _ => false,
    };
    if is_vec_ty(dst) {
        let peeled = match src.kind() {
            ty::Ref(_, t, _) => *t,
            _ => src,
        };
        match peeled.kind() {
            ty::Array(..) | ty::Slice(_) => {
                return Some(Ok(Callee::Shim(Shim::VecFromArray)));
            }
            _ if is_vec_ty(peeled) => {
                return Some(Ok(Callee::Shim(Shim::VecFromArray)));
            }
            _ => {}
        }
    }
    // Other pairs: emit the real body (Into's forwards are trivial).
    None
}

/// `HashMap` public API → shim. Only the methods with a JS `Map` equivalent
/// are mapped; iteration order is unspecified in Rust for `HashMap`, so the
/// JS `Map` insertion order is a valid schedule.
pub fn classify_map(path: &str, in_std: bool) -> Option<Shim> {
    use crate::tyutil::ends_seg as es;
    if !in_std {
        return None;
    }
    use Shim::*;
    if (es(path, &["HashMap", "new"]) || es(path, &["HashMap", "with_capacity"]))
        || es(path, &["HashMap", "default"])
    {
        return Some(MapNew);
    }
    if es(path, &["HashMap", "insert"]) {
        return Some(MapInsert);
    }
    if es(path, &["HashMap", "get"]) {
        return Some(MapGet);
    }
    if es(path, &["HashMap", "get_mut"]) {
        return Some(MapGetMut);
    }
    if es(path, &["HashMap", "contains_key"]) {
        return Some(MapContainsKey);
    }
    if es(path, &["HashMap", "remove"]) || es(path, &["HashMap", "remove_entry"]) {
        return Some(MapRemove);
    }
    if es(path, &["HashMap", "len"]) {
        return Some(MapLen);
    }
    if es(path, &["HashMap", "is_empty"]) {
        return Some(MapIsEmpty);
    }
    if es(path, &["HashMap", "iter"])
        || es(path, &["HashMap", "iter_mut"])
        || es(path, &["HashMap", "into_iter"])
    {
        return Some(MapIter);
    }
    if es(path, &["HashMap", "keys"]) {
        return Some(MapKeys);
    }
    if es(path, &["HashMap", "values"]) || es(path, &["HashMap", "values_mut"]) {
        return Some(MapValues);
    }
    None
}

pub fn classify_shim(path: &str, in_std: bool) -> Option<Shim> {
    use crate::tyutil::ends_seg as es;
    if !in_std {
        return None;
    }
    // Suffix/segment matching: paths may render with any re-export prefix
    // (`std::`/`core::`/`alloc::`) or trimmed, and `<impl ...>` headers are
    // erased by normalization.
    let s = match path {
        // --- Iterator protocol (slice::Iter, vec::IntoIter, combinators) ---
        _ if es(path, &["Iter", "next"]) => Shim::IterNext,
        _ if es(path, &["Iter", "next_back"]) => Shim::IterNextBack,
        _ if es(path, &["iter", "next"]) => Shim::IterNext,
        _ if es(path, &["iter", "next_back"]) => Shim::IterNextBack,
        _ if es(path, &["Iter", "len"]) => Shim::IterLen,
        _ if es(path, &["iter", "len"]) => Shim::IterLen,
        _ if es(path, &["Iter", "size_hint"]) => Shim::IterSizeHint,
        _ if es(path, &["iter", "size_hint"]) => Shim::IterSizeHint,
        _ if es(path, &["Iter", "as_slice"]) => Shim::IterAsSlice,
        _ if es(path, &["iter", "as_mut_slice"]) => Shim::IterAsSlice,
        // Iterator default methods (lowered eagerly in this backend).
        _ if es(path, &["Iterator", "count"]) => Shim::IterCount,
        _ if es(path, &["Iterator", "last"]) => Shim::IterLast,
        _ if es(path, &["Iterator", "nth"]) => Shim::IterNth,
        _ if es(path, &["Iterator", "position"]) => Shim::IterPosition,
        _ if es(path, &["Iterator", "find"]) => Shim::IterFind,
        _ if es(path, &["Iterator", "any"]) => Shim::IterAny,
        _ if es(path, &["Iterator", "all"]) => Shim::IterAll,
        _ if es(path, &["Iterator", "fold"]) => Shim::IterFold,
        _ if es(path, &["Iterator", "reduce"]) => Shim::IterFold,
        _ if es(path, &["Iterator", "for_each"]) => Shim::IterForEach,
        _ if es(path, &["Iterator", "sum"]) => Shim::IterSum,
        _ if es(path, &["Iterator", "product"]) => Shim::IterProduct,
        _ if es(path, &["Iterator", "min"]) => Shim::IterMin,
        _ if es(path, &["Iterator", "max"]) => Shim::IterMax,
        _ if es(path, &["Iterator", "min_by"]) => Shim::IterMinBy,
        _ if es(path, &["Iterator", "max_by"]) => Shim::IterMaxBy,
        _ if es(path, &["Iterator", "map"]) => Shim::IterMap,
        _ if es(path, &["Iterator", "filter"]) => Shim::IterFilter,
        _ if es(path, &["Iterator", "filter_map"]) => Shim::IterFilterMap,
        _ if es(path, &["Iterator", "flat_map"]) => Shim::IterFlatMap,
        _ if es(path, &["Iterator", "flatten"]) => Shim::IterFlatten,
        _ if es(path, &["Iterator", "rev"]) => Shim::IterRev,
        _ if es(path, &["Iterator", "enumerate"]) => Shim::IterEnumerate,
        _ if es(path, &["Iterator", "zip"]) => Shim::IterZip,
        _ if es(path, &["Iterator", "chain"]) => Shim::IterChain,
        _ if es(path, &["Iterator", "take"]) => Shim::IterTake,
        _ if es(path, &["Iterator", "skip"]) => Shim::IterSkip,
        _ if es(path, &["Iterator", "step_by"]) => Shim::IterStepBy,
        _ if es(path, &["Iterator", "cloned"]) => Shim::IterCloned,
        _ if es(path, &["Iterator", "copied"]) => Shim::IterCopied,
        _ if es(path, &["Iterator", "collect"]) => Shim::IterCollect,
        _ if es(path, &["Iterator", "eq"]) => Shim::IterEq,
        _ if es(path, &["Iterator", "next"]) => Shim::IterNext,
        _ if es(path, &["Iterator", "next_back"]) => Shim::IterNextBack,
        _ if es(path, &["IntoIterator", "into_iter"]) => Shim::VecIntoIter,
        _ if es(path, &["Clone", "clone"]) => Shim::CloneFn,
        _ if es(path, &["ToString", "to_string"]) => Shim::ToStringFn,
        _ if es(path, &["hint", "must_use"]) => Shim::MustUseFn,
        _ if es(path, &["f32", "clamp"]) | es(path, &["f64", "clamp"]) => Shim::FloatClamp,
        _ if es(path, &["RangeInclusive", "new"]) => Shim::RangeIncNew,
        _ if es(path, &["RangeInclusive", "contains"]) => Shim::RangeIncContains,
        _ if es(path, &["Write", "write_fmt"]) => Shim::WriteFmtTrait,
        _ if es(path, &["SpecWriteFmt", "spec_write_fmt"]) => Shim::WriteFmtTrait,
        _ if es(path, &["Option", "unwrap"]) => Shim::OptUnwrap,
        _ if es(path, &["Option", "expect"]) => Shim::OptExpect,
        _ if es(path, &["Option", "is_some"]) => Shim::OptIsSome,
        _ if es(path, &["Option", "is_none"]) => Shim::OptIsNone,
        _ if es(path, &["Option", "map"]) => Shim::OptMap,
        _ if es(path, &["Option", "map_or"]) => Shim::OptMapOr,
        _ if es(path, &["Option", "map_or_else"]) => Shim::OptMapOrElse,
        _ if es(path, &["Option", "and_then"]) => Shim::OptAndThen,
        _ if es(path, &["Option", "take"]) => Shim::OptTake,
        _ if es(path, &["Option", "ok_or"]) => Shim::OptOkOr,
        _ if es(path, &["Option", "ok_or_else"]) => Shim::OptOkOrElse,
        _ if es(path, &["Option", "or_else"]) => Shim::OptOrElse,
        _ if es(path, &["Option", "unwrap_or"]) => Shim::OptUnwrapOr,
        _ if es(path, &["Option", "unwrap_or_else"]) => Shim::OptUnwrapOrElse,
        _ if es(path, &["Option", "unwrap_or_default"]) => Shim::OptUnwrapOrDefault,
        _ if es(path, &["Option", "filter"]) => Shim::OptFilter,
        _ if es(path, &["Option", "copied"]) => Shim::OptCopied,
        _ if es(path, &["Option", "cloned"]) => Shim::OptCloned,
        _ if es(path, &["Option", "as_ref"]) => Shim::OptAsRef,
        _ if es(path, &["Option", "as_deref"]) => Shim::OptAsDeref,
        _ if es(path, &["Option", "contains"]) => Shim::OptContains,
        _ if es(path, &["Option", "and"]) => Shim::OptAnd,
        _ if es(path, &["Option", "or"]) => Shim::OptOr,
        _ if es(path, &["Option", "xor"]) => Shim::OptXor,
        _ if es(path, &["Option", "inspect"]) => Shim::OptInspect,
        _ if es(path, &["Result", "map"]) => Shim::ResMap,
        _ if es(path, &["Result", "map_err"]) => Shim::ResMapErr,
        _ if es(path, &["Result", "and_then"]) => Shim::ResAndThen,
        _ if es(path, &["Result", "or_else"]) => Shim::ResOrElse,
        _ if es(path, &["Result", "unwrap_or"]) => Shim::ResUnwrapOr,
        _ if es(path, &["Result", "unwrap_or_else"]) => Shim::ResUnwrapOrElse,
        _ if es(path, &["Result", "err"]) => Shim::ResErr,
        _ if es(path, &["Result", "is_ok"]) => Shim::ResIsOk,
        _ if es(path, &["Result", "is_err"]) => Shim::ResIsErr,
        _ if es(path, &["Result", "ok"]) => Shim::ResOk,
        _ if es(path, &["Result", "as_ref"]) => Shim::ResAsRef,
        _ if es(path, &["Result", "cloned"]) => Shim::ResCloned,
        _ if es(path, &["Result", "copied"]) => Shim::ResCopied,
        _ if es(path, &["Result", "unwrap"]) => Shim::ResUnwrap,
        _ if es(path, &["Result", "expect"]) => Shim::ResExpect,
        _ if es(path, &["Result", "contains"]) => Shim::ResContains,
        // `?` desugar (`core::ops::try_trait`).
        _ if es(path, &["Try", "branch"]) => Shim::TryBranch,
        _ if es(path, &["FromResidual", "from_residual"]) => Shim::TryFromResidual,
        // --- str::Chars / Bytes ---
        _ if es(path, &["Chars", "next"]) => Shim::CharsNext,
        _ if es(path, &["Bytes", "next"]) => Shim::BytesNext,
        // --- Vec ---
        _ if es(path, &["Vec", "new"]) => Shim::VecNew,
        _ if es(path, &["Vec", "with_capacity"]) => Shim::VecWithCapacity,
        _ if es(path, &["Vec", "push"]) => Shim::VecPush,
        _ if es(path, &["Vec", "pop"]) => Shim::VecPop,
        _ if es(path, &["Vec", "len"]) => Shim::VecLen,
        _ if es(path, &["Vec", "is_empty"]) => Shim::VecIsEmpty,
        _ if es(path, &["Vec", "clear"]) => Shim::VecClear,
        _ if es(path, &["Vec", "truncate"]) => Shim::VecTruncate,
        _ if es(path, &["Vec", "remove"]) => Shim::VecRemove,
        _ if es(path, &["Vec", "insert"]) => Shim::VecInsert,
        _ if es(path, &["Vec", "first"]) => Shim::VecFirst,
        _ if es(path, &["Vec", "last"]) => Shim::VecLast,
        _ if es(path, &["Vec", "get"]) => Shim::VecGet,
        _ if es(path, &["Vec", "reserve"]) => Shim::VecReserve,
        _ if es(path, &["Vec", "reserve_exact"]) => Shim::VecReserve,
        _ if es(path, &["Vec", "shrink_to_fit"]) => Shim::VecReserve,
        _ if es(path, &["Vec", "shrink_to"]) => Shim::VecReserve,
        _ if es(path, &["Vec", "capacity"]) => Shim::VecCapacity,
        _ if es(path, &["Vec", "swap_remove"]) => Shim::VecSwapRemove,
        _ if es(path, &["Vec", "retain"]) => Shim::VecRetain,
        _ if es(path, &["Vec", "retain_mut"]) => Shim::VecRetain,
        _ if es(path, &["Vec", "into_iter"]) => Shim::VecIntoIter,
        _ if es(path, &["Vec", "as_slice"]) => Shim::VecAsSlice,
        _ if es(path, &["Vec", "as_mut_slice"]) => Shim::VecAsSlice,
        _ if es(path, &["vec", "from_elem"]) => Shim::VecFromElem,
        _ if es(path, &["Vec", "extend_from_slice"]) => Shim::VecExtendFromSlice,
        _ if es(path, &["Vec", "extend_from_within"]) => Shim::VecExtendFromWithin,
        _ if es(path, &["Vec", "append"]) => Shim::VecAppend,
        _ if es(path, &["Vec", "split_off"]) => Shim::VecSplitOff,
        _ if es(path, &["Vec", "clone"]) => Shim::VecClone,
        _ if es(path, &["Vec", "index"]) => Shim::VecIndex,
        _ if es(path, &["Vec", "index_mut"]) => Shim::VecIndexMut,
        _ if es(path, &["Deref", "deref"]) => Shim::BoxDeref,
        _ if es(path, &["DerefMut", "deref_mut"]) => Shim::BoxDerefMut,
        // Trait-impl forms: `<Vec<T> as Index>::index`, `<[T] as Index>::index`.
        _ if es(path, &["ops", "index"]) => Shim::SliceIndex,
        _ if es(path, &["Index", "index"]) => Shim::SliceIndex,
        _ if es(path, &["IndexMut", "index_mut"]) => Shim::SliceIndexMut,
        _ if es(path, &["Vec", "from"]) => Shim::VecFromArray,
        _ if es(path, &["slice", "reverse"]) => Shim::SliceReverse,
        _ if es(path, &["iter", "into_iter"]) => Shim::SliceIter,
        _ if es(path, &["slice", "sort"]) => Shim::SliceSort,
        _ if es(path, &["slice", "sort_unstable"]) => Shim::SliceSort,
        _ if es(path, &["slice", "to_owned"]) => Shim::SliceToVec,
        _ if es(path, &["ops", "index"]) => Shim::SliceIndex,
        // --- &[T] ---
        _ if es(path, &["slice", "len"]) || es(path, &["[T]", "len"]) => Shim::SliceLen,
        _ if es(path, &["slice", "is_empty"]) => Shim::SliceIsEmpty,
        _ if es(path, &["slice", "get"]) => Shim::SliceGet,
        _ if es(path, &["slice", "first"]) => Shim::SliceFirst,
        _ if es(path, &["slice", "last"]) => Shim::SliceLast,
        _ if es(path, &["slice", "iter"]) => Shim::SliceIter,
        _ if es(path, &["slice", "iter_mut"]) => Shim::SliceIterMut,
        _ if es(path, &["slice", "split_at"]) => Shim::SliceSplitAt,
        _ if es(path, &["slice", "to_vec"]) => Shim::SliceToVec,
        _ if es(path, &["ConvertVec", "to_vec"]) => Shim::SliceToVec,
        _ if es(path, &["slice", "index"]) => Shim::SliceIndex,
        _ if es(path, &["slice", "index_mut"]) => Shim::SliceIndexMut,
        _ if es(path, &["SliceConcatExt", "concat"]) || es(path, &["slice", "concat"]) => {
            Shim::SliceConcat
        }
        _ if es(path, &["slice", "copy_from_slice"]) => Shim::SliceCopyFromSlice,
        _ if es(path, &["SliceConcatExt", "join"]) || es(path, &["slice", "join"]) => {
            Shim::SliceJoin
        }
        _ if es(path, &["slice", "contains"]) => Shim::SliceContains,
        _ if es(path, &["slice", "starts_with"]) => Shim::SliceStartsWith,
        _ if es(path, &["slice", "ends_with"]) => Shim::SliceEndsWith,
        // --- String ---
        _ if es(path, &["String", "new"]) => Shim::StringNew,
        _ if es(path, &["String", "push_str"]) => Shim::StringPushStr,
        _ if es(path, &["String", "push"]) => Shim::StringPush,
        _ if es(path, &["String", "len"]) => Shim::StringLen,
        _ if es(path, &["String", "is_empty"]) => Shim::StringIsEmpty,
        _ if es(path, &["String", "as_str"]) => Shim::StringAsStr,
        _ if es(path, &["String", "as_bytes"]) => Shim::StrAsBytes,
        _ if es(path, &["String", "clear"]) => Shim::StringClear,
        _ if es(path, &["String", "truncate"]) => Shim::StringTruncate,
        _ if es(path, &["String", "pop"]) => Shim::StringPop,
        _ if es(path, &["String", "clone"]) => Shim::StringClone,
        _ if es(path, &["String", "add"]) => Shim::StringAdd,
        _ if es(path, &["String", "from"]) => Shim::StringFrom,
        _ if es(path, &["String", "write_fmt"]) => Shim::FmtWriteFmt,
        // --- str methods ---
        _ if es(path, &["str", "len"]) => Shim::StrLen,
        _ if es(path, &["str", "is_empty"]) => Shim::StrIsEmpty,
        _ if es(path, &["str", "as_bytes"]) => Shim::StrAsBytes,
        _ if es(path, &["str", "chars"]) => Shim::StrChars,
        _ if es(path, &["str", "bytes"]) => Shim::StrBytes,
        _ if es(path, &["str", "parse"]) => Shim::StrParse,
        _ if es(path, &["str", "replace"]) => Shim::StrReplace,
        _ if es(path, &["str", "strip_prefix"]) => Shim::StrStripPrefix,
        _ if es(path, &["str", "trim_start_matches"]) => Shim::StrTrimStartMatches,
        _ if es(path, &["char", "encode_utf8"]) => Shim::CharEncodeUtf8,
        _ if es(path, &["IntoIter", "as_slice"]) => Shim::IterAsSlice,
        _ if es(path, &["IntoIter", "as_mut_slice"]) => Shim::IterAsSlice,
        _ if es(path, &["SpecExtend", "spec_extend"]) => Shim::SpecExtend,
        // Integer pow family. Matching is by last segment only (`powf`,
        // `powi`, `powu` must not match: they are floats).
        _ if es(path, &["pow"]) => Shim::IntPow,
        _ if es(path, &["wrapping_pow"]) => Shim::IntPowWrapping,
        _ if es(path, &["checked_pow"]) => Shim::IntPowChecked,
        _ if es(path, &["overflowing_pow"]) => Shim::IntPowOverflowing,
        _ if es(path, &["saturating_pow"]) => Shim::IntPowSaturating,
        _ if es(path, &["str", "split_at"]) => Shim::StrSplitAt,
        _ if es(path, &["str", "get"]) => Shim::StrGet,
        _ if es(path, &["str", "index"]) || es(path, &["str", "index_mut"]) => Shim::StrIndex,
        _ if es(path, &["str", "repeat"]) => Shim::StrRepeat,
        _ if es(path, &["str", "to_uppercase"]) => Shim::StrToUpper,
        _ if es(path, &["str", "to_lowercase"]) => Shim::StrToLower,
        _ if es(path, &["str", "trim"]) => Shim::StrTrim,
        _ if es(path, &["str", "contains"]) => Shim::StrContains,
        _ if es(path, &["str", "starts_with"]) => Shim::StrStartsWith,
        _ if es(path, &["str", "ends_with"]) => Shim::StrEndsWith,
        _ if es(path, &["str", "find"]) => Shim::StrFind,
        // --- Box / Rc / Arc ---
        _ if es(path, &["Box", "new"]) => Shim::BoxNew,
        _ if es(path, &["Box", "leak"]) => Shim::BoxLeak,
        _ if es(path, &["Box", "new_uninit"]) => Shim::BoxNewUninit,
        _ if es(path, &["Box", "new_zeroed"]) => Shim::BoxNewUninit,
        _ if es(path, &["Box", "assume_init"]) => Shim::BoxAssumeInitIntoVec,
        _ if es(path, &["box_new_uninit"]) => Shim::BoxNewUninit,
        _ if es(path, &["box_new_zeroed"]) => Shim::BoxNewUninit,
        _ if es(path, &["box_assume_init_into_vec_unsafe"]) => Shim::BoxAssumeInitIntoVec,
        _ if es(path, &["Vec", "into_vec"]) => Shim::BoxAssumeInitIntoVec,
        _ if es(path, &["Vec", "set_len"]) => Shim::VecSetLen,
        _ if es(path, &["Box", "deref"]) => Shim::BoxDeref,
        _ if es(path, &["String", "deref"]) => Shim::BoxDeref,
        _ if es(path, &["Box", "deref_mut"]) => Shim::BoxDerefMut,
        _ if es(path, &["Rc", "new"]) => Shim::RcNew,
        _ if es(path, &["Rc", "clone"]) => Shim::RcClone,
        _ if es(path, &["Rc", "deref"]) => Shim::RcDeref,
        _ if es(path, &["Arc", "new"]) => Shim::ArcNew,
        _ if es(path, &["Arc", "clone"]) => Shim::ArcClone,
        _ if es(path, &["Arc", "deref"]) => Shim::ArcDeref,
        // --- Cell / RefCell ---
        _ if es(path, &["Cell", "new"]) => Shim::CellNew,
        _ if es(path, &["Cell", "get"]) => Shim::CellGet,
        _ if es(path, &["Cell", "set"]) => Shim::CellSet,
        _ if es(path, &["Cell", "replace"]) => Shim::CellReplace,
        _ if es(path, &["RefCell", "new"]) => Shim::RefCellNew,
        _ if es(path, &["RefCell", "borrow_mut"]) => Shim::RefCellBorrowMut,
        _ if es(path, &["RefCell", "borrow"]) => Shim::RefCellBorrow,
        _ if es(path, &["RefCell", "get_mut"]) => Shim::RefCellGetMut,
        _ if es(path, &["RefMut", "deref"]) => Shim::RefMutDeref,
        _ if es(path, &["RefMut", "deref_mut"]) => Shim::RefMutDerefMut,
        _ if es(path, &["Ref", "deref"]) => Shim::RefDeref,
        _ if es(path, &["Ref", "clone"]) => Shim::RefClone,
        // --- mem / ptr / hint / char ---
        _ if es(path, &["mem", "swap"]) => Shim::MemSwap,
        _ if es(path, &["mem", "replace"]) => Shim::MemReplace,
        _ if es(path, &["mem", "forget"]) => Shim::MemForget,
        _ if es(path, &["ptr", "null"]) => Shim::PtrNull,
        _ if es(path, &["ptr", "null_mut"]) => Shim::PtrNull,
        _ if es(path, &["ptr", "is_null"]) => Shim::PtrIsNull,
        _ if es(path, &["hint", "black_box"]) => Shim::HintBlackBox,
        _ if es(path, &["hint", "likely"]) => Shim::HintLikely,
        _ if es(path, &["hint", "unlikely"]) => Shim::HintLikely,
        _ if es(path, &["hint", "spin_loop"]) => Shim::HintSpinLoop,
        _ if es(path, &["char", "len_utf8"]) => Shim::CharLenUtf8,
        // --- char predicates (Unicode in the runtime, not in core's tables).
        // Matched on the last segment only: char methods live in
        // `core::char::methods`, so `["char", "is_lowercase"]` never matches. ---
        _ if es(path, &["is_lowercase"]) => Shim::CharIsLowercase,
        _ if es(path, &["is_uppercase"]) => Shim::CharIsUppercase,
        _ if es(path, &["is_alphabetic"]) => Shim::CharIsAlphabetic,
        _ if es(path, &["is_numeric"]) => Shim::CharIsNumeric,
        _ if es(path, &["is_alphanumeric"]) => Shim::CharIsAlphanumeric,
        _ if es(path, &["is_whitespace"]) => Shim::CharIsWhitespace,
        _ if es(path, &["is_control"]) => Shim::CharIsControl,
        _ if es(path, &["is_ascii_lowercase"]) => Shim::CharIsAsciiLowercase,
        _ if es(path, &["is_ascii_uppercase"]) => Shim::CharIsAsciiUppercase,
        _ if es(path, &["is_ascii_digit"]) => Shim::CharIsAsciiDigit,
        _ if es(path, &["is_ascii_alphabetic"]) => Shim::CharIsAsciiAlphabetic,
        _ if es(path, &["is_ascii_alphanumeric"]) => Shim::CharIsAsciiAlphanumeric,
        _ if es(path, &["to_ascii_lowercase"]) => Shim::CharToAsciiLowercase,
        _ if es(path, &["to_ascii_uppercase"]) => Shim::CharToAsciiUppercase,
        // Pattern searchers would need std bodies; match directly instead.
        _ if es(path, &["str", "matches"]) => Shim::StrMatches,
        // --- panic / unwind ---
        _ if es(path, &["panic", "catch_unwind"]) => Shim::CatchUnwind,
        // --- fmt ---
        _ if es(path, &["Arguments", "estimated_capacity"]) => Shim::FmtEstimatedCapacity,
        _ if es(path, &["Arguments", "from_str"]) => Shim::FmtFromStr,
        _ if es(path, &["Arguments", "from_str_nonconst"]) => Shim::FmtFromStr,
        _ if es(path, &["fmt", "format"]) => Shim::FmtFormat,
        _ if es(path, &["stdio", "_print"]) => Shim::IoPrint,
        _ => return None,
    };
    Some(s)
}

pub fn classify_panic(path: &str, in_std: bool) -> Option<PanicKind> {
    if !in_std {
        return None;
    }
    let path = path.strip_suffix("<>").unwrap_or(path);
    let k = match path {
        _ if ends(path, "core::panicking::panic") => PanicKind::Str,
        _ if ends(path, "core::panicking::panic_fmt") => PanicKind::Fmt,
        // `std::rt::panic_fmt` is a re-export of the same function
        // (`std/src/rt.rs`: `pub use core::panicking::panic_fmt`); some
        // expansions (e.g. `assert!` with format args) name it via `std`.
        _ if ends(path, "std::rt::panic_fmt") => PanicKind::Fmt,
        _ if ends(path, "core::panicking::panic_bounds_check") => PanicKind::BoundsCheck,
        _ if ends(path, "core::panicking::panic_nounwind") => PanicKind::Nounwind,
        _ if ends(path, "core::panicking::panic_nounwind_fmt") => PanicKind::Fmt,
        _ if ends(path, "core::panicking::assert_failed") => PanicKind::AssertFailed,
        _ if ends(
            path,
            "core::panicking::panic_misaligned_pointer_dereference",
        ) =>
        {
            PanicKind::Misaligned
        }
        _ if ends(path, "alloc::alloc::handle_alloc_error") => PanicKind::Generic,
        _ if path.starts_with("core::panicking::") => PanicKind::Generic,
        _ => return None,
    };
    Some(k)
}

pub fn classify_arg_new(path: &str, in_std: bool) -> Option<ArgNewKind> {
    if !in_std {
        return None;
    }
    use crate::tyutil::ends_seg as es;
    let k = match path {
        _ if es(path, &["Argument", "new_display"]) => ArgNewKind::Display,
        _ if es(path, &["Argument", "new_debug"]) => ArgNewKind::Debug,
        _ if es(path, &["Argument", "new_debug_noop"]) => ArgNewKind::Noop,
        _ if es(path, &["Argument", "new_octal"]) => ArgNewKind::Octal,
        _ if es(path, &["Argument", "new_lower_hex"]) => ArgNewKind::LowerHex,
        _ if es(path, &["Argument", "new_upper_hex"]) => ArgNewKind::UpperHex,
        _ if es(path, &["Argument", "new_pointer"]) => ArgNewKind::Pointer,
        _ if es(path, &["Argument", "new_binary"]) => ArgNewKind::Binary,
        _ if es(path, &["Argument", "new_lower_exp"]) => ArgNewKind::LowerExp,
        _ if es(path, &["Argument", "new_upper_exp"]) => ArgNewKind::UpperExp,
        _ if es(path, &["Argument", "from_usize"]) => ArgNewKind::Count,
        _ => return None,
    };
    Some(k)
}

pub fn classify_intrinsic(name: &str) -> Intrinsic {
    use FloatOp::*;
    use Intrinsic::*;
    match name {
        "unreachable" => Unreachable,
        "abort" => Abort,
        "size_of" => SizeOf,
        "align_of" => AlignOf,
        "needs_drop" => NeedsDrop,
        "variant_count" => VariantCount,
        "discriminant_value" => DiscriminantValue,
        "ptr_guaranteed_cmp" => PtrGuaranteedCmp,
        "wrapping_add" => WrappingAdd,
        "wrapping_sub" => WrappingSub,
        "wrapping_mul" => WrappingMul,
        "saturating_add" => SaturatingAdd,
        "saturating_sub" => SaturatingSub,
        "saturating_mul" => SaturatingMul,
        "unchecked_add" => UncheckedAdd,
        "unchecked_sub" => UncheckedSub,
        "unchecked_mul" => UncheckedMul,
        "unchecked_neg" => UncheckedNeg,
        "unchecked_shl" => UncheckedShl,
        "unchecked_shr" => UncheckedShr,
        "unchecked_div" => UncheckedDiv,
        "unchecked_rem" => UncheckedRem,
        "add_with_overflow" => AddWithOverflow,
        "sub_with_overflow" => SubWithOverflow,
        "mul_with_overflow" => MulWithOverflow,
        "rotate_left" => RotateLeft,
        "rotate_right" => RotateRight,
        "ctpop" => Ctpop,
        "ctlz" | "ctlz_nonzero" => Ctlz,
        "cttz" | "cttz_nonzero" => Cttz,
        "bswap" => Bswap,
        "bitreverse" => Bitreverse,
        "offset" | "arith_offset" => Offset,
        "transmute" | "transmute_unchecked" => Transmute,
        "drop_in_place" => DropInPlace,
        "ub_checks" => UbChecks,
        "overflow_checks" => OverflowChecks,
        "contract_checks" => UbChecks,
        "likely" | "unlikely" => Likely,
        "black_box" => BlackBox,
        "cold_path" => BlackBox,
        "caller_location" => CallerLocation,
        "const_eval_select" => ConstEvalSelect,
        "ptr_metadata" => PtrMetadata,
        "move_val_init" => MoveValInit,
        "fence" | "compiler_fence" => Fence,
        "is_val_statically_known" => IsValStaticallyKnown,
        "assert_inhabited" | "assert_zero_valid" | "assert_mem_uninitialized_valid" => {
            AssertZeroValid
        }
        "prefetch_read_instruction"
        | "prefetch_write_instruction"
        | "prefetch_read_data"
        | "prefetch_write_data" => AssertZeroValid,
        "assume" => AssertZeroValid, // statement-level assume is separate; call form -> ()
        // Floats (real spellings are `<op>f<width>`; f16/f128 stay
        // unsupported at the type level, so only f32/f64 are mapped).
        "sqrt" | "sqrtf32" | "sqrtf64" => Float(Sqrt),
        "sin" | "sinf32" => Float(Sin),
        "cos" | "cosf32" => Float(Cos),
        "tan" | "tanf32" => Float(Tan),
        "asin" | "asinf32" => Float(Asin),
        "acos" | "acosf32" => Float(Acos),
        "atan" | "atanf32" => Float(Atan),
        "atan2" | "atan2f32" => Float(Atan2),
        "sinh" | "sinhf32" => Float(Sinh),
        "cosh" | "coshf32" => Float(Cosh),
        "tanh" | "tanhf32" => Float(Tanh),
        "asinh" | "asinhf32" => Float(Asinh),
        "acosh" | "acoshf32" => Float(Acosh),
        "atanh" | "atanhf32" => Float(Atanh),
        "exp" | "expf32" => Float(Exp),
        "exp2" | "exp2f32" => Float(Exp2),
        "log" | "logf32" => Float(Log),
        "log2" | "log2f32" => Float(Log2),
        "log10" | "log10f32" => Float(Log10),
        "powf" | "powff32" | "powf32" | "powf64" => Float(Powf),
        "powi" | "powif32" | "powif64" => Float(Powi),
        "floor" | "floorf32" | "floorf64" => Float(Floor),
        "ceil" | "ceilf32" | "ceilf64" => Float(Ceil),
        "trunc" | "truncf32" | "truncf64" => Float(Trunc),
        "round" | "roundf32" | "roundf64" => Float(Round),
        "round_ties_even" | "round_ties_even_f32" | "round_ties_even_f64" => Float(RoundTiesEven),
        "fract" | "fractf32" => Float(Fract),
        "fabs" | "fabsf32" => Float(Abs),
        "minnumf32" | "minnumf64" => Float(Min),
        "maxnumf32" | "maxnumf64" => Float(Max),
        "minimum_number_nsz_f32" | "minimum_number_nsz_f64" => Float(Min),
        "maximum_number_nsz_f32" | "maximum_number_nsz_f64" => Float(Max),
        "minimumf32" | "minimumf64" => Float(Minimum),
        "maximumf32" | "maximumf64" => Float(Maximum),
        "mul_add" | "mul_addf32" | "fmuladdf32" | "fmuladdf64" => Float(MulAdd),
        "fmaf" | "fmaf32" | "fmaf64" => Float(Fma),
        "copysign" | "copysignf32" | "copysignf64" => Float(Copysign),
        "integer_min" => IntegerMin,
        "integer_max" => IntegerMax,
        "log1p" | "log1pf32" => Float(Log1p),
        "expm1" | "expm1f32" => Float(ExpM1),
        "hypot" | "hypotf32" => Float(Hypot),
        "cbrt" | "cbrtf32" => Float(Cbrt),
        "fadd_fast" => FloatArith(FloatArithOp::Add),
        "fsub_fast" => FloatArith(FloatArithOp::Sub),
        "fmul_fast" => FloatArith(FloatArithOp::Mul),
        "fdiv_fast" => FloatArith(FloatArithOp::Div),
        "frem_fast" => FloatArith(FloatArithOp::Rem),
        "fneg_fast" => FloatArith(FloatArithOp::Neg),
        _ if name.starts_with("simd_") => UnsupportedInt("SIMD intrinsics are rejected outright"),
        _ if name.starts_with("atomic_") => {
            UnsupportedInt("atomic intrinsics are not supported (single-threaded target)")
        }
        _ if name.starts_with("volatile_")
            || name.starts_with("nontemporal_")
            || name.starts_with("unaligned_") =>
        {
            UnsupportedInt("volatile/unaligned memory operations are not supported")
        }
        _ if name.starts_with("ptr_offset_from")
            || name.starts_with("const_allocate")
            || name.starts_with("raw_eq")
            || name.starts_with("three_way_compare") =>
        {
            UnsupportedInt("pointer-identity intrinsics are not supported")
        }
        _ if name.starts_with("copy") || name.starts_with("write_bytes") => {
            UnsupportedInt("raw memory copies are not supported")
        }
        _ if name.starts_with("read_via_copy") || name.starts_with("ptr_") => {
            UnsupportedInt("raw pointer intrinsics are not supported")
        }
        _ => UnsupportedInt("intrinsic is not supported"),
    }
}

/// Types that must have finished dropping without user code in our model.
/// `true` = drop is a no-op (dealloc / flag updates / refcount ticks).
pub fn noop_drop(path: &str, in_std: bool) -> bool {
    if !in_std {
        return false;
    }
    path.contains("cell::Ref")
        || path.contains("cell::BorrowRef")
        || path.contains("rc::Rc")
        || path.contains("sync::Arc")
        || path.contains("fmt::")
        || path.contains("cell::Cell")
}

/// Find the `eq` method of the `PartialEq` trait that declares `ne_did`.
fn ne_eq_sibling(tcx: TyCtxt<'_>, ne_did: DefId) -> Option<DefId> {
    let assoc = tcx.opt_associated_item(ne_did)?;
    // For an impl item, hop to the trait declaration first (for an
    // unresolvable trait item this falls back to the item itself, whose
    // parent may still own a same-named `eq`).
    let trait_ne = assoc.trait_item_def_id().unwrap_or(ne_did);
    let trait_did = tcx.parent(trait_ne);
    tcx.associated_items(trait_did)
        .filter_by_name_unhygienic(rustc_span::symbol::sym::eq)
        .next()
        .map(|item| item.def_id)
}

/// Map primitive operator-trait methods (`<i32 as Add>::add`, `!x`, ...) to
/// the same inline shapes used for MIR `BinOp`/`UnOp`. Without this these
/// resolve to core instances with no bundle to link.
fn classify_op_trait(tcx: TyCtxt<'_>, did: DefId) -> Option<Shim> {
    use crate::tyutil::is_std_item;
    let assoc = tcx.opt_associated_item(did)?;
    let trait_item = assoc.trait_item_def_id()?;
    let trait_did = tcx.parent(trait_item);
    if !is_std_item(tcx, trait_did) {
        return None;
    }
    let trait_name = tcx.opt_item_name(trait_did)?.as_str().to_string();
    let method = tcx.opt_item_name(did)?.as_str().to_string();
    let shim = match (trait_name.as_str(), method.as_str()) {
        ("Add", "add") => Shim::PrimAdd,
        ("Sub", "sub") => Shim::PrimSub,
        ("Mul", "mul") => Shim::PrimMul,
        ("Div", "div") => Shim::PrimDiv,
        ("Rem", "rem") => Shim::PrimRem,
        ("BitAnd", "bitand") => Shim::PrimBitAnd,
        ("BitOr", "bitor") => Shim::PrimBitOr,
        ("BitXor", "bitxor") => Shim::PrimBitXor,
        ("Shl", "shl") => Shim::PrimShl,
        ("Shr", "shr") => Shim::PrimShr,
        ("Neg", "neg") => Shim::PrimNeg,
        ("Not", "not") => Shim::PrimNot,
        ("AsRef", "as_ref") => Shim::AsRefFn,
        ("AsMut", "as_mut") => Shim::AsMutFn,
        ("Borrow", "borrow") => Shim::BorrowFn,
        ("BorrowMut", "borrow_mut") => Shim::BorrowFn,
        _ => return None,
    };
    // The concrete receiver type is not available here for monomorphized
    // impl items; emission validates the actual operand type.
    Some(shim)
}

/// Map an f32/f64 inherent method to the `FloatOp` it shares with the
/// same-named intrinsic (`x.round()` ≡ `roundf64(x)`). Without this these
/// resolve to core instances with no bundle to link. Only fires for real
/// std floats (`in_std` gate); the second-to-last path segment must be
/// `f32`/`f64` and the last the method name.
pub fn classify_float_method(path: &str, in_std: bool) -> Option<FloatOp> {
    use crate::tyutil::ends_seg as es;
    use FloatOp::*;
    if !in_std {
        return None;
    }
    let method = path.split("::").filter(|s| !s.is_empty()).last()?;
    if !(es(path, &["f32", method]) || es(path, &["f64", method])) {
        return None;
    }
    Some(match method {
        "sqrt" => Sqrt,
        "sin" => Sin,
        "cos" => Cos,
        "tan" => Tan,
        "asin" => Asin,
        "acos" => Acos,
        "atan" => Atan,
        "atan2" => Atan2,
        "sinh" => Sinh,
        "cosh" => Cosh,
        "tanh" => Tanh,
        "asinh" => Asinh,
        "acosh" => Acosh,
        "atanh" => Atanh,
        "exp" => Exp,
        "exp2" => Exp2,
        "ln" => Log,
        "log" => LogBase,
        "log2" => Log2,
        "log10" => Log10,
        "powf" => Powf,
        "powi" => Powi,
        "floor" => Floor,
        "ceil" => Ceil,
        "trunc" => Trunc,
        "round" => Round,
        "round_ties_even" => RoundTiesEven,
        "fract" => Fract,
        "abs" => Abs,
        // `min`/`max` ignore a lone NaN (yield the other argument);
        // `minimum`/`maximum` propagate NaN.
        "min" => Min,
        "max" => Max,
        "minimum" => Minimum,
        "maximum" => Maximum,
        "mul_add" => MulAdd,
        "hypot" => Hypot,
        "cbrt" => Cbrt,
        "copysign" => Copysign,
        "exp_m1" => ExpM1,
        "ln_1p" => Log1p,
        "signum" => Signum,
        "recip" => Recip,
        "to_degrees" => ToDegrees,
        "to_radians" => ToRadians,
        "is_nan" => IsNan,
        "is_finite" => IsFinite,
        "is_infinite" => IsInfinite,
        "is_sign_positive" => IsSignPositive,
        "is_sign_negative" => IsSignNegative,
        _ => return None,
    })
}

/// `true` when `did` is an inherent method whose impl Self type is an
/// integer. Inherent paths normalize away the `<impl i32>` segment (e.g.
/// `core::num::::wrapping_add`), so the receiver cannot be read from text.
fn inherent_int_self(tcx: TyCtxt<'_>, did: DefId) -> bool {
    use crate::tyutil::repr_of;
    let assoc = match tcx.opt_associated_item(did) {
        Some(assoc) => assoc,
        None => return false,
    };
    if assoc.trait_item_def_id().is_some() {
        return false;
    }
    let parent = tcx.parent(did);
    if !matches!(tcx.def_kind(parent), rustc_hir::def::DefKind::Impl { .. }) {
        return false;
    }
    let self_ty =
        tcx.normalize_erasing_regions(mono_env(), tcx.type_of(parent).instantiate_identity());
    matches!(
        repr_of(tcx, self_ty),
        crate::tyutil::Repr::Num(_) | crate::tyutil::Repr::Big(_)
    )
}

/// Map integer min/max/clamp/abs methods (and `cmp::{min,max}` free fns)
/// to relational shims. The ternary form is valid for both the JS-number
/// (≤32-bit) and BigInt (64/128-bit) representations, unlike `Math.min`.
pub fn classify_int_method(tcx: TyCtxt<'_>, did: DefId, path: &str, in_std: bool) -> Option<Shim> {
    use crate::tyutil::ends_seg as es;
    if !in_std {
        return None;
    }
    // `std::cmp::{min,max}` free functions take `(a, b)` directly.
    // `Ord::{min,max,clamp}` trait methods (e.g. `n.max(2)` — integers have
    // no inherent min/max) take `(self, other)` the same way.
    if es(path, &["cmp", "max"]) || es(path, &["Ord", "max"]) {
        return Some(Shim::IntMax);
    }
    if es(path, &["cmp", "min"]) || es(path, &["Ord", "min"]) {
        return Some(Shim::IntMin);
    }
    if es(path, &["Ord", "clamp"]) {
        return Some(Shim::IntClamp);
    }
    let method = path.split("::").filter(|s| !s.is_empty()).last()?;
    if !matches!(
        method,
        "min"
            | "max"
            | "clamp"
            | "abs"
            | "unsigned_abs"
            | "wrapping_add"
            | "wrapping_sub"
            | "wrapping_mul"
            | "wrapping_neg"
            | "wrapping_shl"
            | "wrapping_shr"
            | "saturating_add"
            | "saturating_sub"
            | "saturating_mul"
            | "checked_add"
            | "checked_sub"
            | "checked_mul"
            | "checked_div"
            | "checked_rem"
            | "checked_neg"
            | "overflowing_add"
            | "overflowing_sub"
            | "overflowing_mul"
            | "overflowing_neg"
    ) {
        return None;
    }
    let int_recv = [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    ]
    .iter()
    .any(|r| es(path, &[r, method]))
        || inherent_int_self(tcx, did);
    if !int_recv {
        return None;
    }
    Some(match method {
        "min" => Shim::IntMin,
        "max" => Shim::IntMax,
        "clamp" => Shim::IntClamp,
        "abs" | "unsigned_abs" => Shim::IntAbs,
        "wrapping_add" => Shim::IntWrappingAdd,
        "wrapping_sub" => Shim::IntWrappingSub,
        "wrapping_mul" => Shim::IntWrappingMul,
        "wrapping_neg" => Shim::IntWrappingNeg,
        "wrapping_shl" => Shim::IntWrappingShl,
        "wrapping_shr" => Shim::IntWrappingShr,
        "saturating_add" => Shim::IntSaturatingAdd,
        "saturating_sub" => Shim::IntSaturatingSub,
        "saturating_mul" => Shim::IntSaturatingMul,
        "checked_add" => Shim::IntCheckedAdd,
        "checked_sub" => Shim::IntCheckedSub,
        "checked_mul" => Shim::IntCheckedMul,
        "checked_div" => Shim::IntCheckedDiv,
        "checked_rem" => Shim::IntCheckedRem,
        "checked_neg" => Shim::IntCheckedNeg,
        "overflowing_add" => Shim::IntOverflowingAdd,
        "overflowing_sub" => Shim::IntOverflowingSub,
        "overflowing_mul" => Shim::IntOverflowingMul,
        "overflowing_neg" => Shim::IntOverflowingNeg,
        _ => return None,
    })
}

/// Resolve + classify a call target. Returns the instance for `Direct`.
#[allow(clippy::too_many_arguments)]
pub fn resolve_and_classify<'tcx>(
    tcx: TyCtxt<'tcx>,
    did: DefId,
    args: ty::GenericArgsRef<'tcx>,
    _span: rustc_span::Span,
    is_js_import: &dyn Fn(&DefId) -> bool,
    enclosing_params: Option<ty::GenericArgsRef<'tcx>>,
    enclosing_fn_def_id: Option<DefId>,
) -> Result<Callee<'tcx>, String> {
    let path = def_path(tcx, did);
    let in_std = crate::tyutil::is_std_item(tcx, did);
    // `PartialEq::ne` redirect: resolve the sibling `eq` for the same Self
    // and negate. Overriding `ne` inconsistently with `eq` violates the
    // trait contract (clippy::partialeq_ne_impl), so `!eq` is always correct.
    if crate::tyutil::ends_seg(&path, &["PartialEq", "ne"]) {
        if let Some(eq_did) = ne_eq_sibling(tcx, did) {
            match resolve_and_classify(
                tcx,
                eq_did,
                args,
                _span,
                is_js_import,
                enclosing_params,
                enclosing_fn_def_id,
            ) {
                Ok(inner) => return Ok(Callee::Not(Box::new(inner))),
                // Fall through: the `ne` flow below reports the failure.
                Err(_) => {}
            }
        }
    }
    // Subst free inference params in the callee's args against the enclosing
    // function's generic param types (opaque impl-trait params are concretized
    // below by subst_opaque_param), so `try_resolve` monomorphizes the call.
    let mut args = enclosing_params
        .map(|params| crate::tyutil::subst_free_params(tcx, params, args, enclosing_fn_def_id))
        .unwrap_or(args);
    // Handle `impl Into<T>::into` where `Self` is an unresolved impl-trait param:
    // in the isolated codegen `try_resolve` cannot monomorphize it, so emit an
    // identity conversion to `T` (the 2nd arg).
    if crate::tyutil::ends_seg(&path, &["Into", "into"]) && args.len() >= 2 {
        if let ty::TyKind::Param(p) = args.type_at(0).kind() {
            if let Some(fn_def_id) = enclosing_fn_def_id {
                let generics = tcx.generics_of(fn_def_id);
                if (p.index as usize) < generics.own_params.len() {
                    if let ty::GenericParamDefKind::Type {
                        synthetic: true, ..
                    } = generics.own_params[p.index as usize].kind
                    {
                        return Ok(Callee::Shim(Shim::IntoFn));
                    }
                }
            }
        }
    }
    if let Some(msg) = blocklisted(&path, in_std) {
        return Err(msg.to_string());
    }
    if tcx.is_foreign_item(did) {
        if is_js_import(&did) {
            return Ok(Callee::JsImport(did));
        }
        // `std::sys::cmath::*`: libm calls std float methods fall back
        // to (whichever ones lack an LLVM lowering on this target call
        // into cmath instead of an intrinsic).
        if crate::tyutil::is_std_item(tcx, did) {
            use crate::tyutil::ends_seg as es;
            use FloatOp::*;
            let op = if es(&path, &["cmath", "acos"]) {
                Acos
            } else if es(&path, &["cmath", "acosh"]) {
                Acosh
            } else if es(&path, &["cmath", "asin"]) {
                Asin
            } else if es(&path, &["cmath", "asinh"]) {
                Asinh
            } else if es(&path, &["cmath", "atan"]) {
                Atan
            } else if es(&path, &["cmath", "atan2"]) {
                Atan2
            } else if es(&path, &["cmath", "cosh"]) {
                Cosh
            } else if es(&path, &["cmath", "expm1"]) || es(&path, &["cmath", "expm1f"]) {
                ExpM1
            } else if es(&path, &["cmath", "hypot"]) || es(&path, &["cmath", "hypotf"]) {
                Hypot
            } else if es(&path, &["cmath", "log1p"]) || es(&path, &["cmath", "log1pf"]) {
                Log1p
            } else if es(&path, &["cmath", "sinh"]) {
                Sinh
            } else if es(&path, &["cmath", "tan"]) {
                Tan
            } else if es(&path, &["cmath", "tanh"]) {
                Tanh
            } else {
                return Err(format!(
                    "call to extern function `{path}` without #[js_import] (native linking is unavailable)"
                ));
            };
            return Ok(Callee::Intrinsic(Intrinsic::Float(op)));
        }
        return Err(format!(
            "call to extern function `{path}` without #[js_import] (native linking is unavailable)"
        ));
    }
    if in_std && crate::tyutil::ends_seg(&path, &["Arguments", "new"]) {
        return Ok(Callee::FmtNew);
    }
    // `to_owned` erased its `for str` discriminator; dispatch on Self.
    if in_std && crate::tyutil::ends_seg(&path, &["ToOwned", "to_owned"]) {
        let self_ty = crate::tyutil::subst_ty(tcx, args, args.type_at(0));
        match self_ty.kind() {
            ty::Str => return Ok(Callee::Shim(Shim::StrToOwned)),
            ty::Slice(_) => return Ok(Callee::Shim(Shim::SliceToVec)),
            _ => {}
        }
    }
    if let Some(k) = classify_arg_new(&path, in_std) {
        return Ok(Callee::ArgNew(k));
    }
    // Structural equality for string/slice/vec Self types: their bodies
    // are byte/pointer shuffles with no JS equivalent, while comparison is
    // structural (`__deep_eq`, with `String` cells normalized). MIR names
    // either the trait (`PartialEq::eq`) or the impl method (`str::eq`);
    // matching on the method name + Self type covers both, and only these
    // std Self types, so manual `PartialEq` impls keep emitting real logic.
    if in_std {
        let mname = tcx.opt_item_name(did).map(|nm| nm.as_str().to_string());
        if mname.as_deref() == Some("eq") || mname.as_deref() == Some("ne") {
            if !args.is_empty() {
                // Peel reference args (`&T == &U` compares `T` with `U`) and
                // transparent wrappers (`Box`/`Rc`/`Arc`, whose delegating
                // impls live in core/alloc with no bundle): all are
                // transparent, so the pointee's impl is directly callable.
                // Raw pointers are NOT peeled (address comparison must keep
                // its type). Note `PartialEq` is generic over `[Self, Rhs]`,
                // so every type arg is peeled, not just the first.
                let mut fixed: Vec<ty::GenericArg<'_>> = args.iter().collect();
                for g in fixed.iter_mut() {
                    if let ty::GenericArgKind::Type(t) = g.kind() {
                        *g = crate::tyutil::peel_eq_self(tcx, t).into();
                    }
                }
                args = tcx.mk_args(&fixed);
                let self_ty = crate::tyutil::subst_ty(tcx, args, args.type_at(0));
                let structural = match self_ty.kind() {
                    ty::Str | ty::Slice(_) => true,
                    ty::Adt(def, _) if crate::tyutil::is_std_item(tcx, def.did()) => {
                        let p = crate::tyutil::def_path(tcx, def.did());
                        // `__deep_eq` compares `{t, v}` shapes recursively
                        // (NaN-aware), so `Option`/`Result` use it directly
                        // instead of linking core's derived impls (no bundle).
                        // Other std containers (HashMap, ...) are NOT added:
                        // their JS reprs don't deep-compare correctly.
                        p.ends_with("string::String")
                            || p == "String"
                            || p.ends_with("vec::Vec")
                            || p == "Vec"
                            || p.ends_with("option::Option")
                            || p == "Option"
                            || p.ends_with("result::Result")
                            || p == "Result"
                    }
                    _ => false,
                };
                if structural {
                    return Ok(Callee::Shim(if mname.as_deref() == Some("ne") {
                        Shim::StructuralNe
                    } else {
                        Shim::StructuralEq
                    }));
                }
                // Primitive Self (bool/char/int/float): inline `===`/`!==`
                // exactly like `BinOp` (NaN-correct, unlike `__deep_eq`).
                // References were peeled above, so this also covers `&f64`
                // field comparisons inside derived impls.
                match self_ty.kind() {
                    ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) | ty::Float(_) => {
                        return Ok(Callee::Shim(if mname.as_deref() == Some("ne") {
                            Shim::PrimNe
                        } else {
                            Shim::PrimEq
                        }));
                    }
                    _ => {}
                }
            }
        }
    }
    if let Some(k) = classify_panic(&path, in_std) {
        return Ok(Callee::Panic(k));
    }
    // `HashMap` (and `hashbrown`, its implementation crate) operate on raw
    // memory. Intercept the public API and emulate it with JS `Map`s;
    // everything below `HashMap::*` is unsupported. Must run before the
    // blocklist: hasher paths contain "thread" (RandomState pulls in
    // `sys::thread_local`), which the blocklist would reject first.
    if let Some(shim) = classify_map(&path, in_std) {
        return Ok(Callee::Shim(shim));
    }
    if let Some(msg) = blocklisted(&path, in_std) {
        return Ok(Callee::Unsupported(msg));
    }

    if let Some(shim) = classify_shim(&path, in_std) {
        return Ok(Callee::Shim(shim));
    }
    // f32/f64 inherent methods share the float-intrinsic ops (`x.round()`
    // ≡ `roundf64(x)`); without this they resolve to core instances with
    // no bundle to link.
    if let Some(op) = classify_float_method(&path, in_std) {
        return Ok(Callee::Intrinsic(Intrinsic::Float(op)));
    }
    // Integer min/max/clamp/abs (methods and `cmp::{min,max}` free fns).
    if let Some(shim) = classify_int_method(tcx, did, &path, in_std) {
        return Ok(Callee::Shim(shim));
    }
    // `<String as Deref>::deref` — trait method whose path loses the type
    // name after normalization; dispatch on Self.
    if in_std && crate::tyutil::ends_seg(&path, &["Deref", "deref"]) {
        let self_ty = crate::tyutil::subst_ty(
            tcx,
            args,
            args.iter()
                .find_map(|a| match a.kind() {
                    ty::GenericArgKind::Type(t) => Some(t),
                    _ => None,
                })
                .unwrap_or(tcx.types.str_),
        );
        if let ty::Adt(def, _) = self_ty.kind() {
            let p = def_path(tcx, def.did());
            if crate::tyutil::is_std_item(tcx, def.did())
                && (p.ends_with("string::String") || p == "String")
            {
                return Ok(Callee::Shim(Shim::BoxDeref));
            }
        }
    }
    let env = crate::tyutil::mono_env();
    let inst = match ty::Instance::try_resolve(tcx, env, did, args) {
        Ok(Some(inst)) => {
            let concrete: Vec<ty::GenericArg<'tcx>> = inst
                .args
                .iter()
                .map(|arg| subst_opaque_param(tcx, arg).unwrap_or(arg))
                .collect();
            if inst.args != tcx.mk_args(&concrete) {
                let concrete = tcx.mk_args(&concrete);
                match ty::Instance::try_resolve(tcx, env, did, concrete) {
                    Ok(Some(i)) => i,
                    _ => return Err(format!("could not monomorphize `{path}`")),
                }
            } else {
                inst
            }
        }
        Ok(None) => {
            // `args` can contain unresolved opaque impl-trait params
            // (`impl Into<T>` -> `Alias(Opaque, Into<T>)`): the item's MIR
            // params are already lowered to the projection's output type
            // and the caller passes concrete values, so re-resolve by
            // substituting each opaque with the projection's output type
            // (args[1] for `Into<T>`/`From<S>`), reproducing the concrete
            // instance rustc would build at a call site.
            let concrete: Vec<ty::GenericArg<'tcx>> = args
                .iter()
                .map(|arg| subst_opaque_param(tcx, arg).unwrap_or(arg))
                .collect();
            let concrete = tcx.mk_args(&concrete);
            match ty::Instance::try_resolve(tcx, env, did, concrete) {
                Ok(Some(inst)) => inst,
                _ => return Err(format!("could not monomorphize `{path}`")),
            }
        }
        Err(_) => return Err(format!("could not resolve `{path}`")),
    };
    // Conversions that need the *resolved* types (`Into`/`From`): the call
    // site may only carry the trait's own params, so dispatch on `inst.args`
    // ([Self, Target] for `From`/`Into` impls).
    match inst.def {
        ty::InstanceKind::Item(did2) => {
            if let Some(res) = conversion_of_inst(tcx, inst, &path, is_js_import) {
                return res;
            }
            if matches!(tcx.def_kind(did2), rustc_hir::def::DefKind::Ctor(..)) {
                return Ok(Callee::CtorFn);
            }
            let p2 = def_path(tcx, did2);
            let in_std2 = crate::tyutil::is_std_item(tcx, did2);
            if let Some(k) = classify_panic(&p2, in_std2) {
                return Ok(Callee::Panic(k));
            }
            if let Some(msg) = blocklisted(&p2, in_std2) {
                return Ok(Callee::Unsupported(msg));
            }
            if let Some(shim) = classify_shim(&p2, in_std2) {
                return Ok(Callee::Shim(shim));
            }
            if let Some(shim) = classify_op_trait(tcx, did2) {
                return Ok(Callee::Shim(shim));
            }
            Ok(Callee::Direct(inst))
        }
        ty::InstanceKind::Intrinsic(did2) => {
            let name = tcx
                .opt_item_name(did2)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "intrinsic".to_string());
            Ok(Callee::Intrinsic(classify_intrinsic(&name)))
        }
        ty::InstanceKind::LlvmIntrinsic(did2) => {
            let name = tcx
                .opt_item_name(did2)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "llvm-intrinsic".to_string());
            Err(format!("llvm intrinsic `{name}` is not supported"))
        }
        ty::InstanceKind::Virtual(..) => {
            Err("dynamic dispatch (`dyn Trait`) is not supported".to_string())
        }
        ty::InstanceKind::Shim(shim) => match shim {
            ty::ShimKind::ClosureOnce { closure, .. } => {
                // `FnOnce::call_once(self, args_tuple)`: call the closure
                // body directly as `name(env, ...tuple)`.
                let closure_ty = args.type_at(0);
                match closure_ty.kind() {
                    ty::Closure(_, cargs) => Ok(Callee::ClosureOnce(closure, cargs)),
                    _ => Err("FnOnce::call_once on non-closure".to_string()),
                }
            }
            _ => Err("compiler-generated shim calls are not supported".to_string()),
        },
    }
}

//! MIR constant rendering (§4 number model).
//!
//! Every Rust constant becomes a JS expression: numbers stay numbers,
//! `i64/u64/i128/u128` become BigInt literals, `&str` becomes a JS string,
//! float literals use shortest-round-trip formatting.

#![allow(dead_code)]

use rustc_abi::Size;
use rustc_middle::mir;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::Span;

use crate::tyutil::{mono_env, subst_ty, subst_unchecked};

pub struct ConstCx<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub args: ty::GenericArgsRef<'tcx>,
    pub span: Span,
}

impl<'tcx> ConstCx<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, args: ty::GenericArgsRef<'tcx>, span: Span) -> Self {
        ConstCx { tcx, args, span }
    }
    fn ty(&self, ty: Ty<'tcx>) -> Ty<'tcx> {
        subst_ty(self.tcx, self.args, ty)
    }
}

/// Render a `mir::Const` to a JS expression.
pub fn render_mir_const<'tcx>(cx: &ConstCx<'tcx>, c: mir::Const<'tcx>) -> Result<String, String> {
    let ty = cx.ty(c.ty());

    match c {
        mir::Const::Val(cv, _) => render_const_value(cx, cv, ty),
        mir::Const::Ty(_, ct) => {
            let ct = subst_unchecked(cx.tcx, cx.args, ct);
            render_ty_const(cx, ct, ty)
        }
        mir::Const::Unevaluated(uc, _) => {
            let args = subst_unchecked(cx.tcx, cx.args, uc.args);
            let uc = mir::UnevaluatedConst {
                def: uc.def,
                args,
                promoted: uc.promoted,
            };
            // Peel auto-ref promotion chains first: `&&T` (operator autoref
            // applied to a promoted `&T`) evaluates to a pointer scalar the
            // renderer cannot follow, but the promoted body materializes the
            // inner `&T` const, which the single-reference paths render.
            if let Some(s) = peel_promoted_ref(cx, &uc, ty, 0) {
                return s;
            }
            match cx.tcx.const_eval_resolve(mono_env(), uc, cx.span) {
                Ok(cv) => render_const_value(cx, cv, ty),
                Err(_) => Err(format!("could not evaluate constant of type {ty}")),
            }
        }
    }
}

/// Render a reference-to-reference promotion (`&&T`) by walking the
/// promoted body to the inner `&T` (or `T`) const and rendering that.
/// Operator desugar borrows both sides (`a == b` becomes `eq(&a, &b)`), so
/// comparing two shared references produces `&&T` promotion consts; the
/// evaluated form is a pointer scalar into a static, which has no JS
/// equivalent, but the promotion body (`_v = const C; _r = &_v`) names the
/// materialized const directly. Reads through `tcx.promoted_mir` — the same
/// query const-eval uses — so it works for upstream crates too.
/// Render an aggregate of const operands (unit enum variants like
/// `Token::Caret`, `Some('.')`) as its JS value. Returns `None` when any
/// operand is not a plain const.
fn render_promoted_aggregate<'tcx>(
    cx: &ConstCx<'tcx>,
    kind: &mir::AggregateKind<'tcx>,
    ops: &rustc_index::IndexVec<rustc_abi::FieldIdx, mir::Operand<'tcx>>,
) -> Option<String> {
    use crate::tyutil::discriminant_value;
    let mut out = Vec::with_capacity(ops.len());
    for op in ops.iter() {
        if let mir::Operand::Constant(c) = op {
            out.push(render_mir_const(cx, c.const_.clone()).ok()?);
        } else {
            return None;
        }
    }
    match kind {
        mir::AggregateKind::Tuple | mir::AggregateKind::Array(_) => {
            Some(format!("[{}]", out.join(",")))
        }
        mir::AggregateKind::Adt(did, variant, args, _, _) => {
            let args = crate::tyutil::subst_args(cx.tcx, cx.args, args);
            let adt = cx.tcx.adt_def(*did);
            if adt.is_enum() {
                let enum_ty = ty::Ty::new_adt(cx.tcx, adt, args);
                let d = discriminant_value(cx.tcx, enum_ty, *variant)?;
                Some(format!("{{t:{d},v:[{}]}}", out.join(",")))
            } else {
                Some(format!("[{}]", out.join(",")))
            }
        }
        _ => None,
    }
}

fn peel_promoted_ref<'tcx>(
    cx: &ConstCx<'tcx>,
    uc: &mir::UnevaluatedConst<'tcx>,
    ty: Ty<'tcx>,
    depth: usize,
) -> Option<Result<String, String>> {
    if depth > 8 {
        return None;
    }
    if !matches!(ty.kind(), ty::Ref(_, inner, _) if matches!(inner.kind(), ty::Ref(..))) {
        return None;
    }
    let prom = uc.promoted?;
    let proms = cx.tcx.promoted_mir(uc.def);
    let body = proms.get(prom)?;
    // Map every assigned place to its rvalue, then follow the borrow chain
    // from the return place (`_0 = &_1`, `_1 = &_2`, ...) down to the
    // materialized value: either a const or an aggregate of consts
    // (`_2 = Token::Caret`, `_2 = Option::Some(const '.')`).
    use std::collections::HashMap;
    let mut assigns: HashMap<mir::Place<'tcx>, &mir::Rvalue<'tcx>> = HashMap::new();
    for bb in body.basic_blocks.iter() {
        for st in &bb.statements {
            if let mir::StatementKind::Assign(bx) = &st.kind {
                let (place, rvalue) = &**bx;
                assigns.insert(*place, rvalue);
            }
        }
    }
    let mut cur = mir::Place::return_place();
    let mut steps = 0usize;
    let inner = loop {
        if steps > 8 {
            return None;
        }
        steps += 1;
        let rvalue = assigns.get(&cur)?;
        match rvalue {
            mir::Rvalue::Ref(.., p) => cur = *p,
            mir::Rvalue::Use(mir::Operand::Constant(c), _) => {
                break c.const_.clone();
            }
            mir::Rvalue::Aggregate(kind, ops) => {
                return render_promoted_aggregate(cx, kind, ops)
                    .map(Ok)
                    .or_else(|| {
                        // Not const-only: give up (falls back to eval).
                        None
                    });
            }
            _ => return None,
        }
    };
    // Recurse (the inner const may itself be a peeled chain).
    let inner_ty = cx.ty(inner.ty());
    if matches!(inner_ty.kind(), ty::Ref(_, i2, _) if matches!(i2.kind(), ty::Ref(..))) {
        if let mir::Const::Unevaluated(uc2, _) = &inner {
            let args2 = subst_unchecked(cx.tcx, cx.args, uc2.args);
            let uc2 = mir::UnevaluatedConst {
                def: uc2.def,
                args: args2,
                promoted: uc2.promoted,
            };
            return peel_promoted_ref(cx, &uc2, inner_ty, depth + 1);
        }
    }
    Some(render_mir_const(cx, inner))
}

fn render_ty_const<'tcx>(
    cx: &ConstCx<'tcx>,
    ct: ty::Const<'tcx>,
    ty: Ty<'tcx>,
) -> Result<String, String> {
    use ty::ConstKind;
    match ct.kind() {
        ConstKind::Value(vc) => {
            if let Some(s) = vc.try_to_scalar() {
                return scalar_to_js(cx, s, vc.ty);
            }
            if let Some(bytes) = vc.try_to_raw_bytes(cx.tcx) {
                return match ty.kind() {
                    ty::Str | ty::Ref(..) => Ok(js_string(bytes)),
                    _ => Ok(format!(
                        "[{}]",
                        bytes
                            .iter()
                            .map(|b| b.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    )),
                };
            }
            if let Some(branch) = vc.try_to_branch() {
                return render_branch(cx, branch, ty);
            }
            Err(format!("unsupported constant value of type {ty}"))
        }
        ConstKind::Alias(..) => {
            // Best effort: evaluate array lengths and friends.
            match eval_to_value(cx, ct) {
                Some(cv) => render_const_value(cx, cv, ty),
                None => Err(format!("could not evaluate constant of type {ty}")),
            }
        }
        _ => Err(format!("unsupported generic constant of type {ty}")),
    }
}

fn eval_to_value<'tcx>(cx: &ConstCx<'tcx>, ct: ty::Const<'tcx>) -> Option<mir::ConstValue> {
    use ty::ConstKind;
    let alias = match ct.kind() {
        ConstKind::Alias(_, a) => a,
        _ => return None,
    };
    // Only plain anon consts / free consts can go through UnevaluatedConst.
    let def = match alias.kind {
        ty::AliasConstKind::Anon { def_id } => def_id.into(),
        ty::AliasConstKind::Free { def_id } => def_id.into(),
        _ => return None,
    };
    let args = subst_unchecked(cx.tcx, cx.args, alias.args);
    let uc = mir::UnevaluatedConst::new(def, args);
    cx.tcx.const_eval_resolve(mono_env(), uc, cx.span).ok()
}

/// Evaluate a length-like const (for `[x; N]`, array types).
pub fn eval_usize<'tcx>(cx: &ConstCx<'tcx>, ct: ty::Const<'tcx>) -> Result<u64, String> {
    let ct = subst_unchecked(cx.tcx, cx.args, ct);
    if let Some(n) = ct.try_to_target_usize(cx.tcx) {
        return Ok(n);
    }
    if let Some(vc) = ct.try_to_value() {
        if let Some(n) = vc.try_to_target_usize(cx.tcx) {
            return Ok(n);
        }
    }
    match eval_to_value(cx, ct).and_then(|cv| cv.try_to_target_usize(cx.tcx)) {
        Some(n) => Ok(n),
        None => Err("could not evaluate array length".to_string()),
    }
}

fn render_branch<'tcx>(
    cx: &ConstCx<'tcx>,
    branch: &[ty::Const<'tcx>],
    ty: Ty<'tcx>,
) -> Result<String, String> {
    match ty.kind() {
        ty::Array(elem_ty, _) | ty::Slice(elem_ty) => {
            let mut out = Vec::with_capacity(branch.len());
            for ct in branch {
                out.push(render_ty_const(cx, *ct, *elem_ty)?);
            }
            Ok(format!("[{}]", out.join(",")))
        }
        ty::Tuple(tys) => {
            if branch.len() != tys.len() {
                return Err(format!("tuple constant arity mismatch for {ty}"));
            }
            let mut out = Vec::with_capacity(branch.len());
            for (ct, fty) in branch.iter().zip(tys.iter()) {
                out.push(render_ty_const(cx, *ct, fty)?);
            }
            Ok(format!("[{}]", out.join(",")))
        }
        ty::Adt(def, args) if !def.is_enum() => {
            let variant = def.non_enum_variant();
            if branch.len() != variant.fields.len() {
                return Err(format!("struct constant arity mismatch for {ty}"));
            }
            let mut out = Vec::with_capacity(branch.len());
            for (ct, f) in branch.iter().zip(variant.fields.iter()) {
                let fty = subst_unchecked(cx.tcx, args, cx.tcx.type_of(f.did).skip_binder());
                let fty = cx.ty(fty);
                out.push(render_ty_const(cx, *ct, fty)?);
            }
            Ok(format!("[{}]", out.join(",")))
        }
        ty::Ref(_, inner, _) if matches!(inner.kind(), ty::Str) => {
            let bytes: Vec<u8> = branch
                .iter()
                .map(|ct| ct.try_to_value()?.try_to_leaf().map(|l| l.to_u8()))
                .collect::<Option<_>>()
                .ok_or_else(|| format!("bad str constant of type {ty}"))?;
            Ok(js_string(&bytes))
        }
        _ => Err(format!("unsupported aggregate constant of type {ty}")),
    }
}

pub fn render_const_value<'tcx>(
    cx: &ConstCx<'tcx>,
    cv: mir::ConstValue,
    ty: Ty<'tcx>,
) -> Result<String, String> {
    use mir::ConstValue;
    // `core::alloc::Layout` (and friends) exist only at compile time.
    if let ty::Adt(def, _) = ty.kind() {
        if let Some(name) = cx.tcx.opt_item_name(def.did()) {
            if name.as_str() == "Layout" {
                return Ok("[0,0]".to_string());
            }
        }
    }
    match cv {
        ConstValue::ZeroSized => {
            // Zero-sized enums still carry a discriminant when matched on
            // (`Option::<!>::None` from `?` residuals): render the single
            // inhabited variant's shape instead of `undefined`.
            if let ty::Adt(def, _) = ty.kind() {
                if def.is_enum() {
                    let layout = cx
                        .tcx
                        .layout_of(mono_env().as_query_input(ty))
                        .map_err(|_| format!("bad layout for constant of type {ty}"))?;
                    if let rustc_abi::Variants::Single { index } = &layout.variants {
                        let d = crate::tyutil::discriminant_value(cx.tcx, ty, *index).ok_or_else(
                            || format!("bad enum discriminant for constant of type {ty}"),
                        )?;
                        return Ok(format!("{{t:{d},v:[]}}"));
                    }
                }
            }
            Ok("undefined".to_string())
        }
        ConstValue::Scalar(s) => scalar_to_js(cx, s, ty),
        ConstValue::Slice { alloc_id, meta } => {
            let bytes = read_alloc_bytes(cx, alloc_id, 0, meta)?;
            match ty.kind() {
                ty::Ref(_, inner, _) if matches!(inner.kind(), ty::Str) => Ok(js_string(&bytes)),
                ty::Str => Ok(js_string(&bytes)),
                _ => {
                    // Byte slice or u8 array materialized as Slice.
                    Ok(format!(
                        "[{}]",
                        bytes
                            .iter()
                            .map(|b| b.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    ))
                }
            }
        }
        ConstValue::Indirect { alloc_id, offset } => {
            // Structs/tuples/enums: walk the layout field by field.
            if scalar_size(ty).is_none() && !matches!(ty.kind(), ty::Str) {
                let alloc = match cx.tcx.global_alloc(alloc_id) {
                    mir::interpret::GlobalAlloc::Memory(a) => a,
                    _ => return Err("bad aggregate constant".to_string()),
                };
                return const_value_at(cx, alloc.inner(), ty, offset.bytes());
            }
            // A reference to sized data: read the scalar payload directly.
            let size = scalar_size(ty)
                .ok_or_else(|| format!("unsupported indirect constant of type {ty}"))?;
            let bytes = read_alloc_bytes(cx, alloc_id, offset.bytes(), size as u64)?;
            let mut buf = [0u8; 16];
            buf[..bytes.len()].copy_from_slice(&bytes);
            scalar_bits_to_js(u128::from_le_bytes(buf), ty)
        }
    }
}

/// Read a value of `ty` out of an allocation at `offset`, recursing into
/// aggregates (promoted `&Struct(..)` / `&(a, b)` constants reach here).
fn const_value_at<'tcx>(
    cx: &ConstCx<'tcx>,
    alloc: &mir::interpret::Allocation,
    ty: Ty<'tcx>,
    offset: u64,
) -> Result<String, String> {
    use mir::interpret::AllocRange;
    let ty = cx.ty(ty);
    if let Some(size) = scalar_size(ty) {
        let bytes = alloc.get_bytes_unchecked(AllocRange {
            start: Size::from_bytes(offset),
            size: Size::from_bytes(size as u64),
        });
        let mut buf = [0u8; 16];
        buf[..bytes.len()].copy_from_slice(bytes);
        return scalar_bits_to_js(u128::from_le_bytes(buf), ty);
    }
    if matches!(ty.kind(), ty::Str) {
        // Bare `str` has no size; only reachable through `&[u8; N]`-style
        // slices, which are handled by the Slice arm.
        return Err("bare str constant".to_string());
    }
    let layout = cx
        .tcx
        .layout_of(mono_env().as_query_input(ty))
        .map_err(|_| format!("bad layout for constant of type {ty}"))?;
    let read_at = |at: u64, size: usize| -> Result<u128, String> {
        let bytes = alloc.get_bytes_unchecked(AllocRange {
            start: Size::from_bytes(at),
            size: Size::from_bytes(size as u64),
        });
        let mut buf = [0u8; 16];
        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(u128::from_le_bytes(buf))
    };
    // Enums carry the compact tag first, then the active variant's fields.
    if let ty::Adt(def, args) = ty.kind() {
        if def.is_enum() {
            let variant_idx = match &layout.variants {
                rustc_abi::Variants::Empty => {
                    return Err("uninhabited enum constant".to_string());
                }
                rustc_abi::Variants::Single { index } => *index,
                rustc_abi::Variants::Multiple {
                    tag, tag_encoding, ..
                } => {
                    match tag_encoding {
                        rustc_abi::TagEncoding::Direct => {
                            let discr_size = tag.size(&cx.tcx).bytes() as usize;
                            let discr_val = read_at(offset, discr_size)?;
                            def.discriminants(cx.tcx)
                                .find(|(_, d)| d.val == discr_val)
                                .map(|(v, _)| v)
                                .ok_or("bad enum discriminant")?
                        }
                        rustc_abi::TagEncoding::Niche {
                            untagged_variant,
                            niche_variants,
                            niche_start,
                        } => {
                            // Reverse `tag_for_variant`: the niche value
                            // encodes the variant index, anything else is
                            // the untagged variant (e.g. `Some` payload).
                            let tag_size = tag.size(&cx.tcx).bytes() as usize;
                            let tag_val = read_at(offset, tag_size)?;
                            // (This toolchain's `RangeInclusive` exposes
                            // public `start`/`last` fields, no accessors.)
                            let start = niche_variants.start.as_u32() as u128;
                            let last = niche_variants.last.as_u32();
                            let i = tag_val.wrapping_sub(*niche_start).wrapping_add(start);
                            u32::try_from(i)
                                .ok()
                                .map(rustc_abi::VariantIdx::from_u32)
                                .filter(|v| {
                                    let n = v.as_u32();
                                    n >= niche_variants.start.as_u32() && n <= last
                                })
                                .unwrap_or(*untagged_variant)
                        }
                    }
                }
            };
            let variant = def.variant(variant_idx);
            let mut out = Vec::with_capacity(variant.fields.len());
            for (i, f) in variant.fields.iter().enumerate() {
                let fty = cx.ty(subst_unchecked(
                    cx.tcx,
                    args,
                    cx.tcx.type_of(f.did).skip_binder(),
                ));
                let off = offset + layout.fields.offset(i).bytes();
                out.push(const_value_at(cx, alloc, fty, off)?);
            }
            let discr = def.discriminant_for_variant(cx.tcx, variant_idx).val;
            return Ok(format!("{{t:{discr},v:[{}]}}", out.join(",")));
        }
    }
    let fields: Vec<Ty<'tcx>> = match ty.kind() {
        ty::Tuple(tys) => tys.to_vec(),
        ty::Adt(def, args) => def.variants()[rustc_abi::VariantIdx::from_usize(0)]
            .fields
            .iter()
            .map(|f| {
                cx.ty(subst_unchecked(
                    cx.tcx,
                    args,
                    cx.tcx.type_of(f.did).skip_binder(),
                ))
            })
            .collect(),
        _ => return Err(format!("unsupported aggregate constant of type {ty}")),
    };
    // Field offsets come from the layout, so `#[repr(Rust)]` reordering is
    // handled.
    let mut out = Vec::with_capacity(fields.len());
    for (i, fty) in fields.iter().enumerate() {
        let off = offset + layout.fields.offset(i).bytes();
        out.push(const_value_at(cx, alloc, *fty, off)?);
    }
    Ok(format!("[{}]", out.join(",")))
}

fn scalar_size(ty: Ty<'_>) -> Option<usize> {
    let n = match ty.kind() {
        ty::Bool => 1,
        ty::Char => 4,
        ty::Int(t) => match t {
            ty::IntTy::I8 => 1,
            ty::IntTy::I16 => 2,
            ty::IntTy::I32 => 4,
            ty::IntTy::I64 => 8,
            ty::IntTy::I128 => 16,
            ty::IntTy::Isize => 8,
        },
        ty::Uint(t) => match t {
            ty::UintTy::U8 => 1,
            ty::UintTy::U16 => 2,
            ty::UintTy::U32 => 4,
            ty::UintTy::U64 => 8,
            ty::UintTy::U128 => 16,
            ty::UintTy::Usize => 8,
        },
        ty::Float(t) => match t {
            ty::FloatTy::F16 => 2,
            ty::FloatTy::F32 => 4,
            ty::FloatTy::F64 => 8,
            ty::FloatTy::F128 => 16,
        },
        _ => return None,
    };
    Some(n)
}

fn read_alloc_bytes(
    cx: &ConstCx<'_>,
    alloc_id: mir::interpret::AllocId,
    offset: u64,
    len: u64,
) -> Result<Vec<u8>, String> {
    use mir::interpret::{AllocRange, GlobalAlloc};
    let alloc = match cx.tcx.global_alloc(alloc_id) {
        GlobalAlloc::Memory(a) => a,
        _ => return Err("unsupported static allocation".to_string()),
    };
    let alloc = alloc.inner();
    let range = AllocRange {
        start: Size::from_bytes(offset),
        size: Size::from_bytes(len),
    };
    Ok(alloc.get_bytes_unchecked(range).to_vec())
}

pub fn scalar_to_js<'tcx>(
    cx: &ConstCx<'tcx>,
    s: mir::interpret::Scalar,
    ty: Ty<'tcx>,
) -> Result<String, String> {
    use mir::interpret::Scalar;
    match s {
        Scalar::Int(si) => {
            let size = scalar_size(ty).ok_or_else(|| format!("scalar of unexpected type {ty}"))?;
            scalar_bits_to_js(si.to_bits(Size::from_bytes(size)), ty)
        }
        // Frozen reference to a scalar (e.g. promoted `&(2 + 3)`): read the
        // pointee out of the allocation. Also `&[u8; N]` (format templates).
        Scalar::Ptr(ptr, _) => {
            let (prov, offset) = ptr.into_raw_parts();
            match ty.kind() {
                ty::Ref(_, inner, _) => match scalar_size(*inner) {
                    Some(n) => {
                        let bytes =
                            read_alloc_bytes(cx, prov.alloc_id(), offset.bytes(), n as u64)?;
                        let mut buf = [0u8; 16];
                        buf[..bytes.len()].copy_from_slice(&bytes);
                        scalar_bits_to_js(u128::from_le_bytes(buf), *inner)
                    }
                    None => match inner.kind() {
                        ty::Array(elem, len) => {
                            let n = eval_usize(cx, *len).map_err(|e| e.to_string())?;
                            let esz = scalar_size(*elem)
                                .ok_or_else(|| "array of unsupported elements".to_string())?;
                            let bytes = read_alloc_bytes(
                                cx,
                                prov.alloc_id(),
                                offset.bytes(),
                                n.saturating_mul(esz as u64),
                            )?;
                            let mut out = Vec::with_capacity(n as usize);
                            for i in 0..n as usize {
                                let mut buf = [0u8; 16];
                                buf[..esz].copy_from_slice(&bytes[i * esz..(i + 1) * esz]);
                                out.push(scalar_bits_to_js(u128::from_le_bytes(buf), *elem)?);
                            }
                            Ok(format!("[{}]", out.join(",")))
                        }
                        // Promoted aggregate: `&Struct(..)` / `&(a, b)` / `&Variant`.
                        ty::Adt(..) | ty::Tuple(..) => {
                            let alloc = match cx.tcx.global_alloc(prov.alloc_id()) {
                                mir::interpret::GlobalAlloc::Memory(a) => a,
                                _ => return Err("bad aggregate constant".to_string()),
                            };
                            const_value_at(cx, alloc.inner(), *inner, offset.bytes())
                        }
                        _ => Err("pointer constants are not supported".to_string()),
                    },
                },
                _ => Err("pointer constants are not supported".to_string()),
            }
        }
    }
}

fn scalar_bits_to_js(bits: u128, ty: Ty<'_>) -> Result<String, String> {
    let out = match ty.kind() {
        ty::Bool => {
            if bits == 0 {
                "false".to_string()
            } else {
                "true".to_string()
            }
        }
        ty::Char => (bits as u32).to_string(),
        ty::Int(t) => match t {
            ty::IntTy::I8 | ty::IntTy::I16 | ty::IntTy::I32 | ty::IntTy::Isize => {
                let n = size_of_int(t, false);
                let v = sign_extend(bits, n);
                v.to_string()
            }
            ty::IntTy::I64 | ty::IntTy::I128 => {
                let n = size_of_int(t, false);
                format!("{}n", sign_extend(bits, n))
            }
        },
        ty::Uint(t) => match t {
            ty::UintTy::U8 | ty::UintTy::U16 | ty::UintTy::U32 | ty::UintTy::Usize => {
                (bits as u64).to_string()
            }
            ty::UintTy::U64 | ty::UintTy::U128 => format!("{bits}n"),
        },
        ty::Float(t) => match t {
            ty::FloatTy::F32 => render_f64(f32::from_bits(bits as u32) as f64),
            ty::FloatTy::F64 => render_f64(f64::from_bits(bits as u64)),
            _ => return Err(format!("{:?} constants are not supported", t)),
        },
        _ => return Err(format!("scalar constant of unexpected type {ty}")),
    };
    Ok(out)
}

fn size_of_int(t: &ty::IntTy, _unsigned: bool) -> u32 {
    match t {
        ty::IntTy::I8 => 8,
        ty::IntTy::I16 => 16,
        ty::IntTy::I32 => 32,
        ty::IntTy::I64 => 64,
        ty::IntTy::I128 => 128,
        ty::IntTy::Isize => 64,
    }
}

fn sign_extend(bits: u128, nbits: u32) -> i128 {
    if nbits >= 128 {
        bits as i128
    } else {
        ((bits as i128) << (128 - nbits)) >> (128 - nbits)
    }
}

fn render_f64(f: f64) -> String {
    if f.is_nan() {
        "NaN".to_string()
    } else if f.is_infinite() {
        if f > 0.0 {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        }
    } else if f == 0.0 {
        if f.is_sign_negative() {
            "-0".to_string()
        } else {
            "0".to_string()
        }
    } else {
        format!("{:?}", f)
    }
}

/// Render bytes as a JS string literal (assumed UTF-8; lossy fallback).
pub fn js_string(bytes: &[u8]) -> String {
    let s = std::str::from_utf8(bytes).unwrap_or("<invalid utf8>");
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{{{:x}}}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

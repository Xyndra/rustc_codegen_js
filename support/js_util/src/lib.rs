//! JS-interop primitives for `rustc_codegen_js`.
//!
//! Most code should use the higher-level `dom` crate instead of touching
//! this directly. The pieces here are:
//!
//! - [`JsValue`]: an opaque JS object handle (DOM node, canvas context,
//!   ...). Passed by reference, never serialized; `Clone` is identity,
//!   `==` is reference identity.
//! - `js0`–`js4`: explicit inline-JS escape hatches (see below). Each is
//!   peered at its call site from the `&str` template literal plus the
//!   trailing `$N` splices and is never emitted.
//!
//! ```rust,ignore
//! use js_util::{JsValue, js1};
//!
//! let el: JsValue = js1("document.getElementById($0)", "stage");
//! ```
//!
//! `$N` splices pass values by reference (strings splice as JS strings, so
//! write bare `$N`, never `"...$N..."`); `String` results arrive
//! cell-wrapped exactly like `#[js::import]` returns. In statement position
//! the return type cannot be inferred, so write
//! `let _: () = js1("$0.replaceChildren()", el);`.

#![no_std]
#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

/// Opaque JS object handle: a DOM node, canvas context, or any other live
/// JS value. Only manipulable through inline JS (or `$N`-template
/// `#[js::import]`s); see the `dom` crate for a typed API.
#[js::opaque]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct JsValue;

impl core::fmt::Debug for JsValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "JsValue")
    }
}

impl JsValue {
    /// The JS `null` value, for explicit null construction and comparison.
    pub fn null() -> JsValue {
        js0("null")
    }

    /// The JS `undefined` value.
    pub fn undefined() -> JsValue {
        js0("undefined")
    }
}

// Inline-JS stubs, one per splice arity. Safe (not `unsafe`) so call sites
// stay clean: the template is always a literal and execution cannot violate
// Rust memory safety (there is no linear memory; handles stay opaque).

/// Inline JS with no splices.
#[js::inline]
pub fn js0<T>(_: &str) -> T {
    panic!("js intrinsic (only meaningful under rustc_codegen_js)")
}

/// Inline JS with one splice (`$0`).
#[js::inline]
pub fn js1<T, A0>(_: &str, _: A0) -> T {
    panic!("js intrinsic (only meaningful under rustc_codegen_js)")
}

/// Inline JS with two splices (`$0`, `$1`).
#[js::inline]
pub fn js2<T, A0, A1>(_: &str, _: A0, _: A1) -> T {
    panic!("js intrinsic (only meaningful under rustc_codegen_js)")
}

/// Inline JS with three splices (`$0`–`$2`).
#[js::inline]
pub fn js3<T, A0, A1, A2>(_: &str, _: A0, _: A1, _: A2) -> T {
    panic!("js intrinsic (only meaningful under rustc_codegen_js)")
}

/// Inline JS with four splices (`$0`–`$3`).
#[js::inline]
pub fn js4<T, A0, A1, A2, A3>(_: &str, _: A0, _: A1, _: A2, _: A3) -> T {
    panic!("js intrinsic (only meaningful under rustc_codegen_js)")
}

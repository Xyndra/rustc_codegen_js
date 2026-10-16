//! Typed DOM API for `rustc_codegen_js`, with `web-sys` naming.
//!
//! Handles are transparent newtypes over [`js_util::JsValue`]: wrapping
//! and `.0` are free, `Clone`/`Copy`/`PartialEq` behave as identity, and
//! nothing is ever serialized. Every method binds the true JS operation
//! through a `$N`-template `#[js::import]` — no host shims, no runtime
//! glue to maintain.
//!
//! ```rust,ignore
//! use dom::{document, Element};
//!
//! let stage: Element = document()
//!     .get_element_by_id("stage")
//!     .expect("missing #stage");
//! stage.set_inner_html("<b>hi</b>");
//! ```
//!
//! Nullable DOM reads return [`Option`]; fallible (throwing) DOM methods
//! are currently only bound where they cannot throw — throwing variants
//! are an open item (no exception capture yet).

#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

use js_util::JsValue;

pub use js_util::JsValue as Handle;

/// The global `window`.
#[js::opaque]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Window(pub JsValue);

/// The global `window.document`.
#[js::opaque]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Document(pub JsValue);

/// A DOM element.
#[js::opaque]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Element(pub JsValue);

/// A `<textarea>` element.
#[js::opaque]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HtmlTextAreaElement(pub JsValue);

/// The global `window`.
#[js::import = "(() => window)"]
fn window_value() -> Window {
    panic!("dom intrinsic")
}

/// The global `window.document`.
#[js::import = "(() => window.document)"]
fn document_value() -> Document {
    panic!("dom intrinsic")
}

/// The global `window`.
pub fn window() -> Window {
    window_value()
}

/// The global `window.document`.
pub fn document() -> Document {
    document_value()
}

impl Window {
    /// `window.document`.
    #[js::import = "$0.document"]
    pub fn document(&self) -> Document {
        panic!("dom intrinsic")
    }
}

impl Document {
    /// `document.getElementById(id)`, or `None` when absent.
    #[js::import = "$0.getElementById($1)"]
    pub fn get_element_by_id(&self, _id: &str) -> Option<Element> {
        panic!("dom intrinsic")
    }

    /// `document.querySelector(selectors)`: `Err` when the selector is
    /// invalid (throws), `Ok(None)` when nothing matches.
    #[js::import = "$0.querySelector($1)"]
    pub fn query_selector(&self, _selectors: &str) -> Result<Option<Element>, JsValue> {
        panic!("dom intrinsic")
    }

    /// `document.createElement(tag)`.
    #[js::import = "$0.createElement($1)"]
    pub fn create_element(&self, _tag: &str) -> Element {
        panic!("dom intrinsic")
    }
}

impl Element {
    /// Upcast-free reinterpretation (transparent newtypes share the handle).
    pub fn from_handle(handle: JsValue) -> Self {
        Self(handle)
    }

    /// `el.innerHTML`.
    #[js::import = "$0.innerHTML"]
    pub fn inner_html(&self) -> String {
        panic!("dom intrinsic")
    }

    /// `el.innerHTML = html`.
    #[js::import = "$0.innerHTML = $1"]
    pub fn set_inner_html(&self, _html: &str) {
        panic!("dom intrinsic")
    }

    /// `el.textContent = text` (safe text, never parsed as HTML).
    #[js::import = "$0.textContent = $1"]
    pub fn set_text_content(&self, _text: &str) {
        panic!("dom intrinsic")
    }

    /// `el.replaceChildren()` (clear all children).
    #[js::import = "$0.replaceChildren()"]
    pub fn replace_children(&self) {
        panic!("dom intrinsic")
    }

    /// `el.insertAdjacentHTML(position, html)`.
    #[js::import = "$0.insertAdjacentHTML($1, $2)"]
    pub fn insert_adjacent_html(&self, _position: &str, _html: &str) {
        panic!("dom intrinsic")
    }

    /// `el.getAttribute(name)`, or `None` when absent.
    #[js::import = "$0.getAttribute($1)"]
    pub fn get_attribute(&self, _name: &str) -> Option<String> {
        panic!("dom intrinsic")
    }

    /// `el.setAttribute(name, value)`.
    #[js::import = "$0.setAttribute($1, $2)"]
    pub fn set_attribute(&self, _name: &str, _value: &str) {
        panic!("dom intrinsic")
    }

    /// `el.addEventListener(event, callback)`. Only capture-free `fn`
    /// callbacks cross (a capturing closure's env has no JS counterpart).
    #[js::import = "$0.addEventListener($1, $2)"]
    pub fn add_event_listener(&self, _event: &str, _callback: fn()) {
        panic!("dom intrinsic")
    }

    /// `el.remove()`.
    #[js::import = "$0.remove()"]
    pub fn remove(&self) {
        panic!("dom intrinsic")
    }

    /// `el.closest(selectors)`: `Err` when the selector is invalid
    /// (throws), `Ok(None)` when no ancestor matches.
    #[js::import = "$0.closest($1)"]
    pub fn closest(&self, _selectors: &str) -> Result<Option<Element>, JsValue> {
        panic!("dom intrinsic")
    }
}

impl HtmlTextAreaElement {
    /// Reinterpret an element handle (e.g. from `get_element_by_id`).
    pub fn from_element(el: Element) -> Self {
        Self(el.0)
    }

    /// Upcast back to a plain element.
    pub fn as_element(self) -> Element {
        Element(self.0)
    }

    /// `textarea.value`.
    #[js::import = "$0.value"]
    pub fn value(&self) -> String {
        panic!("dom intrinsic")
    }

    /// `textarea.value = value`.
    #[js::import = "$0.value = $1"]
    pub fn set_value(&self, _value: &str) {
        panic!("dom intrinsic")
    }
}

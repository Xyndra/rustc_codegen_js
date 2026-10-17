#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SubmitEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SubmitEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &SubmitEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &SubmitEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &SubmitEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &SubmitEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &SubmitEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &SubmitEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "HtmlElement")]
    #[doc = "Get the `submitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `SubmitEventInit`*"]
#[js::import = "None"]
pub fn get_submitter(this: &SubmitEventInit) -> Option<HtmlElement> { panic!("web-sys intrinsic") }
    #[cfg(feature = "HtmlElement")]
    #[doc = "Change the `submitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `SubmitEventInit`*"]
#[js::import = "None = $1"]
pub fn set_submitter(this: &SubmitEventInit, val: Option<&HtmlElement>) { panic!("web-sys intrinsic") }
impl SubmitEventInit {
    #[doc = "Construct a new `SubmitEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "HtmlElement")]
    #[deprecated = "Use `set_submitter()` instead."]
#[js::import = "None"]
pub fn submitter(&mut self, val: Option<&HtmlElement>) -> &mut Self { self.set_submitter(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for SubmitEventInit {
    fn default() -> Self {
        Self::new()
    }
}

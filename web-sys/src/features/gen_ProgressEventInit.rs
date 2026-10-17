#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProgressEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ProgressEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &ProgressEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &ProgressEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &ProgressEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &ProgressEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &ProgressEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &ProgressEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `lengthComputable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None"]
pub fn get_length_computable(this: &ProgressEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lengthComputable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None = $1"]
pub fn set_length_computable(this: &ProgressEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None"]
pub fn get_loaded(this: &ProgressEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None = $1"]
pub fn set_loaded(this: &ProgressEventInit, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `total` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None"]
pub fn get_total(this: &ProgressEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `total` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
#[js::import = "None = $1"]
pub fn set_total(this: &ProgressEventInit, val: f64) { panic!("web-sys intrinsic") }
impl ProgressEventInit {
    #[doc = "Construct a new `ProgressEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
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
    #[deprecated = "Use `set_length_computable()` instead."]
#[js::import = "None"]
pub fn length_computable(&mut self, val: bool) -> &mut Self { self.set_length_computable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_loaded()` instead."]
#[js::import = "None"]
pub fn loaded(&mut self, val: f64) -> &mut Self { self.set_loaded(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_total()` instead."]
#[js::import = "None"]
pub fn total(&mut self, val: f64) -> &mut Self { self.set_total(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ProgressEventInit {
    fn default() -> Self {
        Self::new()
    }
}

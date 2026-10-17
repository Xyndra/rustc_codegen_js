#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlobEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BlobEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &BlobEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &BlobEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &BlobEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &BlobEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &BlobEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &BlobEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Blob")]
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobEventInit`*"]
#[js::import = "None"]
pub fn get_data(this: &BlobEventInit) -> Option<Blob> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Blob")]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobEventInit`*"]
#[js::import = "None = $1"]
pub fn set_data(this: &BlobEventInit, val: Option<&Blob>) { panic!("web-sys intrinsic") }
impl BlobEventInit {
    #[doc = "Construct a new `BlobEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEventInit`*"]
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
    #[cfg(feature = "Blob")]
    #[deprecated = "Use `set_data()` instead."]
#[js::import = "None"]
pub fn data(&mut self, val: Option<&Blob>) -> &mut Self { self.set_data(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BlobEventInit {
    fn default() -> Self {
        Self::new()
    }
}

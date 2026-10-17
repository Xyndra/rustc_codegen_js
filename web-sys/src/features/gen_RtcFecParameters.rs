#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcFecParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcFecParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
#[js::import = "None"]
pub fn get_ssrc(this: &RtcFecParameters) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
#[js::import = "None = $1"]
pub fn set_ssrc(this: &RtcFecParameters, val: u32) { panic!("web-sys intrinsic") }
impl RtcFecParameters {
    #[doc = "Construct a new `RtcFecParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_ssrc()` instead."]
#[js::import = "None"]
pub fn ssrc(&mut self, val: u32) -> &mut Self { self.set_ssrc(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcFecParameters {
    fn default() -> Self {
        Self::new()
    }
}

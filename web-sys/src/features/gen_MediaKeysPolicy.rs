#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeysPolicy` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeysPolicy(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `minHdcpVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
#[js::import = "None"]
pub fn get_min_hdcp_version(this: &MediaKeysPolicy) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `minHdcpVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
#[js::import = "None = $1"]
pub fn set_min_hdcp_version(this: &MediaKeysPolicy, val: &str) { panic!("web-sys intrinsic") }
impl MediaKeysPolicy {
    #[doc = "Construct a new `MediaKeysPolicy`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_min_hdcp_version()` instead."]
#[js::import = "None"]
pub fn min_hdcp_version(&mut self, val: &str) -> &mut Self { self.set_min_hdcp_version(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaKeysPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtcpParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtcpParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `cname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
#[js::import = "None"]
pub fn get_cname(this: &RtcRtcpParameters) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
#[js::import = "None = $1"]
pub fn set_cname(this: &RtcRtcpParameters, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `reducedSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
#[js::import = "None"]
pub fn get_reduced_size(this: &RtcRtcpParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `reducedSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
#[js::import = "None = $1"]
pub fn set_reduced_size(this: &RtcRtcpParameters, val: bool) { panic!("web-sys intrinsic") }
impl RtcRtcpParameters {
    #[doc = "Construct a new `RtcRtcpParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_cname()` instead."]
#[js::import = "None"]
pub fn cname(&mut self, val: &str) -> &mut Self { self.set_cname(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_reduced_size()` instead."]
#[js::import = "None"]
pub fn reduced_size(&mut self, val: bool) -> &mut Self { self.set_reduced_size(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcRtcpParameters {
    fn default() -> Self {
        Self::new()
    }
}

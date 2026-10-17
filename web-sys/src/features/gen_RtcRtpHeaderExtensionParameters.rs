#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpHeaderExtensionParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpHeaderExtensionParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
#[js::import = "None"]
pub fn get_encrypted(this: &RtcRtpHeaderExtensionParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
#[js::import = "None = $1"]
pub fn set_encrypted(this: &RtcRtpHeaderExtensionParameters, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcRtpHeaderExtensionParameters) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcRtpHeaderExtensionParameters, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
#[js::import = "None"]
pub fn get_uri(this: &RtcRtpHeaderExtensionParameters) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
#[js::import = "None = $1"]
pub fn set_uri(this: &RtcRtpHeaderExtensionParameters, val: &str) { panic!("web-sys intrinsic") }
impl RtcRtpHeaderExtensionParameters {
    #[doc = "Construct a new `RtcRtpHeaderExtensionParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionParameters`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_encrypted()` instead."]
#[js::import = "None"]
pub fn encrypted(&mut self, val: bool) -> &mut Self { self.set_encrypted(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_id()` instead."]
#[js::import = "None"]
pub fn id(&mut self, val: u16) -> &mut Self { self.set_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_uri()` instead."]
#[js::import = "None"]
pub fn uri(&mut self, val: &str) -> &mut Self { self.set_uri(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcRtpHeaderExtensionParameters {
    fn default() -> Self {
        Self::new()
    }
}

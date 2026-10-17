#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpHeaderExtensionCapability` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpHeaderExtensionCapability(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
#[js::import = "None"]
pub fn get_uri(this: &RtcRtpHeaderExtensionCapability) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
#[js::import = "None = $1"]
pub fn set_uri(this: &RtcRtpHeaderExtensionCapability, val: &str) { panic!("web-sys intrinsic") }
impl RtcRtpHeaderExtensionCapability {
    #[doc = "Construct a new `RtcRtpHeaderExtensionCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
#[js::import = "None"]
pub fn new(uri: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_uri(uri);
        ret
    }
    #[deprecated = "Use `set_uri()` instead."]
#[js::import = "None"]
pub fn uri(&mut self, val: &str) -> &mut Self { self.set_uri(val) { panic!("web-sys intrinsic") }
        self
    }
}

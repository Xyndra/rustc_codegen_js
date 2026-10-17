#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcCertificate` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificate`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcCertificate(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `expires` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate/expires)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificate`*"]
impl RtcCertificate {
    #[js::import = "$0.expires"]
pub fn expires(&self) -> f64 { panic!("web-sys intrinsic") }
}

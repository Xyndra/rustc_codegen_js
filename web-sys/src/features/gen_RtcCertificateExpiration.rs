#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcCertificateExpiration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcCertificateExpiration(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
#[js::import = "None"]
pub fn get_expires(this: &RtcCertificateExpiration) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
#[js::import = "None = $1"]
pub fn set_expires(this: &RtcCertificateExpiration, val: f64) { panic!("web-sys intrinsic") }
impl RtcCertificateExpiration {
    #[doc = "Construct a new `RtcCertificateExpiration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCertificateExpiration`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_expires()` instead."]
#[js::import = "None"]
pub fn expires(&mut self, val: f64) -> &mut Self { self.set_expires(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcCertificateExpiration {
    fn default() -> Self {
        Self::new()
    }
}

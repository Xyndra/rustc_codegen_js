#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcOfferAnswerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferAnswerOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcOfferAnswerOptions(pub ::wasm_bindgen::JsValue);
impl RtcOfferAnswerOptions {
    #[doc = "Construct a new `RtcOfferAnswerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferAnswerOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
}
impl Default for RtcOfferAnswerOptions {
    fn default() -> Self {
        Self::new()
    }
}

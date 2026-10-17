#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcAnswerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcAnswerOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcAnswerOptions(pub ::wasm_bindgen::JsValue);
impl RtcAnswerOptions {
    #[doc = "Construct a new `RtcAnswerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcAnswerOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
}
impl Default for RtcAnswerOptions {
    fn default() -> Self {
        Self::new()
    }
}

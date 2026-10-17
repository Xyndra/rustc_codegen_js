#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyError`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeyError(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `systemCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyError/systemCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyError`*"]
impl MediaKeyError {
    #[js::import = "$0.systemCode"]
pub fn system_code(&self) -> u32 { panic!("web-sys intrinsic") }
}

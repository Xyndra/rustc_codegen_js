#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStreamError(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
impl MediaStreamError {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
impl MediaStreamError {
    #[js::import = "$0.message"]
pub fn message(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `constraint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/constraint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamError`*"]
impl MediaStreamError {
    #[js::import = "$0.constraint"]
pub fn constraint(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}

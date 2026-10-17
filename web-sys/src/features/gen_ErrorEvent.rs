#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ErrorEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
impl ErrorEvent {
    #[js::import = "$0.message"]
pub fn message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `filename` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/filename)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
impl ErrorEvent {
    #[js::import = "$0.filename"]
pub fn filename(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lineno` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/lineno)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
impl ErrorEvent {
    #[js::import = "$0.lineno"]
pub fn lineno(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `colno` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/colno)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
impl ErrorEvent {
    #[js::import = "$0.colno"]
pub fn colno(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
impl ErrorEvent {
    #[js::import = "$0.error"]
pub fn error(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ErrorEvent(..)` constructor, creating a new instance of `ErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/ErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`*"]
impl ErrorEvent {
    #[js::import = "new ErrorEvent($0)"]
pub fn new(type_: &str) -> Result<ErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ErrorEventInit")]
    #[doc = "The `new ErrorEvent(..)` constructor, creating a new instance of `ErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/ErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEvent`, `ErrorEventInit`*"]
impl ErrorEvent {
    #[js::import = "new ErrorEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &ErrorEventInit,) -> Result<ErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}

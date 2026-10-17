#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlContextEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebGlContextEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `statusMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/statusMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
impl WebGlContextEvent {
    #[js::import = "$0.statusMessage"]
pub fn status_message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new WebGlContextEvent(..)` constructor, creating a new instance of `WebGlContextEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
impl WebGlContextEvent {
    #[js::import = "new WebGLContextEvent($0)"]
pub fn new(type_: &str) -> Result<WebGlContextEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlContextEventInit")]
    #[doc = "The `new WebGlContextEvent(..)` constructor, creating a new instance of `WebGlContextEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEvent`, `WebGlContextEventInit`*"]
impl WebGlContextEvent {
    #[js::import = "new WebGLContextEvent($0, $1)"]
pub fn new_with_event_init(type_: &str, event_init: &WebGlContextEventInit,) -> Result<WebGlContextEvent, JsValue> { panic!("web-sys intrinsic") }
}

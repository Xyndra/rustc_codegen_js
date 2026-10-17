#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SFrameTransformErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct SFrameTransformErrorEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformErrorEventType")]
    #[doc = "Getter for the `errorType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/errorType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`, `SFrameTransformErrorEventType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransformErrorEvent {
    #[js::import = "$0.errorType"]
pub fn error_type(&self) -> SFrameTransformErrorEventType { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `keyID` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/keyID)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransformErrorEvent {
    #[js::import = "$0.keyID"]
pub fn key_id(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `frame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/frame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransformErrorEvent {
    #[js::import = "$0.frame"]
pub fn frame(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformErrorEventInit")]
    #[doc = "The `new SFrameTransformErrorEvent(..)` constructor, creating a new instance of `SFrameTransformErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/SFrameTransformErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`, `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransformErrorEvent {
    #[js::import = "new SFrameTransformErrorEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &SFrameTransformErrorEventInit,) -> Result<SFrameTransformErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyFrameRequestEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyFrameRequestEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct KeyFrameRequestEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `rid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent/rid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyFrameRequestEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl KeyFrameRequestEvent {
    #[js::import = "$0.rid"]
pub fn rid(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new KeyFrameRequestEvent(..)` constructor, creating a new instance of `KeyFrameRequestEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent/KeyFrameRequestEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyFrameRequestEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl KeyFrameRequestEvent {
    #[js::import = "new KeyFrameRequestEvent($0)"]
pub fn new(type_: &str) -> Result<KeyFrameRequestEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new KeyFrameRequestEvent(..)` constructor, creating a new instance of `KeyFrameRequestEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyFrameRequestEvent/KeyFrameRequestEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyFrameRequestEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl KeyFrameRequestEvent {
    #[js::import = "new KeyFrameRequestEvent($0, $1)"]
pub fn new_with_rid(type_: &str, rid: &str) -> Result<KeyFrameRequestEvent, JsValue> { panic!("web-sys intrinsic") }
}

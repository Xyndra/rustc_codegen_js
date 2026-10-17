#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrInputSourceEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct XrInputSourceEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrFrame")]
    #[doc = "Getter for the `frame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent/frame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrInputSourceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSourceEvent {
    #[js::import = "$0.frame"]
pub fn frame(&self) -> XrFrame { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrInputSource")]
    #[doc = "Getter for the `inputSource` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent/inputSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSource`, `XrInputSourceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSourceEvent {
    #[js::import = "$0.inputSource"]
pub fn input_source(&self) -> XrInputSource { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrInputSourceEventInit")]
    #[doc = "The `new XrInputSourceEvent(..)` constructor, creating a new instance of `XrInputSourceEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent/XRInputSourceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourceEvent`, `XrInputSourceEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSourceEvent {
    #[js::import = "new XRInputSourceEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &XrInputSourceEventInit,) -> Result<XrInputSourceEvent, JsValue> { panic!("web-sys intrinsic") }
}

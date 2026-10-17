#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrInputSourcesChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct XrInputSourcesChangeEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[doc = "Getter for the `session` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/session)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEvent`, `XrSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSourcesChangeEvent {
    #[js::import = "$0.session"]
pub fn session(&self) -> XrSession { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `added` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/added)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSourcesChangeEvent {
    #[js::import = "$0.added"]
pub fn added(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `removed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/removed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSourcesChangeEvent {
    #[js::import = "$0.removed"]
pub fn removed(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrInputSourcesChangeEventInit")]
    #[doc = "The `new XrInputSourcesChangeEvent(..)` constructor, creating a new instance of `XrInputSourcesChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/XRInputSourcesChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSourcesChangeEvent`, `XrInputSourcesChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSourcesChangeEvent {
    #[js::import = "new XRInputSourcesChangeEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &XrInputSourcesChangeEventInit,) -> Result<XrInputSourcesChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}

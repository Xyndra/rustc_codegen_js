#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrSessionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSessionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct XrSessionEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[doc = "Getter for the `session` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSessionEvent/session)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSession`, `XrSessionEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrSessionEvent {
    #[js::import = "$0.session"]
pub fn session(&self) -> XrSession { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSessionEventInit")]
    #[doc = "The `new XrSessionEvent(..)` constructor, creating a new instance of `XrSessionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSessionEvent/XRSessionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionEvent`, `XrSessionEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrSessionEvent {
    #[js::import = "new XRSessionEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &XrSessionEventInit,) -> Result<XrSessionEvent, JsValue> { panic!("web-sys intrinsic") }
}

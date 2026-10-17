#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidConnectionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidConnectionEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct HidConnectionEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDevice")]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDConnectionEvent/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidConnectionEvent`, `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl HidConnectionEvent {
    #[js::import = "$0.device"]
pub fn device(&self) -> HidDevice { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidConnectionEventInit")]
    #[doc = "The `new HidConnectionEvent(..)` constructor, creating a new instance of `HidConnectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDConnectionEvent/HIDConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidConnectionEvent`, `HidConnectionEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl HidConnectionEvent {
    #[js::import = "new HIDConnectionEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &HidConnectionEventInit,) -> Result<HidConnectionEvent, JsValue> { panic!("web-sys intrinsic") }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbConnectionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConnectionEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbConnectionEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDevice")]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConnectionEvent/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConnectionEvent`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbConnectionEvent {
    #[js::import = "$0.device"]
pub fn device(&self) -> UsbDevice { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbConnectionEventInit")]
    #[doc = "The `new UsbConnectionEvent(..)` constructor, creating a new instance of `UsbConnectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConnectionEvent/USBConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConnectionEvent`, `UsbConnectionEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbConnectionEvent {
    #[js::import = "new USBConnectionEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &UsbConnectionEventInit,) -> Result<UsbConnectionEvent, JsValue> { panic!("web-sys intrinsic") }
}

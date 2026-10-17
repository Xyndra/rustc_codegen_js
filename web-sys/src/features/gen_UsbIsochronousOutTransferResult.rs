#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbIsochronousOutTransferResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbIsochronousOutTransferResult(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `packets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferResult/packets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousOutTransferResult {
    #[js::import = "$0.packets"]
pub fn packets(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new UsbIsochronousOutTransferResult(..)` constructor, creating a new instance of `UsbIsochronousOutTransferResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferResult/USBIsochronousOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousOutTransferResult {
    #[js::import = "new USBIsochronousOutTransferResult($0)"]
pub fn new(packets: &::wasm_bindgen::JsValue,) -> Result<UsbIsochronousOutTransferResult, JsValue> { panic!("web-sys intrinsic") }
}

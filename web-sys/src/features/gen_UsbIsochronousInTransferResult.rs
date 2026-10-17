#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbIsochronousInTransferResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbIsochronousInTransferResult(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferResult/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousInTransferResult {
    #[js::import = "$0.data"]
pub fn data(&self) -> Option<::js_sys::DataView> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `packets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferResult/packets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousInTransferResult {
    #[js::import = "$0.packets"]
pub fn packets(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new UsbIsochronousInTransferResult(..)` constructor, creating a new instance of `UsbIsochronousInTransferResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferResult/USBIsochronousInTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousInTransferResult {
    #[js::import = "new USBIsochronousInTransferResult($0)"]
pub fn new(packets: &::wasm_bindgen::JsValue,) -> Result<UsbIsochronousInTransferResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new UsbIsochronousInTransferResult(..)` constructor, creating a new instance of `UsbIsochronousInTransferResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferResult/USBIsochronousInTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousInTransferResult {
    #[js::import = "new USBIsochronousInTransferResult($0, $1)"]
pub fn new_with_data(packets: &::wasm_bindgen::JsValue, data: Option<&::js_sys::DataView>,) -> Result<UsbIsochronousInTransferResult, JsValue> { panic!("web-sys intrinsic") }
}

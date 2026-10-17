#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbOutTransferResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbOutTransferResult(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `bytesWritten` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/bytesWritten)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbOutTransferResult {
    #[js::import = "$0.bytesWritten"]
pub fn bytes_written(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbOutTransferResult {
    #[js::import = "$0.status"]
pub fn status(&self) -> UsbTransferStatus { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[doc = "The `new UsbOutTransferResult(..)` constructor, creating a new instance of `UsbOutTransferResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/USBOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbOutTransferResult {
    #[js::import = "new USBOutTransferResult($0)"]
pub fn new(status: UsbTransferStatus) -> Result<UsbOutTransferResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[doc = "The `new UsbOutTransferResult(..)` constructor, creating a new instance of `UsbOutTransferResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBOutTransferResult/USBOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbOutTransferResult`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbOutTransferResult {
    #[js::import = "new USBOutTransferResult($0, $1)"]
pub fn new_with_bytes_written(status: UsbTransferStatus, bytes_written: u32,) -> Result<UsbOutTransferResult, JsValue> { panic!("web-sys intrinsic") }
}

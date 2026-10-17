#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbIsochronousOutTransferPacket` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferPacket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferPacket`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbIsochronousOutTransferPacket(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `bytesWritten` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferPacket/bytesWritten)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferPacket`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousOutTransferPacket {
    #[js::import = "$0.bytesWritten"]
pub fn bytes_written(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferPacket/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferPacket`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousOutTransferPacket {
    #[js::import = "$0.status"]
pub fn status(&self) -> UsbTransferStatus { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[doc = "The `new UsbIsochronousOutTransferPacket(..)` constructor, creating a new instance of `UsbIsochronousOutTransferPacket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferPacket/USBIsochronousOutTransferPacket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferPacket`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousOutTransferPacket {
    #[js::import = "new USBIsochronousOutTransferPacket($0)"]
pub fn new(status: UsbTransferStatus) -> Result<UsbIsochronousOutTransferPacket, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[doc = "The `new UsbIsochronousOutTransferPacket(..)` constructor, creating a new instance of `UsbIsochronousOutTransferPacket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferPacket/USBIsochronousOutTransferPacket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferPacket`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousOutTransferPacket {
    #[js::import = "new USBIsochronousOutTransferPacket($0, $1)"]
pub fn new_with_bytes_written(status: UsbTransferStatus, bytes_written: u32,) -> Result<UsbIsochronousOutTransferPacket, JsValue> { panic!("web-sys intrinsic") }
}

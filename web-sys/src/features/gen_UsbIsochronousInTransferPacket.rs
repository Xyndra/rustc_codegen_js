#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbIsochronousInTransferPacket` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferPacket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferPacket`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbIsochronousInTransferPacket(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferPacket/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferPacket`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousInTransferPacket {
    #[js::import = "$0.data"]
pub fn data(&self) -> Option<::js_sys::DataView> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferPacket/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferPacket`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousInTransferPacket {
    #[js::import = "$0.status"]
pub fn status(&self) -> UsbTransferStatus { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[doc = "The `new UsbIsochronousInTransferPacket(..)` constructor, creating a new instance of `UsbIsochronousInTransferPacket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferPacket/USBIsochronousInTransferPacket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferPacket`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousInTransferPacket {
    #[js::import = "new USBIsochronousInTransferPacket($0)"]
pub fn new(status: UsbTransferStatus) -> Result<UsbIsochronousInTransferPacket, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbTransferStatus")]
    #[doc = "The `new UsbIsochronousInTransferPacket(..)` constructor, creating a new instance of `UsbIsochronousInTransferPacket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousInTransferPacket/USBIsochronousInTransferPacket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousInTransferPacket`, `UsbTransferStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbIsochronousInTransferPacket {
    #[js::import = "new USBIsochronousInTransferPacket($0, $1)"]
pub fn new_with_data(status: UsbTransferStatus, data: Option<&::js_sys::DataView>,) -> Result<UsbIsochronousInTransferPacket, JsValue> { panic!("web-sys intrinsic") }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbEndpoint` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbEndpoint(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `endpointNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/endpointNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbEndpoint {
    #[js::import = "$0.endpointNumber"]
pub fn endpoint_number(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDirection")]
    #[doc = "Getter for the `direction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/direction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDirection`, `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbEndpoint {
    #[js::import = "$0.direction"]
pub fn direction(&self) -> UsbDirection { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbEndpointType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbEndpoint`, `UsbEndpointType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbEndpoint {
    #[js::import = "$0.type"]
pub fn type_(&self) -> UsbEndpointType { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `packetSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/packetSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbEndpoint {
    #[js::import = "$0.packetSize"]
pub fn packet_size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "UsbAlternateInterface", feature = "UsbDirection",))]
    #[doc = "The `new UsbEndpoint(..)` constructor, creating a new instance of `UsbEndpoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/USBEndpoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`, `UsbDirection`, `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbEndpoint {
    #[js::import = "new USBEndpoint($0, $1, $2)"]
pub fn new(alternate: &UsbAlternateInterface, endpoint_number: u8, direction: UsbDirection,) -> Result<UsbEndpoint, JsValue> { panic!("web-sys intrinsic") }
}

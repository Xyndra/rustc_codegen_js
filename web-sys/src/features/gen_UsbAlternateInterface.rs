#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbAlternateInterface` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbAlternateInterface(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `alternateSetting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/alternateSetting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbAlternateInterface {
    #[js::import = "$0.alternateSetting"]
pub fn alternate_setting(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `interfaceClass` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceClass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbAlternateInterface {
    #[js::import = "$0.interfaceClass"]
pub fn interface_class(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `interfaceSubclass` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceSubclass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbAlternateInterface {
    #[js::import = "$0.interfaceSubclass"]
pub fn interface_subclass(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `interfaceProtocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceProtocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbAlternateInterface {
    #[js::import = "$0.interfaceProtocol"]
pub fn interface_protocol(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `interfaceName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbAlternateInterface {
    #[js::import = "$0.interfaceName"]
pub fn interface_name(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `endpoints` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/endpoints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbAlternateInterface {
    #[js::import = "$0.endpoints"]
pub fn endpoints(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbInterface")]
    #[doc = "The `new UsbAlternateInterface(..)` constructor, creating a new instance of `UsbAlternateInterface`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/USBAlternateInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`, `UsbInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbAlternateInterface {
    #[js::import = "new USBAlternateInterface($0, $1)"]
pub fn new(device_interface: &UsbInterface, alternate_setting: u8,) -> Result<UsbAlternateInterface, JsValue> { panic!("web-sys intrinsic") }
}

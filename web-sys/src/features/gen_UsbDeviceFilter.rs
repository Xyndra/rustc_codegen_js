#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbDeviceFilter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbDeviceFilter(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `classCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_class_code(this: &UsbDeviceFilter) -> Option<u8> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `classCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_class_code(this: &UsbDeviceFilter, val: u8) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_product_id(this: &UsbDeviceFilter) -> Option<u16> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_product_id(this: &UsbDeviceFilter, val: u16) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `protocolCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_protocol_code(this: &UsbDeviceFilter) -> Option<u8> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `protocolCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_protocol_code(this: &UsbDeviceFilter, val: u8) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_serial_number(this: &UsbDeviceFilter) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_serial_number(this: &UsbDeviceFilter, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `subclassCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_subclass_code(this: &UsbDeviceFilter) -> Option<u8> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `subclassCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_subclass_code(this: &UsbDeviceFilter, val: u8) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_vendor_id(this: &UsbDeviceFilter) -> Option<u16> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_vendor_id(this: &UsbDeviceFilter, val: u16) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl UsbDeviceFilter {
    #[doc = "Construct a new `UsbDeviceFilter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_class_code()` instead."]
#[js::import = "None"]
pub fn class_code(&mut self, val: u8) -> &mut Self { self.set_class_code(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_product_id()` instead."]
#[js::import = "None"]
pub fn product_id(&mut self, val: u16) -> &mut Self { self.set_product_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_protocol_code()` instead."]
#[js::import = "None"]
pub fn protocol_code(&mut self, val: u8) -> &mut Self { self.set_protocol_code(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_serial_number()` instead."]
#[js::import = "None"]
pub fn serial_number(&mut self, val: &str) -> &mut Self { self.set_serial_number(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_subclass_code()` instead."]
#[js::import = "None"]
pub fn subclass_code(&mut self, val: u8) -> &mut Self { self.set_subclass_code(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_vendor_id()` instead."]
#[js::import = "None"]
pub fn vendor_id(&mut self, val: u16) -> &mut Self { self.set_vendor_id(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UsbDeviceFilter {
    fn default() -> Self {
        Self::new()
    }
}

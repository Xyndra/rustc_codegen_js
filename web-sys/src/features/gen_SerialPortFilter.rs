#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialPortFilter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct SerialPortFilter(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usbProductId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_usb_product_id(this: &SerialPortFilter) -> Option<u16> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usbProductId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_usb_product_id(this: &SerialPortFilter, val: u16) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usbVendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_usb_vendor_id(this: &SerialPortFilter) -> Option<u16> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usbVendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_usb_vendor_id(this: &SerialPortFilter, val: u16) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl SerialPortFilter {
    #[doc = "Construct a new `SerialPortFilter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_usb_product_id()` instead."]
#[js::import = "None"]
pub fn usb_product_id(&mut self, val: u16) -> &mut Self { self.set_usb_product_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_usb_vendor_id()` instead."]
#[js::import = "None"]
pub fn usb_vendor_id(&mut self, val: u16) -> &mut Self { self.set_usb_vendor_id(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SerialPortFilter {
    fn default() -> Self {
        Self::new()
    }
}

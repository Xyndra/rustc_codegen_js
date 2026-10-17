#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AllowedUsbDevice` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct AllowedUsbDevice(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_product_id(this: &AllowedUsbDevice) -> u8 { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_product_id(this: &AllowedUsbDevice, val: u8) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_serial_number(this: &AllowedUsbDevice) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_serial_number(this: &AllowedUsbDevice, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_vendor_id(this: &AllowedUsbDevice) -> u8 { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_vendor_id(this: &AllowedUsbDevice, val: u8) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl AllowedUsbDevice {
    #[doc = "Construct a new `AllowedUsbDevice`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new(product_id: u8, vendor_id: u8) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_product_id(product_id);
        ret.set_vendor_id(vendor_id);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_product_id()` instead."]
#[js::import = "None"]
pub fn product_id(&mut self, val: u8) -> &mut Self { self.set_product_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_serial_number()` instead."]
#[js::import = "None"]
pub fn serial_number(&mut self, val: &str) -> &mut Self { self.set_serial_number(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_vendor_id()` instead."]
#[js::import = "None"]
pub fn vendor_id(&mut self, val: u8) -> &mut Self { self.set_vendor_id(val) { panic!("web-sys intrinsic") }
        self
    }
}

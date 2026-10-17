#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbControlTransferParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbControlTransferParameters(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `index` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_index(this: &UsbControlTransferParameters) -> u16 { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `index` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_index(this: &UsbControlTransferParameters, val: u16) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbRecipient")]
    #[doc = "Get the `recipient` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbRecipient`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_recipient(this: &UsbControlTransferParameters) -> UsbRecipient { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbRecipient")]
    #[doc = "Change the `recipient` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbRecipient`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_recipient(this: &UsbControlTransferParameters, val: UsbRecipient) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_request(this: &UsbControlTransferParameters) -> u8 { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_request(this: &UsbControlTransferParameters, val: u8) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbRequestType")]
    #[doc = "Get the `requestType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbRequestType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_request_type(this: &UsbControlTransferParameters) -> UsbRequestType { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbRequestType")]
    #[doc = "Change the `requestType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbRequestType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_request_type(this: &UsbControlTransferParameters, val: UsbRequestType) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_value(this: &UsbControlTransferParameters) -> u16 { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_value(this: &UsbControlTransferParameters, val: u16) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl UsbControlTransferParameters {
    #[cfg(all(feature = "UsbRecipient", feature = "UsbRequestType",))]
    #[doc = "Construct a new `UsbControlTransferParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbRecipient`, `UsbRequestType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new(index: u16, recipient: UsbRecipient, request: u8, request_type: UsbRequestType, value: u16,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_index(index);
        ret.set_recipient(recipient);
        ret.set_request(request);
        ret.set_request_type(request_type);
        ret.set_value(value);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_index()` instead."]
#[js::import = "None"]
pub fn index(&mut self, val: u16) -> &mut Self { self.set_index(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbRecipient")]
    #[deprecated = "Use `set_recipient()` instead."]
#[js::import = "None"]
pub fn recipient(&mut self, val: UsbRecipient) -> &mut Self { self.set_recipient(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_request()` instead."]
#[js::import = "None"]
pub fn request(&mut self, val: u8) -> &mut Self { self.set_request(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbRequestType")]
    #[deprecated = "Use `set_request_type()` instead."]
#[js::import = "None"]
pub fn request_type(&mut self, val: UsbRequestType) -> &mut Self { self.set_request_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_value()` instead."]
#[js::import = "None"]
pub fn value(&mut self, val: u16) -> &mut Self { self.set_value(val) { panic!("web-sys intrinsic") }
        self
    }
}

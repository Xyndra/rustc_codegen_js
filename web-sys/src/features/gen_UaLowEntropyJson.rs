#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UaLowEntropyJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UaLowEntropyJson(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `brands` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_brands(this: &UaLowEntropyJson) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `brands` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_brands(this: &UaLowEntropyJson, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_mobile(this: &UaLowEntropyJson) -> Option<bool> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_mobile(this: &UaLowEntropyJson, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `platform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_platform(this: &UaLowEntropyJson) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `platform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_platform(this: &UaLowEntropyJson, val: &str) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl UaLowEntropyJson {
    #[doc = "Construct a new `UaLowEntropyJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_brands()` instead."]
#[js::import = "None"]
pub fn brands(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_brands(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_mobile()` instead."]
#[js::import = "None"]
pub fn mobile(&mut self, val: bool) -> &mut Self { self.set_mobile(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_platform()` instead."]
#[js::import = "None"]
pub fn platform(&mut self, val: &str) -> &mut Self { self.set_platform(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UaLowEntropyJson {
    fn default() -> Self {
        Self::new()
    }
}

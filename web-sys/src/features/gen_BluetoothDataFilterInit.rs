#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothDataFilterInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct BluetoothDataFilterInit(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataPrefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_data_prefix(this: &BluetoothDataFilterInit) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataPrefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_data_prefix(this: &BluetoothDataFilterInit, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_mask(this: &BluetoothDataFilterInit) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_mask(this: &BluetoothDataFilterInit, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl BluetoothDataFilterInit {
    #[doc = "Construct a new `BluetoothDataFilterInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDataFilterInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_data_prefix()` instead."]
#[js::import = "None"]
pub fn data_prefix(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_data_prefix(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_mask()` instead."]
#[js::import = "None"]
pub fn mask(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_mask(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for BluetoothDataFilterInit {
    fn default() -> Self {
        Self::new()
    }
}

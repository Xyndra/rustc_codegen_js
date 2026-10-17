#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothPermissionStorage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct BluetoothPermissionStorage(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_allowed_devices(this: &BluetoothPermissionStorage) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_allowed_devices(this: &BluetoothPermissionStorage, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl BluetoothPermissionStorage {
    #[doc = "Construct a new `BluetoothPermissionStorage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new(allowed_devices: &::wasm_bindgen::JsValue) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_allowed_devices(allowed_devices);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_allowed_devices()` instead."]
#[js::import = "None"]
pub fn allowed_devices(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_allowed_devices(val) { panic!("web-sys intrinsic") }
        self
    }
}

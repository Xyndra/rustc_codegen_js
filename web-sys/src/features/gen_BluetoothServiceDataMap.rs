#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothServiceDataMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct BluetoothServiceDataMap(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothServiceDataMap {
    #[js::import = "$0.size"]
pub fn size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothServiceDataMap {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothServiceDataMap {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothServiceDataMap {
    #[js::import = "$0.get($1)"]
pub fn get(&self, key: &str) -> Option<::js_sys::DataView> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothServiceDataMap {
    #[js::import = "$0.has($1)"]
pub fn has(&self, key: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothServiceDataMap {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothServiceDataMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothServiceDataMap {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothRemoteGattService` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct BluetoothRemoteGattService(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDevice`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.device"]
pub fn device(&self) -> BluetoothDevice { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `uuid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/uuid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.uuid"]
pub fn uuid(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `isPrimary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/isPrimary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.isPrimary"]
pub fn is_primary(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.oncharacteristicvaluechanged"]
pub fn oncharacteristicvaluechanged(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.oncharacteristicvaluechanged = $1"]
pub fn set_oncharacteristicvaluechanged(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onserviceadded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceadded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.onserviceadded"]
pub fn onserviceadded(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onserviceadded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceadded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.onserviceadded = $1"]
pub fn set_onserviceadded(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onservicechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onservicechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.onservicechanged"]
pub fn onservicechanged(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onservicechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onservicechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.onservicechanged = $1"]
pub fn set_onservicechanged(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onserviceremoved` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceremoved)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.onserviceremoved"]
pub fn onserviceremoved(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onserviceremoved` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/onserviceremoved)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.onserviceremoved = $1"]
pub fn set_onserviceremoved(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getCharacteristic()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristic)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getCharacteristic($1)"]
pub fn get_characteristic_with_str(&self, characteristic: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getCharacteristic()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristic)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getCharacteristic($1)"]
pub fn get_characteristic_with_u32(&self, characteristic: u32) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getCharacteristics()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristics)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getCharacteristics()"]
pub fn get_characteristics(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getCharacteristics()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristics)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getCharacteristics($1)"]
pub fn get_characteristics_with_str(&self, characteristic: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getCharacteristics()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getCharacteristics)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getCharacteristics($1)"]
pub fn get_characteristics_with_u32(&self, characteristic: u32) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getIncludedService()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getIncludedService($1)"]
pub fn get_included_service_with_str(&self, service: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getIncludedService()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getIncludedService($1)"]
pub fn get_included_service_with_u32(&self, service: u32) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getIncludedServices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedServices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getIncludedServices()"]
pub fn get_included_services(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getIncludedServices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedServices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getIncludedServices($1)"]
pub fn get_included_services_with_str(&self, service: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getIncludedServices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTService/getIncludedServices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattService {
    #[js::import = "$0.getIncludedServices($1)"]
pub fn get_included_services_with_u32(&self, service: u32) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}

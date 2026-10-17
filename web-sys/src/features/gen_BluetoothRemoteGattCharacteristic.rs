#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothRemoteGattCharacteristic` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct BluetoothRemoteGattCharacteristic(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattService")]
    #[doc = "Getter for the `service` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/service)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.service"]
pub fn service(&self) -> BluetoothRemoteGattService { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `uuid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/uuid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.uuid"]
pub fn uuid(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothCharacteristicProperties")]
    #[doc = "Getter for the `properties` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/properties)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothCharacteristicProperties`, `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.properties"]
pub fn properties(&self) -> BluetoothCharacteristicProperties { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.value"]
pub fn value(&self) -> Option<::js_sys::DataView> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.oncharacteristicvaluechanged"]
pub fn oncharacteristicvaluechanged(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.oncharacteristicvaluechanged = $1"]
pub fn set_oncharacteristicvaluechanged(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getDescriptor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.getDescriptor($1)"]
pub fn get_descriptor_with_str(&self, descriptor: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getDescriptor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.getDescriptor($1)"]
pub fn get_descriptor_with_u32(&self, descriptor: u32) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getDescriptors()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptors)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.getDescriptors()"]
pub fn get_descriptors(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getDescriptors()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptors)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.getDescriptors($1)"]
pub fn get_descriptors_with_str(&self, descriptor: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getDescriptors()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/getDescriptors)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.getDescriptors($1)"]
pub fn get_descriptors_with_u32(&self, descriptor: u32) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `readValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/readValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.readValue()"]
pub fn read_value(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `startNotifications()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/startNotifications)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.startNotifications()"]
pub fn start_notifications(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `stopNotifications()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/stopNotifications)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.stopNotifications()"]
pub fn stop_notifications(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `writeValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.writeValue($1)"]
pub fn write_value_with_buffer_source(&self, value: &::js_sys::Object) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `writeValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.writeValue($1)"]
pub fn write_value_with_u8_slice(&self, value: &mut [u8]) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `writeValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.writeValue($1)"]
pub fn write_value_with_u8_array(&self, value: &::js_sys::Uint8Array) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `writeValueWithResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.writeValueWithResponse($1)"]
pub fn write_value_with_response_with_buffer_source(&self, value: &::js_sys::Object) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `writeValueWithResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.writeValueWithResponse($1)"]
pub fn write_value_with_response_with_u8_slice(&self, value: &mut [u8]) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `writeValueWithResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.writeValueWithResponse($1)"]
pub fn write_value_with_response_with_u8_array(&self, value: &::js_sys::Uint8Array) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `writeValueWithoutResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithoutResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.writeValueWithoutResponse($1)"]
pub fn write_value_without_response_with_buffer_source(&self, value: &::js_sys::Object) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `writeValueWithoutResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithoutResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.writeValueWithoutResponse($1)"]
pub fn write_value_without_response_with_u8_slice(&self, value: &mut [u8]) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `writeValueWithoutResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTCharacteristic/writeValueWithoutResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattCharacteristic`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothRemoteGattCharacteristic {
    #[js::import = "$0.writeValueWithoutResponse($1)"]
pub fn write_value_without_response_with_u8_array(&self, value: &::js_sys::Uint8Array) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}

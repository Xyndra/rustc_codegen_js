#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothAdvertisingEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct BluetoothAdvertisingEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`, `BluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothAdvertisingEvent {
    #[js::import = "$0.device"]
pub fn device(&self) -> BluetoothDevice { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `uuids` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/uuids)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothAdvertisingEvent {
    #[js::import = "$0.uuids"]
pub fn uuids(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothAdvertisingEvent {
    #[js::import = "$0.name"]
pub fn name(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `appearance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/appearance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothAdvertisingEvent {
    #[js::import = "$0.appearance"]
pub fn appearance(&self) -> Option<u16> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `txPower` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/txPower)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothAdvertisingEvent {
    #[js::import = "$0.txPower"]
pub fn tx_power(&self) -> Option<i8> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `rssi` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/rssi)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothAdvertisingEvent {
    #[js::import = "$0.rssi"]
pub fn rssi(&self) -> Option<i8> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothManufacturerDataMap")]
    #[doc = "Getter for the `manufacturerData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/manufacturerData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`, `BluetoothManufacturerDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothAdvertisingEvent {
    #[js::import = "$0.manufacturerData"]
pub fn manufacturer_data(&self) -> BluetoothManufacturerDataMap { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothServiceDataMap")]
    #[doc = "Getter for the `serviceData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/serviceData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`, `BluetoothServiceDataMap`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothAdvertisingEvent {
    #[js::import = "$0.serviceData"]
pub fn service_data(&self) -> BluetoothServiceDataMap { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothAdvertisingEventInit")]
    #[doc = "The `new BluetoothAdvertisingEvent(..)` constructor, creating a new instance of `BluetoothAdvertisingEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothAdvertisingEvent/BluetoothAdvertisingEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothAdvertisingEvent`, `BluetoothAdvertisingEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl BluetoothAdvertisingEvent {
    #[js::import = "new BluetoothAdvertisingEvent($0, $1)"]
pub fn new(type_: &str, init: &BluetoothAdvertisingEventInit,) -> Result<BluetoothAdvertisingEvent, JsValue> { panic!("web-sys intrinsic") }
}

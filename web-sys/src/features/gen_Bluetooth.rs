#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Bluetooth` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct Bluetooth(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onavailabilitychanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onavailabilitychanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onavailabilitychanged"]
pub fn onavailabilitychanged(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onavailabilitychanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onavailabilitychanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onavailabilitychanged = $1"]
pub fn set_onavailabilitychanged(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[doc = "Getter for the `referringDevice` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/referringDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`, `BluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.referringDevice"]
pub fn referring_device(&self) -> Option<BluetoothDevice> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onadvertisementreceived` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onadvertisementreceived)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onadvertisementreceived"]
pub fn onadvertisementreceived(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onadvertisementreceived` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onadvertisementreceived)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onadvertisementreceived = $1"]
pub fn set_onadvertisementreceived(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `ongattserverdisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/ongattserverdisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.ongattserverdisconnected"]
pub fn ongattserverdisconnected(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `ongattserverdisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/ongattserverdisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.ongattserverdisconnected = $1"]
pub fn set_ongattserverdisconnected(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.oncharacteristicvaluechanged"]
pub fn oncharacteristicvaluechanged(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `oncharacteristicvaluechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/oncharacteristicvaluechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.oncharacteristicvaluechanged = $1"]
pub fn set_oncharacteristicvaluechanged(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onserviceadded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceadded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onserviceadded"]
pub fn onserviceadded(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onserviceadded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceadded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onserviceadded = $1"]
pub fn set_onserviceadded(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onservicechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onservicechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onservicechanged"]
pub fn onservicechanged(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onservicechanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onservicechanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onservicechanged = $1"]
pub fn set_onservicechanged(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onserviceremoved` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceremoved)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onserviceremoved"]
pub fn onserviceremoved(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onserviceremoved` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceremoved)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.onserviceremoved = $1"]
pub fn set_onserviceremoved(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getAvailability()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/getAvailability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.getAvailability()"]
pub fn get_availability(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getDevices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/getDevices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.getDevices()"]
pub fn get_devices(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RequestDeviceOptions")]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`, `RequestDeviceOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Bluetooth {
    #[js::import = "$0.requestDevice($1)"]
pub fn request_device(&self, options: &RequestDeviceOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}

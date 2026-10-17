#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbDevice` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct UsbDevice(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `usbVersionMajor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/usbVersionMajor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.usbVersionMajor"]
pub fn usb_version_major(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `usbVersionMinor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/usbVersionMinor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.usbVersionMinor"]
pub fn usb_version_minor(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `usbVersionSubminor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/usbVersionSubminor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.usbVersionSubminor"]
pub fn usb_version_subminor(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `deviceClass` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceClass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.deviceClass"]
pub fn device_class(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `deviceSubclass` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceSubclass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.deviceSubclass"]
pub fn device_subclass(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `deviceProtocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceProtocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.deviceProtocol"]
pub fn device_protocol(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/vendorId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.vendorId"]
pub fn vendor_id(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `productId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/productId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.productId"]
pub fn product_id(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `deviceVersionMajor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceVersionMajor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.deviceVersionMajor"]
pub fn device_version_major(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `deviceVersionMinor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceVersionMinor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.deviceVersionMinor"]
pub fn device_version_minor(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `deviceVersionSubminor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceVersionSubminor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.deviceVersionSubminor"]
pub fn device_version_subminor(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `manufacturerName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/manufacturerName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.manufacturerName"]
pub fn manufacturer_name(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `productName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/productName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.productName"]
pub fn product_name(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/serialNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.serialNumber"]
pub fn serial_number(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbConfiguration")]
    #[doc = "Getter for the `configuration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/configuration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConfiguration`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.configuration"]
pub fn configuration(&self) -> Option<UsbConfiguration> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `configurations` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/configurations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.configurations"]
pub fn configurations(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `opened` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/opened)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.opened"]
pub fn opened(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `claimInterface()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/claimInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.claimInterface($1)"]
pub fn claim_interface(&self, interface_number: u8) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDirection")]
    #[doc = "The `clearHalt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/clearHalt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbDirection`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.clearHalt($1, $2)"]
pub fn clear_halt(&self, direction: UsbDirection, endpoint_number: u8) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.close()"]
pub fn close(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbControlTransferParameters")]
    #[doc = "The `controlTransferIn()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferIn)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.controlTransferIn($1, $2)"]
pub fn control_transfer_in(&self, setup: &UsbControlTransferParameters, length: u16) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbControlTransferParameters")]
    #[doc = "The `controlTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.controlTransferOut($1)"]
pub fn control_transfer_out(&self, setup: &UsbControlTransferParameters) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbControlTransferParameters")]
    #[doc = "The `controlTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.controlTransferOut($1, $2)"]
pub fn control_transfer_out_with_buffer_source(&self, setup: &UsbControlTransferParameters, data: &::js_sys::Object) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbControlTransferParameters")]
    #[doc = "The `controlTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.controlTransferOut($1, $2)"]
pub fn control_transfer_out_with_u8_slice(&self, setup: &UsbControlTransferParameters, data: &mut [u8]) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbControlTransferParameters")]
    #[doc = "The `controlTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.controlTransferOut($1, $2)"]
pub fn control_transfer_out_with_u8_array(&self, setup: &UsbControlTransferParameters, data: &::js_sys::Uint8Array) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `forget()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/forget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.forget()"]
pub fn forget(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `isochronousTransferIn()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferIn)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.isochronousTransferIn($1, $2)"]
pub fn isochronous_transfer_in(&self, endpoint_number: u8, packet_lengths: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `isochronousTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.isochronousTransferOut($1, $2, $3)"]
pub fn isochronous_transfer_out_with_buffer_source(&self, endpoint_number: u8, data: &::js_sys::Object, packet_lengths: &::wasm_bindgen::JsValue) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `isochronousTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.isochronousTransferOut($1, $2, $3)"]
pub fn isochronous_transfer_out_with_u8_slice(&self, endpoint_number: u8, data: &mut [u8], packet_lengths: &::wasm_bindgen::JsValue) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `isochronousTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.isochronousTransferOut($1, $2, $3)"]
pub fn isochronous_transfer_out_with_u8_array(&self, endpoint_number: u8, data: &::js_sys::Uint8Array, packet_lengths: &::wasm_bindgen::JsValue) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.open()"]
pub fn open(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `releaseInterface()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/releaseInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.releaseInterface($1)"]
pub fn release_interface(&self, interface_number: u8) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.reset()"]
pub fn reset(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `selectAlternateInterface()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/selectAlternateInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.selectAlternateInterface($1, $2)"]
pub fn select_alternate_interface(&self, interface_number: u8, alternate_setting: u8) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `selectConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/selectConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.selectConfiguration($1)"]
pub fn select_configuration(&self, configuration_value: u8) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `transferIn()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferIn)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.transferIn($1, $2)"]
pub fn transfer_in(&self, endpoint_number: u8, length: u32) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `transferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.transferOut($1, $2)"]
pub fn transfer_out_with_buffer_source(&self, endpoint_number: u8, data: &::js_sys::Object) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `transferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.transferOut($1, $2)"]
pub fn transfer_out_with_u8_slice(&self, endpoint_number: u8, data: &mut [u8]) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `transferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl UsbDevice {
    #[js::import = "$0.transferOut($1, $2)"]
pub fn transfer_out_with_u8_array(&self, endpoint_number: u8, data: &::js_sys::Uint8Array) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}

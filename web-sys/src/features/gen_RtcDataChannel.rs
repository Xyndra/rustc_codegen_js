#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcDataChannel(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.id"]
pub fn id(&self) -> Option<u16> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `reliable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/reliable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.reliable"]
pub fn reliable(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxPacketLifeTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.maxPacketLifeTime"]
pub fn max_packet_life_time(&self) -> Option<u16> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxRetransmits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.maxRetransmits"]
pub fn max_retransmits(&self) -> Option<u16> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcDataChannelState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelState`*"]
impl RtcDataChannel {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> RtcDataChannelState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bufferedAmount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.bufferedAmount"]
pub fn buffered_amount(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bufferedAmountLowThreshold` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.bufferedAmountLowThreshold"]
pub fn buffered_amount_low_threshold(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `bufferedAmountLowThreshold` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.bufferedAmountLowThreshold = $1"]
pub fn set_buffered_amount_low_threshold(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onopen"]
pub fn onopen(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onopen = $1"]
pub fn set_onopen(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onclose"]
pub fn onclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onclose = $1"]
pub fn set_onclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbufferedamountlow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onbufferedamountlow"]
pub fn onbufferedamountlow(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbufferedamountlow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.onbufferedamountlow = $1"]
pub fn set_onbufferedamountlow(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcDataChannelType")]
    #[doc = "Getter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*"]
impl RtcDataChannel {
    #[js::import = "$0.binaryType"]
pub fn binary_type(&self) -> RtcDataChannelType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcDataChannelType")]
    #[doc = "Setter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*"]
impl RtcDataChannel {
    #[js::import = "$0.binaryType = $1"]
pub fn set_binary_type(&self, value: RtcDataChannelType) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.send($1)"]
pub fn send_with_str(&self, data: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.send($1)"]
pub fn send_with_blob(&self, data: &Blob) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.send($1)"]
pub fn send_with_array_buffer(&self, data: &::js_sys::ArrayBuffer) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.send($1)"]
pub fn send_with_array_buffer_view(&self, data: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.send($1)"]
pub fn send_with_u8_array(&self, data: &[u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`*"]
impl RtcDataChannel {
    #[js::import = "$0.send($1)"]
pub fn send_with_js_u8_array(&self, data: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}

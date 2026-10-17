#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcdtmfSender` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcdtmfSender(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `ontonechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
impl RtcdtmfSender {
    #[js::import = "$0.ontonechange"]
pub fn ontonechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontonechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
impl RtcdtmfSender {
    #[js::import = "$0.ontonechange = $1"]
pub fn set_ontonechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `toneBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/toneBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
impl RtcdtmfSender {
    #[js::import = "$0.toneBuffer"]
pub fn tone_buffer(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertDTMF()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
impl RtcdtmfSender {
    #[js::import = "$0.insertDTMF($1)"]
pub fn insert_dtmf(&self, tones: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertDTMF()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
impl RtcdtmfSender {
    #[js::import = "$0.insertDTMF($1, $2)"]
pub fn insert_dtmf_with_duration(&self, tones: &str, duration: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertDTMF()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
impl RtcdtmfSender {
    #[js::import = "$0.insertDTMF($1, $2, $3)"]
pub fn insert_dtmf_with_duration_and_inter_tone_gap(&self, tones: &str, duration: u32, inter_tone_gap: u32) { panic!("web-sys intrinsic") }
}

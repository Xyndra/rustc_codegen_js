#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcdtmfToneChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcdtmfToneChangeEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `tone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/tone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
impl RtcdtmfToneChangeEvent {
    #[js::import = "$0.tone"]
pub fn tone(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new RtcdtmfToneChangeEvent(..)` constructor, creating a new instance of `RtcdtmfToneChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`*"]
impl RtcdtmfToneChangeEvent {
    #[js::import = "new RTCDTMFToneChangeEvent($0)"]
pub fn new(type_: &str) -> Result<RtcdtmfToneChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcdtmfToneChangeEventInit")]
    #[doc = "The `new RtcdtmfToneChangeEvent(..)` constructor, creating a new instance of `RtcdtmfToneChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFToneChangeEvent/RTCDTMFToneChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEvent`, `RtcdtmfToneChangeEventInit`*"]
impl RtcdtmfToneChangeEvent {
    #[js::import = "new RTCDTMFToneChangeEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &RtcdtmfToneChangeEventInit,) -> Result<RtcdtmfToneChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}

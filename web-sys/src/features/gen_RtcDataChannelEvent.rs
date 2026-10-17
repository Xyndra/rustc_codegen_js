#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannelEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcDataChannelEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Getter for the `channel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/channel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEvent`*"]
impl RtcDataChannelEvent {
    #[js::import = "$0.channel"]
pub fn channel(&self) -> RtcDataChannel { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcDataChannelEventInit")]
    #[doc = "The `new RtcDataChannelEvent(..)` constructor, creating a new instance of `RtcDataChannelEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/RTCDataChannelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEvent`, `RtcDataChannelEventInit`*"]
impl RtcDataChannelEvent {
    #[js::import = "new RTCDataChannelEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &RtcDataChannelEventInit,) -> Result<RtcDataChannelEvent, JsValue> { panic!("web-sys intrinsic") }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcSessionDescription` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcSessionDescription(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescription`*"]
impl RtcSessionDescription {
    #[js::import = "$0.type"]
pub fn type_(&self) -> RtcSdpType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescription`*"]
impl RtcSessionDescription {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: RtcSdpType) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sdp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
impl RtcSessionDescription {
    #[js::import = "$0.sdp"]
pub fn sdp(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `sdp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
impl RtcSessionDescription {
    #[js::import = "$0.sdp = $1"]
pub fn set_sdp(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new RtcSessionDescription(..)` constructor, creating a new instance of `RtcSessionDescription`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/RTCSessionDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
impl RtcSessionDescription {
    #[js::import = "new RTCSessionDescription()"]
pub fn new() -> Result<RtcSessionDescription, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[doc = "The `new RtcSessionDescription(..)` constructor, creating a new instance of `RtcSessionDescription`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/RTCSessionDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`, `RtcSessionDescriptionInit`*"]
impl RtcSessionDescription {
    #[js::import = "new RTCSessionDescription($0)"]
pub fn new_with_description_init_dict(description_init_dict: &RtcSessionDescriptionInit,) -> Result<RtcSessionDescription, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescription`*"]
impl RtcSessionDescription {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}

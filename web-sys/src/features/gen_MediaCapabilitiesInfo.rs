#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaCapabilitiesInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaCapabilitiesInfo(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `supported` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/supported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
impl MediaCapabilitiesInfo {
    #[js::import = "$0.supported"]
pub fn supported(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `smooth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/smooth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
impl MediaCapabilitiesInfo {
    #[js::import = "$0.smooth"]
pub fn smooth(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `powerEfficient` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/powerEfficient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
impl MediaCapabilitiesInfo {
    #[js::import = "$0.powerEfficient"]
pub fn power_efficient(&self) -> bool { panic!("web-sys intrinsic") }
}

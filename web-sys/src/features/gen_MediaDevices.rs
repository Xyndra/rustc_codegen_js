#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaDevices` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaDevices(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `ondevicechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
impl MediaDevices {
    #[js::import = "$0.ondevicechange"]
pub fn ondevicechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondevicechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
impl MediaDevices {
    #[js::import = "$0.ondevicechange = $1"]
pub fn set_ondevicechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `enumerateDevices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/enumerateDevices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
impl MediaDevices {
    #[js::import = "$0.enumerateDevices()"]
pub fn enumerate_devices(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getDisplayMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
impl MediaDevices {
    #[js::import = "$0.getDisplayMedia()"]
pub fn get_display_media(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DisplayMediaStreamConstraints")]
    #[doc = "The `getDisplayMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaDevices`*"]
impl MediaDevices {
    #[js::import = "$0.getDisplayMedia($1)"]
pub fn get_display_media_with_constraints(&self, constraints: &DisplayMediaStreamConstraints) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaTrackSupportedConstraints")]
    #[doc = "The `getSupportedConstraints()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getSupportedConstraints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`, `MediaTrackSupportedConstraints`*"]
impl MediaDevices {
    #[js::import = "$0.getSupportedConstraints()"]
pub fn get_supported_constraints(&self) -> MediaTrackSupportedConstraints { panic!("web-sys intrinsic") }
}
    #[doc = "The `getUserMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
impl MediaDevices {
    #[js::import = "$0.getUserMedia()"]
pub fn get_user_media(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamConstraints")]
    #[doc = "The `getUserMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`, `MediaStreamConstraints`*"]
impl MediaDevices {
    #[js::import = "$0.getUserMedia($1)"]
pub fn get_user_media_with_constraints(&self, constraints: &MediaStreamConstraints) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}

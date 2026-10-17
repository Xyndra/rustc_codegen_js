#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaDeviceInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaDeviceInfo(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/deviceId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
impl MediaDeviceInfo {
    #[js::import = "$0.deviceId"]
pub fn device_id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaDeviceKind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`, `MediaDeviceKind`*"]
impl MediaDeviceInfo {
    #[js::import = "$0.kind"]
pub fn kind(&self) -> MediaDeviceKind { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
impl MediaDeviceInfo {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/groupId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
impl MediaDeviceInfo {
    #[js::import = "$0.groupId"]
pub fn group_id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDeviceInfo`*"]
impl MediaDeviceInfo {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}

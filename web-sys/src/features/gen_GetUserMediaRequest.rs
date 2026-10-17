#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GetUserMediaRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct GetUserMediaRequest(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `windowID` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/windowID)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
impl GetUserMediaRequest {
    #[js::import = "$0.windowID"]
pub fn window_id(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `innerWindowID` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/innerWindowID)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
impl GetUserMediaRequest {
    #[js::import = "$0.innerWindowID"]
pub fn inner_window_id(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `callID` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/callID)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
impl GetUserMediaRequest {
    #[js::import = "$0.callID"]
pub fn call_id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rawID` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/rawID)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
impl GetUserMediaRequest {
    #[js::import = "$0.rawID"]
pub fn raw_id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/mediaSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
impl GetUserMediaRequest {
    #[js::import = "$0.mediaSource"]
pub fn media_source(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isSecure` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/isSecure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
impl GetUserMediaRequest {
    #[js::import = "$0.isSecure"]
pub fn is_secure(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isHandlingUserInput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/isHandlingUserInput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUserMediaRequest`*"]
impl GetUserMediaRequest {
    #[js::import = "$0.isHandlingUserInput"]
pub fn is_handling_user_input(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamConstraints")]
    #[doc = "The `getConstraints()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GetUserMediaRequest/getConstraints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetUserMediaRequest`, `MediaStreamConstraints`*"]
impl GetUserMediaRequest {
    #[js::import = "$0.getConstraints()"]
pub fn get_constraints(&self) -> MediaStreamConstraints { panic!("web-sys intrinsic") }
}

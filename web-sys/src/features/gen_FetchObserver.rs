#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FetchObserver(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "FetchState")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`, `FetchState`*"]
impl FetchObserver {
    #[js::import = "$0.state"]
pub fn state(&self) -> FetchState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
impl FetchObserver {
    #[js::import = "$0.onstatechange"]
pub fn onstatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
impl FetchObserver {
    #[js::import = "$0.onstatechange = $1"]
pub fn set_onstatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onrequestprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
impl FetchObserver {
    #[js::import = "$0.onrequestprogress"]
pub fn onrequestprogress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onrequestprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
impl FetchObserver {
    #[js::import = "$0.onrequestprogress = $1"]
pub fn set_onrequestprogress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onresponseprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
impl FetchObserver {
    #[js::import = "$0.onresponseprogress"]
pub fn onresponseprogress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onresponseprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
impl FetchObserver {
    #[js::import = "$0.onresponseprogress = $1"]
pub fn set_onresponseprogress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}

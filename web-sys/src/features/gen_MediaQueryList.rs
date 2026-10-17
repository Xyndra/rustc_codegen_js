#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaQueryList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaQueryList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
impl MediaQueryList {
    #[js::import = "$0.media"]
pub fn media(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `matches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/matches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
impl MediaQueryList {
    #[js::import = "$0.matches"]
pub fn matches(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
impl MediaQueryList {
    #[js::import = "$0.onchange"]
pub fn onchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
impl MediaQueryList {
    #[js::import = "$0.onchange = $1"]
pub fn set_onchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `addListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/addListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
impl MediaQueryList {
    #[js::import = "$0.addListener($1)"]
pub fn add_listener_with_opt_callback(&self, listener: Option<&::js_sys::Function>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventListener")]
    #[doc = "The `addListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/addListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `MediaQueryList`*"]
impl MediaQueryList {
    #[js::import = "$0.addListener($1)"]
pub fn add_listener_with_opt_event_listener(&self, listener: Option<&EventListener>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/removeListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
impl MediaQueryList {
    #[js::import = "$0.removeListener($1)"]
pub fn remove_listener_with_opt_callback(&self, listener: Option<&::js_sys::Function>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventListener")]
    #[doc = "The `removeListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/removeListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `MediaQueryList`*"]
impl MediaQueryList {
    #[js::import = "$0.removeListener($1)"]
pub fn remove_listener_with_opt_event_listener(&self, listener: Option<&EventListener>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}

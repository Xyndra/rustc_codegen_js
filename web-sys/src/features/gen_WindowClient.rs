#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WindowClient` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WindowClient`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WindowClient(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "VisibilityState")]
    #[doc = "Getter for the `visibilityState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/visibilityState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisibilityState`, `WindowClient`*"]
impl WindowClient {
    #[js::import = "$0.visibilityState"]
pub fn visibility_state(&self) -> VisibilityState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `focused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WindowClient`*"]
impl WindowClient {
    #[js::import = "$0.focused"]
pub fn focused(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `focus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WindowClient`*"]
impl WindowClient {
    #[js::import = "$0.focus()"]
pub fn focus(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `navigate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/navigate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WindowClient`*"]
impl WindowClient {
    #[js::import = "$0.navigate($1)"]
pub fn navigate(&self, url: &str) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}

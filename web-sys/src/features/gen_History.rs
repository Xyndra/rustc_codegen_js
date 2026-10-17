#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `History` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
#[js::opaque]
#[derive(Copy)]
pub struct History(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.length"]
pub fn length(&self) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScrollRestoration")]
    #[doc = "Getter for the `scrollRestoration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*"]
impl History {
    #[js::import = "$0.scrollRestoration"]
pub fn scroll_restoration(&self) -> Result<ScrollRestoration, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScrollRestoration")]
    #[doc = "Setter for the `scrollRestoration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*"]
impl History {
    #[js::import = "$0.scrollRestoration = $1"]
pub fn set_scroll_restoration(&self, value: ScrollRestoration) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.state"]
pub fn state(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `back()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/back)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.back()"]
pub fn back(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `forward()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/forward)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.forward()"]
pub fn forward(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `go()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.go()"]
pub fn go(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `go()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.go($1)"]
pub fn go_with_delta(&self, delta: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `pushState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.pushState($1, $2)"]
pub fn push_state(&self, data: &::wasm_bindgen::JsValue, title: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `pushState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.pushState($1, $2, $3)"]
pub fn push_state_with_url(&self, data: &::wasm_bindgen::JsValue, title: &str, url: Option<&str>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.replaceState($1, $2)"]
pub fn replace_state(&self, data: &::wasm_bindgen::JsValue, title: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`*"]
impl History {
    #[js::import = "$0.replaceState($1, $2, $3)"]
pub fn replace_state_with_url(&self, data: &::wasm_bindgen::JsValue, title: &str, url: Option<&str>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}

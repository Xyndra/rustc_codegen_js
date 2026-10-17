#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Geolocation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Geolocation(pub ::wasm_bindgen::JsValue);
    #[doc = "The `clearWatch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/clearWatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
impl Geolocation {
    #[js::import = "$0.clearWatch($1)"]
pub fn clear_watch(&self, watch_id: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
impl Geolocation {
    #[js::import = "$0.getCurrentPosition($1)"]
pub fn get_current_position(&self, success_callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
impl Geolocation {
    #[js::import = "$0.getCurrentPosition($1, $2)"]
pub fn get_current_position_with_error_callback(&self, success_callback: &::js_sys::Function, error_callback: Option<&::js_sys::Function>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PositionOptions")]
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `PositionOptions`*"]
impl Geolocation {
    #[js::import = "$0.getCurrentPosition($1, $2, $3)"]
pub fn get_current_position_with_error_callback_and_options(&self, success_callback: &::js_sys::Function, error_callback: Option<&::js_sys::Function>, options: &PositionOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `watchPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
impl Geolocation {
    #[js::import = "$0.watchPosition($1)"]
pub fn watch_position(&self, success_callback: &::js_sys::Function) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `watchPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
impl Geolocation {
    #[js::import = "$0.watchPosition($1, $2)"]
pub fn watch_position_with_error_callback(&self, success_callback: &::js_sys::Function, error_callback: Option<&::js_sys::Function>) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PositionOptions")]
    #[doc = "The `watchPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `PositionOptions`*"]
impl Geolocation {
    #[js::import = "$0.watchPosition($1, $2, $3)"]
pub fn watch_position_with_error_callback_and_options(&self, success_callback: &::js_sys::Function, error_callback: Option<&::js_sys::Function>, options: &PositionOptions) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}

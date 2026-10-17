#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScreenOrientation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ScreenOrientation(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "OrientationType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OrientationType`, `ScreenOrientation`*"]
impl ScreenOrientation {
    #[js::import = "$0.type"]
pub fn type_(&self) -> Result<OrientationType, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `angle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/angle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
impl ScreenOrientation {
    #[js::import = "$0.angle"]
pub fn angle(&self) -> Result<u16, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
impl ScreenOrientation {
    #[js::import = "$0.onchange"]
pub fn onchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
impl ScreenOrientation {
    #[js::import = "$0.onchange = $1"]
pub fn set_onchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OrientationLockType")]
    #[doc = "The `lock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/lock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OrientationLockType`, `ScreenOrientation`*"]
impl ScreenOrientation {
    #[js::import = "$0.lock($1)"]
pub fn lock(&self, orientation: OrientationLockType) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `unlock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/unlock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
impl ScreenOrientation {
    #[js::import = "$0.unlock()"]
pub fn unlock(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}

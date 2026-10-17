#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Screen` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Screen(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `availWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.availWidth"]
pub fn avail_width(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `availHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.availHeight"]
pub fn avail_height(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.width"]
pub fn width(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.height"]
pub fn height(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `colorDepth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorDepth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.colorDepth"]
pub fn color_depth(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pixelDepth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/pixelDepth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.pixelDepth"]
pub fn pixel_depth(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `top` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/top)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.top"]
pub fn top(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `left` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/left)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.left"]
pub fn left(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `availTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.availTop"]
pub fn avail_top(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `availLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.availLeft"]
pub fn avail_left(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScreenOrientation")]
    #[doc = "Getter for the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/orientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`, `ScreenOrientation`*"]
impl Screen {
    #[js::import = "$0.orientation"]
pub fn orientation(&self) -> ScreenOrientation { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScreenColorGamut")]
    #[doc = "Getter for the `colorGamut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorGamut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`, `ScreenColorGamut`*"]
impl Screen {
    #[js::import = "$0.colorGamut"]
pub fn color_gamut(&self) -> ScreenColorGamut { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScreenLuminance")]
    #[doc = "Getter for the `luminance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/luminance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`, `ScreenLuminance`*"]
impl Screen {
    #[js::import = "$0.luminance"]
pub fn luminance(&self) -> Option<ScreenLuminance> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.onchange"]
pub fn onchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`*"]
impl Screen {
    #[js::import = "$0.onchange = $1"]
pub fn set_onchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}

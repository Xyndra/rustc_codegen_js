#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollBoxObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ScrollBoxObject(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
impl ScrollBoxObject {
    #[js::import = "$0.positionX"]
pub fn position_x(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
impl ScrollBoxObject {
    #[js::import = "$0.positionY"]
pub fn position_y(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scrolledWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
impl ScrollBoxObject {
    #[js::import = "$0.scrolledWidth"]
pub fn scrolled_width(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scrolledHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
impl ScrollBoxObject {
    #[js::import = "$0.scrolledHeight"]
pub fn scrolled_height(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `ensureElementIsVisible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/ensureElementIsVisible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*"]
impl ScrollBoxObject {
    #[js::import = "$0.ensureElementIsVisible($1)"]
pub fn ensure_element_is_visible(&self, child: &Element) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
impl ScrollBoxObject {
    #[js::import = "$0.scrollBy($1, $2)"]
pub fn scroll_by(&self, dx: i32, dy: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollByIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollByIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
impl ScrollBoxObject {
    #[js::import = "$0.scrollByIndex($1)"]
pub fn scroll_by_index(&self, dindexes: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
impl ScrollBoxObject {
    #[js::import = "$0.scrollTo($1, $2)"]
pub fn scroll_to(&self, x: i32, y: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `scrollToElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollToElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*"]
impl ScrollBoxObject {
    #[js::import = "$0.scrollToElement($1)"]
pub fn scroll_to_element(&self, child: &Element) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}

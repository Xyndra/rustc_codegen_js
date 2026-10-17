#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgLengthList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLengthList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgLengthList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLengthList`*"]
impl SvgLengthList {
    #[js::import = "$0.numberOfItems"]
pub fn number_of_items(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLength")]
    #[doc = "The `appendItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/appendItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
impl SvgLengthList {
    #[js::import = "$0.appendItem($1)"]
pub fn append_item(&self, new_item: &SvgLength) -> Result<SvgLength, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLengthList`*"]
impl SvgLengthList {
    #[js::import = "$0.clear()"]
pub fn clear(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLength")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
impl SvgLengthList {
    #[js::import = "$0.getItem($1)"]
pub fn get_item(&self, index: u32) -> Result<SvgLength, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLength")]
    #[doc = "The `initialize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/initialize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
impl SvgLengthList {
    #[js::import = "$0.initialize($1)"]
pub fn initialize(&self, new_item: &SvgLength) -> Result<SvgLength, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLength")]
    #[doc = "The `insertItemBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/insertItemBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
impl SvgLengthList {
    #[js::import = "$0.insertItemBefore($1, $2)"]
pub fn insert_item_before(&self, new_item: &SvgLength, index: u32) -> Result<SvgLength, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLength")]
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
impl SvgLengthList {
    #[js::import = "$0.removeItem($1)"]
pub fn remove_item(&self, index: u32) -> Result<SvgLength, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLength")]
    #[doc = "The `replaceItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/replaceItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
impl SvgLengthList {
    #[js::import = "$0.replaceItem($1, $2)"]
pub fn replace_item(&self, new_item: &SvgLength, index: u32) -> Result<SvgLength, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLength")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
impl SvgLengthList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Result<SvgLength, JsValue> { panic!("web-sys intrinsic") }
}

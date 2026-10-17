#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgNumberList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumberList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgNumberList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumberList`*"]
impl SvgNumberList {
    #[js::import = "$0.numberOfItems"]
pub fn number_of_items(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgNumber")]
    #[doc = "The `appendItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/appendItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
impl SvgNumberList {
    #[js::import = "$0.appendItem($1)"]
pub fn append_item(&self, new_item: &SvgNumber) -> Result<SvgNumber, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumberList`*"]
impl SvgNumberList {
    #[js::import = "$0.clear()"]
pub fn clear(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgNumber")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
impl SvgNumberList {
    #[js::import = "$0.getItem($1)"]
pub fn get_item(&self, index: u32) -> Result<SvgNumber, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgNumber")]
    #[doc = "The `initialize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/initialize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
impl SvgNumberList {
    #[js::import = "$0.initialize($1)"]
pub fn initialize(&self, new_item: &SvgNumber) -> Result<SvgNumber, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgNumber")]
    #[doc = "The `insertItemBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/insertItemBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
impl SvgNumberList {
    #[js::import = "$0.insertItemBefore($1, $2)"]
pub fn insert_item_before(&self, new_item: &SvgNumber, index: u32) -> Result<SvgNumber, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgNumber")]
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
impl SvgNumberList {
    #[js::import = "$0.removeItem($1)"]
pub fn remove_item(&self, index: u32) -> Result<SvgNumber, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgNumber")]
    #[doc = "The `replaceItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/replaceItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
impl SvgNumberList {
    #[js::import = "$0.replaceItem($1, $2)"]
pub fn replace_item(&self, new_item: &SvgNumber, index: u32) -> Result<SvgNumber, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgNumber")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
impl SvgNumberList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Result<SvgNumber, JsValue> { panic!("web-sys intrinsic") }
}

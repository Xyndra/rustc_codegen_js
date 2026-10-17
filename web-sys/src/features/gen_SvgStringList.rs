#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgStringList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgStringList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.numberOfItems"]
pub fn number_of_items(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/appendItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.appendItem($1)"]
pub fn append_item(&self, new_item: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.clear()"]
pub fn clear(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.getItem($1)"]
pub fn get_item(&self, index: u32) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initialize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/initialize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.initialize($1)"]
pub fn initialize(&self, new_item: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertItemBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/insertItemBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.insertItemBefore($1, $2)"]
pub fn insert_item_before(&self, new_item: &str, index: u32) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.removeItem($1)"]
pub fn remove_item(&self, index: u32) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/replaceItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.replaceItem($1, $2)"]
pub fn replace_item(&self, new_item: &str, index: u32) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
impl SvgStringList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}

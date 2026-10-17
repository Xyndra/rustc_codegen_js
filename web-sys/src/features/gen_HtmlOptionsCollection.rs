#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOptionsCollection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlOptionsCollection(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.length = $1"]
pub fn set_length(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.selectedIndex"]
pub fn selected_index(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.selectedIndex = $1"]
pub fn set_selected_index(&self, value: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptionElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.add($1)"]
pub fn add_with_html_option_element(&self, element: &HtmlOptionElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptGroupElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.add($1)"]
pub fn add_with_html_opt_group_element(&self, element: &HtmlOptGroupElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlElement", feature = "HtmlOptionElement",))]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptionElement`, `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_html_option_element_and_opt_html_element(&self, element: &HtmlOptionElement, before: Option<&HtmlElement>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlElement", feature = "HtmlOptGroupElement",))]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptGroupElement`, `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_html_opt_group_element_and_opt_html_element(&self, element: &HtmlOptGroupElement, before: Option<&HtmlElement>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptionElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_html_option_element_and_opt_i32(&self, element: &HtmlOptionElement, before: Option<i32>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptGroupElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_html_opt_group_element_and_opt_i32(&self, element: &HtmlOptGroupElement, before: Option<i32>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
impl HtmlOptionsCollection {
    #[js::import = "$0.remove($1)"]
pub fn remove(&self, index: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptionElement")]
        catch,
        method,
        structural,
        js_class = "HTMLOptionsCollection",
        indexing_setter
    )]
    #[doc = "Indexing setter. As in the literal Javascript `this[key] = value`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*"]
#[js::import = "None"]
pub fn set(this: &HtmlOptionsCollection, index: u32, option: Option<&HtmlOptionElement>,) -> Result<(), JsValue> { panic!("web-sys intrinsic") }

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgPathSegList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegList`*"]
impl SvgPathSegList {
    #[js::import = "$0.numberOfItems"]
pub fn number_of_items(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPathSeg")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSeg`, `SvgPathSegList`*"]
impl SvgPathSegList {
    #[js::import = "$0.getItem($1)"]
pub fn get_item(&self, index: u32) -> Result<SvgPathSeg, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPathSeg")]
        catch,
        method,
        structural,
        js_class = "SVGPathSegList",
        indexing_getter
    )]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSeg`, `SvgPathSegList`*"]
#[js::import = "None"]
pub fn get(this: &SvgPathSegList, index: u32) -> Result<SvgPathSeg, JsValue> { panic!("web-sys intrinsic") }

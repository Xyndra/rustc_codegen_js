#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTransformList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransformList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgTransformList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.numberOfItems"]
pub fn number_of_items(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransform")]
    #[doc = "The `appendItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/appendItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.appendItem($1)"]
pub fn append_item(&self, new_item: &SvgTransform) -> Result<SvgTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.clear()"]
pub fn clear(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransform")]
    #[doc = "The `consolidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/consolidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.consolidate()"]
pub fn consolidate(&self) -> Result<Option<SvgTransform>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
    #[doc = "The `createSVGTransformFromMatrix()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/createSVGTransformFromMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`, `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.createSVGTransformFromMatrix($1)"]
pub fn create_svg_transform_from_matrix(&self, matrix: &SvgMatrix) -> SvgTransform { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransform")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.getItem($1)"]
pub fn get_item(&self, index: u32) -> Result<SvgTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransform")]
    #[doc = "The `initialize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/initialize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.initialize($1)"]
pub fn initialize(&self, new_item: &SvgTransform) -> Result<SvgTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransform")]
    #[doc = "The `insertItemBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/insertItemBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.insertItemBefore($1, $2)"]
pub fn insert_item_before(&self, new_item: &SvgTransform, index: u32) -> Result<SvgTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransform")]
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.removeItem($1)"]
pub fn remove_item(&self, index: u32) -> Result<SvgTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransform")]
    #[doc = "The `replaceItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/replaceItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
impl SvgTransformList {
    #[js::import = "$0.replaceItem($1, $2)"]
pub fn replace_item(&self, new_item: &SvgTransform, index: u32) -> Result<SvgTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransform")]
        catch,
        method,
        structural,
        js_class = "SVGTransformList",
        indexing_getter
    )]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
#[js::import = "None"]
pub fn get(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue> { panic!("web-sys intrinsic") }

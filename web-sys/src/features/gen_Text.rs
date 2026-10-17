#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Text` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Text(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `wholeText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/wholeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
impl Text {
    #[js::import = "$0.wholeText"]
pub fn whole_text(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlSlotElement")]
    #[doc = "Getter for the `assignedSlot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/assignedSlot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`, `Text`*"]
impl Text {
    #[js::import = "$0.assignedSlot"]
pub fn assigned_slot(&self) -> Option<HtmlSlotElement> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Text(..)` constructor, creating a new instance of `Text`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/Text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
impl Text {
    #[js::import = "new Text()"]
pub fn new() -> Result<Text, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Text(..)` constructor, creating a new instance of `Text`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/Text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
impl Text {
    #[js::import = "new Text($0)"]
pub fn new_with_data(data: &str) -> Result<Text, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `splitText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/splitText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
impl Text {
    #[js::import = "$0.splitText($1)"]
pub fn split_text(&self, offset: u32) -> Result<Text, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Text`*"]
impl Text {
    #[js::import = "$0.convertPointFromNode($1, $2)"]
pub fn convert_point_from_node_with_text(&self, point: &DomPointInit, from: &Text) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Element",))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
impl Text {
    #[js::import = "$0.convertPointFromNode($1, $2)"]
pub fn convert_point_from_node_with_element(&self, point: &DomPointInit, from: &Element) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "DomPoint", feature = "DomPointInit",))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
impl Text {
    #[js::import = "$0.convertPointFromNode($1, $2)"]
pub fn convert_point_from_node_with_document(&self, point: &DomPointInit, from: &Document) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Text`*"]
impl Text {
    #[js::import = "$0.convertPointFromNode($1, $2, $3)"]
pub fn convert_point_from_node_with_text_and_options(&self, point: &DomPointInit, from: &Text, options: &ConvertCoordinateOptions) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
impl Text {
    #[js::import = "$0.convertPointFromNode($1, $2, $3)"]
pub fn convert_point_from_node_with_element_and_options(&self, point: &DomPointInit, from: &Element, options: &ConvertCoordinateOptions) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
impl Text {
    #[js::import = "$0.convertPointFromNode($1, $2, $3)"]
pub fn convert_point_from_node_with_document_and_options(&self, point: &DomPointInit, from: &Document, options: &ConvertCoordinateOptions) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomQuad")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `Text`*"]
impl Text {
    #[js::import = "$0.convertQuadFromNode($1, $2)"]
pub fn convert_quad_from_node_with_text(&self, quad: &DomQuad, from: &Text) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "Element",))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `Element`, `Text`*"]
impl Text {
    #[js::import = "$0.convertQuadFromNode($1, $2)"]
pub fn convert_quad_from_node_with_element(&self, quad: &DomQuad, from: &Element) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "DomQuad",))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Text`*"]
impl Text {
    #[js::import = "$0.convertQuadFromNode($1, $2)"]
pub fn convert_quad_from_node_with_document(&self, quad: &DomQuad, from: &Document) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ConvertCoordinateOptions", feature = "DomQuad",))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Text`*"]
impl Text {
    #[js::import = "$0.convertQuadFromNode($1, $2, $3)"]
pub fn convert_quad_from_node_with_text_and_options(&self, quad: &DomQuad, from: &Text, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Element",
    ))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`, `Text`*"]
impl Text {
    #[js::import = "$0.convertQuadFromNode($1, $2, $3)"]
pub fn convert_quad_from_node_with_element_and_options(&self, quad: &DomQuad, from: &Element, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
    ))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Text`*"]
impl Text {
    #[js::import = "$0.convertQuadFromNode($1, $2, $3)"]
pub fn convert_quad_from_node_with_document_and_options(&self, quad: &DomQuad, from: &Document, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Text`*"]
impl Text {
    #[js::import = "$0.convertRectFromNode($1, $2)"]
pub fn convert_rect_from_node_with_text(&self, rect: &DomRectReadOnly, from: &Text) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Element",))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
impl Text {
    #[js::import = "$0.convertRectFromNode($1, $2)"]
pub fn convert_rect_from_node_with_element(&self, rect: &DomRectReadOnly, from: &Element) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
impl Text {
    #[js::import = "$0.convertRectFromNode($1, $2)"]
pub fn convert_rect_from_node_with_document(&self, rect: &DomRectReadOnly, from: &Document) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
impl Text {
    #[js::import = "$0.convertRectFromNode($1, $2, $3)"]
pub fn convert_rect_from_node_with_text_and_options(&self, rect: &DomRectReadOnly, from: &Text, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
impl Text {
    #[js::import = "$0.convertRectFromNode($1, $2, $3)"]
pub fn convert_rect_from_node_with_element_and_options(&self, rect: &DomRectReadOnly, from: &Element, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
impl Text {
    #[js::import = "$0.convertRectFromNode($1, $2, $3)"]
pub fn convert_rect_from_node_with_document_and_options(&self, rect: &DomRectReadOnly, from: &Document, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Text`*"]
impl Text {
    #[js::import = "$0.getBoxQuads()"]
pub fn get_box_quads(&self) -> Result<::js_sys::Array, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BoxQuadOptions")]
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Text/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `Text`*"]
impl Text {
    #[js::import = "$0.getBoxQuads($1)"]
pub fn get_box_quads_with_options(&self, options: &BoxQuadOptions) -> Result<::js_sys::Array, JsValue> { panic!("web-sys intrinsic") }
}

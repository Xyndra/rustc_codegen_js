#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTextContentElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgTextContentElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `textLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/textLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.textLength"]
pub fn text_length(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `lengthAdjust` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/lengthAdjust)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.lengthAdjust"]
pub fn length_adjust(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPoint")]
    #[doc = "The `getCharNumAtPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getCharNumAtPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.getCharNumAtPosition($1)"]
pub fn get_char_num_at_position(&self, point: &SvgPoint) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getComputedTextLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getComputedTextLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.getComputedTextLength()"]
pub fn get_computed_text_length(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPoint")]
    #[doc = "The `getEndPositionOfChar()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getEndPositionOfChar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.getEndPositionOfChar($1)"]
pub fn get_end_position_of_char(&self, charnum: u32) -> Result<SvgPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgRect")]
    #[doc = "The `getExtentOfChar()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getExtentOfChar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgRect`, `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.getExtentOfChar($1)"]
pub fn get_extent_of_char(&self, charnum: u32) -> Result<SvgRect, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getNumberOfChars()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getNumberOfChars)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.getNumberOfChars()"]
pub fn get_number_of_chars(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRotationOfChar()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getRotationOfChar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.getRotationOfChar($1)"]
pub fn get_rotation_of_char(&self, charnum: u32) -> Result<f32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPoint")]
    #[doc = "The `getStartPositionOfChar()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getStartPositionOfChar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.getStartPositionOfChar($1)"]
pub fn get_start_position_of_char(&self, charnum: u32) -> Result<SvgPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getSubStringLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getSubStringLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.getSubStringLength($1, $2)"]
pub fn get_sub_string_length(&self, charnum: u32, nchars: u32) -> Result<f32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `selectSubString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/selectSubString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
impl SvgTextContentElement {
    #[js::import = "$0.selectSubString($1, $2)"]
pub fn select_sub_string(&self, charnum: u32, nchars: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl SvgTextContentElement {
    #[doc = "The `SVGTextContentElement.LENGTHADJUST_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub const LENGTHADJUST_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGTextContentElement.LENGTHADJUST_SPACING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub const LENGTHADJUST_SPACING: u16 = 1u64 as u16;
    #[doc = "The `SVGTextContentElement.LENGTHADJUST_SPACINGANDGLYPHS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub const LENGTHADJUST_SPACINGANDGLYPHS: u16 = 2u64 as u16;
}

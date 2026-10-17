#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssFontFeatureValuesRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssFontFeatureValuesRule(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `fontFamily` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
impl CssFontFeatureValuesRule {
    #[js::import = "$0.fontFamily"]
pub fn font_family(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `fontFamily` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
impl CssFontFeatureValuesRule {
    #[js::import = "$0.fontFamily = $1"]
pub fn set_font_family(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `valueText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
impl CssFontFeatureValuesRule {
    #[js::import = "$0.valueText"]
pub fn value_text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `valueText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/valueText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFeatureValuesRule`*"]
impl CssFontFeatureValuesRule {
    #[js::import = "$0.valueText = $1"]
pub fn set_value_text(&self, value: &str) { panic!("web-sys intrinsic") }
}

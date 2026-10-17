#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgStyleElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgStyleElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `xmlspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
impl SvgStyleElement {
    #[js::import = "$0.xmlspace"]
pub fn xmlspace(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `xmlspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
impl SvgStyleElement {
    #[js::import = "$0.xmlspace = $1"]
pub fn set_xmlspace(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
impl SvgStyleElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
impl SvgStyleElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
impl SvgStyleElement {
    #[js::import = "$0.media"]
pub fn media(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
impl SvgStyleElement {
    #[js::import = "$0.media = $1"]
pub fn set_media(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
impl SvgStyleElement {
    #[js::import = "$0.title"]
pub fn title(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
impl SvgStyleElement {
    #[js::import = "$0.title = $1"]
pub fn set_title(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StyleSheet")]
    #[doc = "Getter for the `sheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/sheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`, `SvgStyleElement`*"]
impl SvgStyleElement {
    #[js::import = "$0.sheet"]
pub fn sheet(&self) -> Option<StyleSheet> { panic!("web-sys intrinsic") }
}

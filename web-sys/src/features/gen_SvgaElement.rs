#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgaElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgaElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.target"]
pub fn target(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `download` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.download"]
pub fn download(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `download` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.download = $1"]
pub fn set_download(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.ping"]
pub fn ping(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.ping = $1"]
pub fn set_ping(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.rel"]
pub fn rel(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.rel = $1"]
pub fn set_rel(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.referrerPolicy"]
pub fn referrer_policy(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.referrerPolicy = $1"]
pub fn set_referrer_policy(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `relList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/relList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.relList"]
pub fn rel_list(&self) -> DomTokenList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.hreflang"]
pub fn hreflang(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.hreflang = $1"]
pub fn set_hreflang(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.text"]
pub fn text(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.text = $1"]
pub fn set_text(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*"]
impl SvgaElement {
    #[js::import = "$0.href"]
pub fn href(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}

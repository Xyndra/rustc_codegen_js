#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLinkElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlLinkElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.disabled"]
pub fn disabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.disabled = $1"]
pub fn set_disabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.href"]
pub fn href(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.href = $1"]
pub fn set_href(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.crossOrigin"]
pub fn cross_origin(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.crossOrigin = $1"]
pub fn set_cross_origin(&self, value: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.rel"]
pub fn rel(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.rel = $1"]
pub fn set_rel(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `relList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/relList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.relList"]
pub fn rel_list(&self) -> DomTokenList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.media"]
pub fn media(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.media = $1"]
pub fn set_media(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.hreflang"]
pub fn hreflang(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.hreflang = $1"]
pub fn set_hreflang(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.referrerPolicy"]
pub fn referrer_policy(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.referrerPolicy = $1"]
pub fn set_referrer_policy(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `sizes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sizes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.sizes"]
pub fn sizes(&self) -> DomTokenList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.charset"]
pub fn charset(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.charset = $1"]
pub fn set_charset(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rev` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.rev"]
pub fn rev(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `rev` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.rev = $1"]
pub fn set_rev(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.target"]
pub fn target(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.target = $1"]
pub fn set_target(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.integrity"]
pub fn integrity(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.integrity = $1"]
pub fn set_integrity(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `as` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.as"]
pub fn as_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `as` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
impl HtmlLinkElement {
    #[js::import = "$0.as = $1"]
pub fn set_as(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StyleSheet")]
    #[doc = "Getter for the `sheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLinkElement`, `StyleSheet`*"]
impl HtmlLinkElement {
    #[js::import = "$0.sheet"]
pub fn sheet(&self) -> Option<StyleSheet> { panic!("web-sys intrinsic") }
}

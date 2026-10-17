#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTrackElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlTrackElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.kind"]
pub fn kind(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.kind = $1"]
pub fn set_kind(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.src"]
pub fn src(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.src = $1"]
pub fn set_src(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `srclang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.srclang"]
pub fn srclang(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `srclang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.srclang = $1"]
pub fn set_srclang(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.label = $1"]
pub fn set_label(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `default` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.default"]
pub fn default(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `default` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.default = $1"]
pub fn set_default(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
impl HtmlTrackElement {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextTrack")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`, `TextTrack`*"]
impl HtmlTrackElement {
    #[js::import = "$0.track"]
pub fn track(&self) -> Option<TextTrack> { panic!("web-sys intrinsic") }
}
impl HtmlTrackElement {
    #[doc = "The `HTMLTrackElement.NONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub const NONE: u16 = 0i64 as u16;
    #[doc = "The `HTMLTrackElement.LOADING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub const LOADING: u16 = 1u64 as u16;
    #[doc = "The `HTMLTrackElement.LOADED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub const LOADED: u16 = 2u64 as u16;
    #[doc = "The `HTMLTrackElement.ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub const ERROR: u16 = 3u64 as u16;
}

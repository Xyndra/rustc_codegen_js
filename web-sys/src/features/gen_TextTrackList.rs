#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrackList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TextTrackList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
impl TextTrackList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
impl TextTrackList {
    #[js::import = "$0.onchange"]
pub fn onchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
impl TextTrackList {
    #[js::import = "$0.onchange = $1"]
pub fn set_onchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
impl TextTrackList {
    #[js::import = "$0.onaddtrack"]
pub fn onaddtrack(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
impl TextTrackList {
    #[js::import = "$0.onaddtrack = $1"]
pub fn set_onaddtrack(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
impl TextTrackList {
    #[js::import = "$0.onremovetrack"]
pub fn onremovetrack(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackList`*"]
impl TextTrackList {
    #[js::import = "$0.onremovetrack = $1"]
pub fn set_onremovetrack(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextTrack")]
    #[doc = "The `getTrackById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/getTrackById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackList`*"]
impl TextTrackList {
    #[js::import = "$0.getTrackById($1)"]
pub fn get_track_by_id(&self, id: &str) -> Option<TextTrack> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextTrack")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackList`*"]
impl TextTrackList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<TextTrack> { panic!("web-sys intrinsic") }
}

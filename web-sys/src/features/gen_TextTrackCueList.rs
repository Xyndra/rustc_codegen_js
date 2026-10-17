#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrackCueList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCueList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TextTrackCueList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCueList`*"]
impl TextTrackCueList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VttCue")]
    #[doc = "The `getCueById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/getCueById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCueList`, `VttCue`*"]
impl TextTrackCueList {
    #[js::import = "$0.getCueById($1)"]
pub fn get_cue_by_id(&self, id: &str) -> Option<VttCue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VttCue")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCueList`, `VttCue`*"]
impl TextTrackCueList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<VttCue> { panic!("web-sys intrinsic") }
}

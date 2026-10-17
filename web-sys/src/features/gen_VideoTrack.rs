#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VideoTrack(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
impl VideoTrack {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
impl VideoTrack {
    #[js::import = "$0.kind"]
pub fn kind(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
impl VideoTrack {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
impl VideoTrack {
    #[js::import = "$0.language"]
pub fn language(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `selected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
impl VideoTrack {
    #[js::import = "$0.selected"]
pub fn selected(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `selected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoTrack`*"]
impl VideoTrack {
    #[js::import = "$0.selected = $1"]
pub fn set_selected(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SourceBuffer")]
    #[doc = "Getter for the `sourceBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/sourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `VideoTrack`*"]
impl VideoTrack {
    #[js::import = "$0.sourceBuffer"]
pub fn source_buffer(&self) -> Option<SourceBuffer> { panic!("web-sys intrinsic") }
}

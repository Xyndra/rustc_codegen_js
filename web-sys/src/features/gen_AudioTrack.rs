#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioTrack(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
impl AudioTrack {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
impl AudioTrack {
    #[js::import = "$0.kind"]
pub fn kind(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
impl AudioTrack {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
impl AudioTrack {
    #[js::import = "$0.language"]
pub fn language(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
impl AudioTrack {
    #[js::import = "$0.enabled"]
pub fn enabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`*"]
impl AudioTrack {
    #[js::import = "$0.enabled = $1"]
pub fn set_enabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SourceBuffer")]
    #[doc = "Getter for the `sourceBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/sourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrack`, `SourceBuffer`*"]
impl AudioTrack {
    #[js::import = "$0.sourceBuffer"]
pub fn source_buffer(&self) -> Option<SourceBuffer> { panic!("web-sys intrinsic") }
}

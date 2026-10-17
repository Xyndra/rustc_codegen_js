#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TextTrack(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "TextTrackKind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackKind`*"]
impl TextTrack {
    #[js::import = "$0.kind"]
pub fn kind(&self) -> TextTrackKind { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
impl TextTrack {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
impl TextTrack {
    #[js::import = "$0.language"]
pub fn language(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
impl TextTrack {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `inBandMetadataTrackDispatchType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/inBandMetadataTrackDispatchType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
impl TextTrack {
    #[js::import = "$0.inBandMetadataTrackDispatchType"]
pub fn in_band_metadata_track_dispatch_type(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextTrackMode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*"]
impl TextTrack {
    #[js::import = "$0.mode"]
pub fn mode(&self) -> TextTrackMode { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextTrackMode")]
    #[doc = "Setter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*"]
impl TextTrack {
    #[js::import = "$0.mode = $1"]
pub fn set_mode(&self, value: TextTrackMode) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextTrackCueList")]
    #[doc = "Getter for the `cues` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/cues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*"]
impl TextTrack {
    #[js::import = "$0.cues"]
pub fn cues(&self) -> Option<TextTrackCueList> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextTrackCueList")]
    #[doc = "Getter for the `activeCues` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/activeCues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*"]
impl TextTrack {
    #[js::import = "$0.activeCues"]
pub fn active_cues(&self) -> Option<TextTrackCueList> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncuechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
impl TextTrack {
    #[js::import = "$0.oncuechange"]
pub fn oncuechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncuechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`*"]
impl TextTrack {
    #[js::import = "$0.oncuechange = $1"]
pub fn set_oncuechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SourceBuffer")]
    #[doc = "Getter for the `sourceBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/sourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `TextTrack`*"]
impl TextTrack {
    #[js::import = "$0.sourceBuffer"]
pub fn source_buffer(&self) -> Option<SourceBuffer> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VttCue")]
    #[doc = "The `addCue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/addCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*"]
impl TextTrack {
    #[js::import = "$0.addCue($1)"]
pub fn add_cue(&self, cue: &VttCue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VttCue")]
    #[doc = "The `removeCue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/removeCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*"]
impl TextTrack {
    #[js::import = "$0.removeCue($1)"]
pub fn remove_cue(&self, cue: &VttCue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}

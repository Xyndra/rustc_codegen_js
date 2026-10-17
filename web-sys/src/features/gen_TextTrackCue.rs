#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextTrackCue` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TextTrackCue(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "TextTrack")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.track"]
pub fn track(&self) -> Option<TextTrack> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.id = $1"]
pub fn set_id(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.startTime"]
pub fn start_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.startTime = $1"]
pub fn set_start_time(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.endTime"]
pub fn end_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.endTime = $1"]
pub fn set_end_time(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pauseOnExit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.pauseOnExit"]
pub fn pause_on_exit(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `pauseOnExit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.pauseOnExit = $1"]
pub fn set_pause_on_exit(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.onenter"]
pub fn onenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.onenter = $1"]
pub fn set_onenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.onexit"]
pub fn onexit(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextTrackCue`*"]
impl TextTrackCue {
    #[js::import = "$0.onexit = $1"]
pub fn set_onexit(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}

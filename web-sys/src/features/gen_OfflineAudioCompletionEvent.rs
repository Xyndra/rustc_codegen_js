#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineAudioCompletionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OfflineAudioCompletionEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `renderedBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent/renderedBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEvent`*"]
impl OfflineAudioCompletionEvent {
    #[js::import = "$0.renderedBuffer"]
pub fn rendered_buffer(&self) -> AudioBuffer { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OfflineAudioCompletionEventInit")]
    #[doc = "The `new OfflineAudioCompletionEvent(..)` constructor, creating a new instance of `OfflineAudioCompletionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent/OfflineAudioCompletionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEvent`, `OfflineAudioCompletionEventInit`*"]
impl OfflineAudioCompletionEvent {
    #[js::import = "new OfflineAudioCompletionEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &OfflineAudioCompletionEventInit,) -> Result<OfflineAudioCompletionEvent, JsValue> { panic!("web-sys intrinsic") }
}

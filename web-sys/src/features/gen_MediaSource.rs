#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaSource` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaSource(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SourceBufferList")]
    #[doc = "Getter for the `sourceBuffers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/sourceBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*"]
impl MediaSource {
    #[js::import = "$0.sourceBuffers"]
pub fn source_buffers(&self) -> SourceBufferList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SourceBufferList")]
    #[doc = "Getter for the `activeSourceBuffers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/activeSourceBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*"]
impl MediaSource {
    #[js::import = "$0.activeSourceBuffers"]
pub fn active_source_buffers(&self) -> SourceBufferList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaSourceReadyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceReadyState`*"]
impl MediaSource {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> MediaSourceReadyState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.duration"]
pub fn duration(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.duration = $1"]
pub fn set_duration(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsourceopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.onsourceopen"]
pub fn onsourceopen(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsourceopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.onsourceopen = $1"]
pub fn set_onsourceopen(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsourceended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.onsourceended"]
pub fn onsourceended(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsourceended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.onsourceended = $1"]
pub fn set_onsourceended(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsourceclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.onsourceclose"]
pub fn onsourceclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsourceclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.onsourceclose = $1"]
pub fn set_onsourceclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MediaSource(..)` constructor, creating a new instance of `MediaSource`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/MediaSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "new MediaSource()"]
pub fn new() -> Result<MediaSource, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SourceBuffer")]
    #[doc = "The `addSourceBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/addSourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*"]
impl MediaSource {
    #[js::import = "$0.addSourceBuffer($1)"]
pub fn add_source_buffer(&self, type_: &str) -> Result<SourceBuffer, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearLiveSeekableRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/clearLiveSeekableRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.clearLiveSeekableRange()"]
pub fn clear_live_seekable_range(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `endOfStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.endOfStream()"]
pub fn end_of_stream(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaSourceEndOfStreamError")]
    #[doc = "The `endOfStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceEndOfStreamError`*"]
impl MediaSource {
    #[js::import = "$0.endOfStream($1)"]
pub fn end_of_stream_with_error(&self, error: MediaSourceEndOfStreamError) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `isTypeSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/isTypeSupported_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "MediaSource.isTypeSupported"]
pub fn is_type_supported(type_: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SourceBuffer")]
    #[doc = "The `removeSourceBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/removeSourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*"]
impl MediaSource {
    #[js::import = "$0.removeSourceBuffer($1)"]
pub fn remove_source_buffer(&self, source_buffer: &SourceBuffer) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setLiveSeekableRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/setLiveSeekableRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSource`*"]
impl MediaSource {
    #[js::import = "$0.setLiveSeekableRange($1, $2)"]
pub fn set_live_seekable_range(&self, start: f64, end: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}

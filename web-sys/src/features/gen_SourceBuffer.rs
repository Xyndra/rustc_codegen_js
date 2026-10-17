#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SourceBuffer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SourceBuffer(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SourceBufferAppendMode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*"]
impl SourceBuffer {
    #[js::import = "$0.mode"]
pub fn mode(&self) -> SourceBufferAppendMode { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SourceBufferAppendMode")]
    #[doc = "Setter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*"]
impl SourceBuffer {
    #[js::import = "$0.mode = $1"]
pub fn set_mode(&self, value: SourceBufferAppendMode) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `updating` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/updating)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.updating"]
pub fn updating(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TimeRanges")]
    #[doc = "Getter for the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/buffered)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `TimeRanges`*"]
impl SourceBuffer {
    #[js::import = "$0.buffered"]
pub fn buffered(&self) -> Result<TimeRanges, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `timestampOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.timestampOffset"]
pub fn timestamp_offset(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `timestampOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.timestampOffset = $1"]
pub fn set_timestamp_offset(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioTrackList")]
    #[doc = "Getter for the `audioTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/audioTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`, `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.audioTracks"]
pub fn audio_tracks(&self) -> AudioTrackList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VideoTrackList")]
    #[doc = "Getter for the `videoTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/videoTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `VideoTrackList`*"]
impl SourceBuffer {
    #[js::import = "$0.videoTracks"]
pub fn video_tracks(&self) -> VideoTrackList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextTrackList")]
    #[doc = "Getter for the `textTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/textTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `TextTrackList`*"]
impl SourceBuffer {
    #[js::import = "$0.textTracks"]
pub fn text_tracks(&self) -> TextTrackList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `appendWindowStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendWindowStart"]
pub fn append_window_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `appendWindowStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendWindowStart = $1"]
pub fn set_append_window_start(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `appendWindowEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendWindowEnd"]
pub fn append_window_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `appendWindowEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendWindowEnd = $1"]
pub fn set_append_window_end(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onupdatestart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onupdatestart"]
pub fn onupdatestart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onupdatestart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onupdatestart = $1"]
pub fn set_onupdatestart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onupdate"]
pub fn onupdate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onupdate = $1"]
pub fn set_onupdate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onupdateend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onupdateend"]
pub fn onupdateend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onupdateend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onupdateend = $1"]
pub fn set_onupdateend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.abort()"]
pub fn abort(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendBuffer($1)"]
pub fn append_buffer_with_array_buffer(&self, data: &::js_sys::ArrayBuffer) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendBuffer($1)"]
pub fn append_buffer_with_array_buffer_view(&self, data: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendBuffer($1)"]
pub fn append_buffer_with_u8_array(&self, data: &mut [u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendBuffer($1)"]
pub fn append_buffer_with_js_u8_array(&self, data: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendBufferAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendBufferAsync($1)"]
pub fn append_buffer_async_with_array_buffer(&self, data: &::js_sys::ArrayBuffer) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendBufferAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendBufferAsync($1)"]
pub fn append_buffer_async_with_array_buffer_view(&self, data: &::js_sys::Object) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendBufferAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendBufferAsync($1)"]
pub fn append_buffer_async_with_u8_array(&self, data: &mut [u8]) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendBufferAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.appendBufferAsync($1)"]
pub fn append_buffer_async_with_js_u8_array(&self, data: &::js_sys::Uint8Array) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `changeType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/changeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.changeType($1)"]
pub fn change_type(&self, type_: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.remove($1, $2)"]
pub fn remove(&self, start: f64, end: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/removeAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`*"]
impl SourceBuffer {
    #[js::import = "$0.removeAsync($1, $2)"]
pub fn remove_async(&self, start: f64, end: f64) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}

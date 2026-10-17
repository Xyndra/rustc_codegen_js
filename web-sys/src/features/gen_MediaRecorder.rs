#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaRecorder(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStream")]
    #[doc = "Getter for the `stream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*"]
impl MediaRecorder {
    #[js::import = "$0.stream"]
pub fn stream(&self) -> MediaStream { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RecordingState")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`, `RecordingState`*"]
impl MediaRecorder {
    #[js::import = "$0.state"]
pub fn state(&self) -> RecordingState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/mimeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.mimeType"]
pub fn mime_type(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondataavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.ondataavailable"]
pub fn ondataavailable(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondataavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.ondataavailable = $1"]
pub fn set_ondataavailable(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.onstart"]
pub fn onstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.onstart = $1"]
pub fn set_onstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.onstop"]
pub fn onstop(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.onstop = $1"]
pub fn set_onstop(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwarning` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onwarning)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.onwarning"]
pub fn onwarning(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwarning` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onwarning)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.onwarning = $1"]
pub fn set_onwarning(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStream")]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*"]
impl MediaRecorder {
    #[js::import = "new MediaRecorder($0)"]
pub fn new_with_media_stream(stream: &MediaStream) -> Result<MediaRecorder, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "MediaRecorderOptions", feature = "MediaStream",))]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`, `MediaRecorderOptions`, `MediaStream`*"]
impl MediaRecorder {
    #[js::import = "new MediaRecorder($0, $1)"]
pub fn new_with_media_stream_and_media_recorder_options(stream: &MediaStream, options: &MediaRecorderOptions,) -> Result<MediaRecorder, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioNode")]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "new MediaRecorder($0)"]
pub fn new_with_audio_node(node: &AudioNode) -> Result<MediaRecorder, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioNode")]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "new MediaRecorder($0, $1)"]
pub fn new_with_audio_node_and_u32(node: &AudioNode, output: u32,) -> Result<MediaRecorder, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "AudioNode", feature = "MediaRecorderOptions",))]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`, `MediaRecorderOptions`*"]
impl MediaRecorder {
    #[js::import = "new MediaRecorder($0, $1, $2)"]
pub fn new_with_audio_node_and_u32_and_options(node: &AudioNode, output: u32, options: &MediaRecorderOptions,) -> Result<MediaRecorder, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `isTypeSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/isTypeSupported_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "MediaRecorder.isTypeSupported"]
pub fn is_type_supported(type_: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `pause()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/pause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.pause()"]
pub fn pause(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/requestData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.requestData()"]
pub fn request_data(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.resume()"]
pub fn resume(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.start()"]
pub fn start(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.start($1)"]
pub fn start_with_time_slice(&self, time_slice: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorder`*"]
impl MediaRecorder {
    #[js::import = "$0.stop()"]
pub fn stop(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}

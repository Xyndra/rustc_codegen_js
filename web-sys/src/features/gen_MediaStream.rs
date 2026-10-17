#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStream(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `active` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/active)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.active"]
pub fn active(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.onaddtrack"]
pub fn onaddtrack(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.onaddtrack = $1"]
pub fn set_onaddtrack(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.onremovetrack"]
pub fn onremovetrack(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onremovetrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.onremovetrack = $1"]
pub fn set_onremovetrack(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.currentTime"]
pub fn current_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "new MediaStream()"]
pub fn new() -> Result<MediaStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "new MediaStream($0)"]
pub fn new_with_stream(stream: &MediaStream) -> Result<MediaStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "new MediaStream($0)"]
pub fn new_with_tracks(tracks: &::wasm_bindgen::JsValue) -> Result<MediaStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*"]
impl MediaStream {
    #[js::import = "$0.addTrack($1)"]
pub fn add_track(&self, track: &MediaStreamTrack) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.clone()"]
pub fn clone(&self) -> MediaStream { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAudioTracks()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getAudioTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.getAudioTracks()"]
pub fn get_audio_tracks(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "The `getTrackById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTrackById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*"]
impl MediaStream {
    #[js::import = "$0.getTrackById($1)"]
pub fn get_track_by_id(&self, track_id: &str) -> Option<MediaStreamTrack> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getTracks()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.getTracks()"]
pub fn get_tracks(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getVideoTracks()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getVideoTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`*"]
impl MediaStream {
    #[js::import = "$0.getVideoTracks()"]
pub fn get_video_tracks(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "The `removeTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/removeTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*"]
impl MediaStream {
    #[js::import = "$0.removeTrack($1)"]
pub fn remove_track(&self, track: &MediaStreamTrack) { panic!("web-sys intrinsic") }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMediaElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlMediaElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaError")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaError`*"]
impl HtmlMediaElement {
    #[js::import = "$0.error"]
pub fn error(&self) -> Option<MediaError> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.src"]
pub fn src(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.src = $1"]
pub fn set_src(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `currentSrc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentSrc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.currentSrc"]
pub fn current_src(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStream")]
    #[doc = "Getter for the `srcObject` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*"]
impl HtmlMediaElement {
    #[js::import = "$0.srcObject"]
pub fn src_object(&self) -> Option<MediaStream> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStream")]
    #[doc = "Setter for the `srcObject` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaStream`*"]
impl HtmlMediaElement {
    #[js::import = "$0.srcObject = $1"]
pub fn set_src_object(&self, value: Option<&MediaStream>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.crossOrigin"]
pub fn cross_origin(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.crossOrigin = $1"]
pub fn set_cross_origin(&self, value: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `networkState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/networkState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.networkState"]
pub fn network_state(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `preload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.preload"]
pub fn preload(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `preload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.preload = $1"]
pub fn set_preload(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TimeRanges")]
    #[doc = "Getter for the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/buffered)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
impl HtmlMediaElement {
    #[js::import = "$0.buffered"]
pub fn buffered(&self) -> TimeRanges { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `seeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.seeking"]
pub fn seeking(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.currentTime"]
pub fn current_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.currentTime = $1"]
pub fn set_current_time(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.duration"]
pub fn duration(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `paused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/paused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.paused"]
pub fn paused(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `defaultPlaybackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.defaultPlaybackRate"]
pub fn default_playback_rate(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `defaultPlaybackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.defaultPlaybackRate = $1"]
pub fn set_default_playback_rate(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.playbackRate"]
pub fn playback_rate(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.playbackRate = $1"]
pub fn set_playback_rate(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TimeRanges")]
    #[doc = "Getter for the `played` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/played)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
impl HtmlMediaElement {
    #[js::import = "$0.played"]
pub fn played(&self) -> TimeRanges { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TimeRanges")]
    #[doc = "Getter for the `seekable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TimeRanges`*"]
impl HtmlMediaElement {
    #[js::import = "$0.seekable"]
pub fn seekable(&self) -> TimeRanges { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.ended"]
pub fn ended(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `autoplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.autoplay"]
pub fn autoplay(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `autoplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.autoplay = $1"]
pub fn set_autoplay(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `loop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.loop"]
pub fn loop_(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `loop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.loop = $1"]
pub fn set_loop(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `controls` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.controls"]
pub fn controls(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `controls` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.controls = $1"]
pub fn set_controls(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `volume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.volume"]
pub fn volume(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `volume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.volume = $1"]
pub fn set_volume(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `muted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.muted"]
pub fn muted(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `muted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.muted = $1"]
pub fn set_muted(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `defaultMuted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.defaultMuted"]
pub fn default_muted(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `defaultMuted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.defaultMuted = $1"]
pub fn set_default_muted(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioTrackList")]
    #[doc = "Getter for the `audioTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/audioTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioTrackList`, `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.audioTracks"]
pub fn audio_tracks(&self) -> AudioTrackList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VideoTrackList")]
    #[doc = "Getter for the `videoTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/videoTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `VideoTrackList`*"]
impl HtmlMediaElement {
    #[js::import = "$0.videoTracks"]
pub fn video_tracks(&self) -> VideoTrackList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextTrackList")]
    #[doc = "Getter for the `textTracks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/textTracks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrackList`*"]
impl HtmlMediaElement {
    #[js::import = "$0.textTracks"]
pub fn text_tracks(&self) -> Option<TextTrackList> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaKeys")]
    #[doc = "Getter for the `mediaKeys` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/mediaKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*"]
impl HtmlMediaElement {
    #[js::import = "$0.mediaKeys"]
pub fn media_keys(&self) -> Option<MediaKeys> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onencrypted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.onencrypted"]
pub fn onencrypted(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onencrypted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.onencrypted = $1"]
pub fn set_onencrypted(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwaitingforkey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.onwaitingforkey"]
pub fn onwaitingforkey(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwaitingforkey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.onwaitingforkey = $1"]
pub fn set_onwaitingforkey(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    #[doc = "The `addTextTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
impl HtmlMediaElement {
    #[js::import = "$0.addTextTrack($1)"]
pub fn add_text_track(&self, kind: TextTrackKind) -> TextTrack { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    #[doc = "The `addTextTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
impl HtmlMediaElement {
    #[js::import = "$0.addTextTrack($1, $2)"]
pub fn add_text_track_with_label(&self, kind: TextTrackKind, label: &str) -> TextTrack { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    #[doc = "The `addTextTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `TextTrack`, `TextTrackKind`*"]
impl HtmlMediaElement {
    #[js::import = "$0.addTextTrack($1, $2, $3)"]
pub fn add_text_track_with_label_and_language(&self, kind: TextTrackKind, label: &str, language: &str) -> TextTrack { panic!("web-sys intrinsic") }
}
    #[doc = "The `canPlayType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canPlayType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.canPlayType($1)"]
pub fn can_play_type(&self, type_: &str) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `fastSeek()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/fastSeek)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.fastSeek($1)"]
pub fn fast_seek(&self, time: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasSuspendTaint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/hasSuspendTaint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.hasSuspendTaint()"]
pub fn has_suspend_taint(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.load()"]
pub fn load(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `pause()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.pause()"]
pub fn pause(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `play()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.play()"]
pub fn play(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `seekToNextFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekToNextFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.seekToNextFrame()"]
pub fn seek_to_next_frame(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaKeys")]
    #[doc = "The `setMediaKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setMediaKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaKeys`*"]
impl HtmlMediaElement {
    #[js::import = "$0.setMediaKeys($1)"]
pub fn set_media_keys(&self, media_keys: Option<&MediaKeys>) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `setVisible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setVisible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
impl HtmlMediaElement {
    #[js::import = "$0.setVisible($1)"]
pub fn set_visible(&self, a_visible: bool) { panic!("web-sys intrinsic") }
}
impl HtmlMediaElement {
    #[doc = "The `HTMLMediaElement.NETWORK_EMPTY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const NETWORK_EMPTY: u16 = 0i64 as u16;
    #[doc = "The `HTMLMediaElement.NETWORK_IDLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const NETWORK_IDLE: u16 = 1u64 as u16;
    #[doc = "The `HTMLMediaElement.NETWORK_LOADING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const NETWORK_LOADING: u16 = 2u64 as u16;
    #[doc = "The `HTMLMediaElement.NETWORK_NO_SOURCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const NETWORK_NO_SOURCE: u16 = 3u64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_NOTHING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_NOTHING: u16 = 0i64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_METADATA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_METADATA: u16 = 1u64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_CURRENT_DATA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_CURRENT_DATA: u16 = 2u64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_FUTURE_DATA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_FUTURE_DATA: u16 = 3u64 as u16;
    #[doc = "The `HTMLMediaElement.HAVE_ENOUGH_DATA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`*"]
    pub const HAVE_ENOUGH_DATA: u16 = 4u64 as u16;
}

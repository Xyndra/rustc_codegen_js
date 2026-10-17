#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaSession` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaSession(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaMetadata")]
    #[doc = "Getter for the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/metadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadata`, `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl MediaSession {
    #[js::import = "$0.metadata"]
pub fn metadata(&self) -> Option<MediaMetadata> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaMetadata")]
    #[doc = "Setter for the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/metadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaMetadata`, `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl MediaSession {
    #[js::import = "$0.metadata = $1"]
pub fn set_metadata(&self, value: Option<&MediaMetadata>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionPlaybackState")]
    #[doc = "Getter for the `playbackState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/playbackState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`, `MediaSessionPlaybackState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl MediaSession {
    #[js::import = "$0.playbackState"]
pub fn playback_state(&self) -> MediaSessionPlaybackState { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionPlaybackState")]
    #[doc = "Setter for the `playbackState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/playbackState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`, `MediaSessionPlaybackState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl MediaSession {
    #[js::import = "$0.playbackState = $1"]
pub fn set_playback_state(&self, value: MediaSessionPlaybackState) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "The `setActionHandler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setActionHandler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`, `MediaSessionAction`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl MediaSession {
    #[js::import = "$0.setActionHandler($1, $2)"]
pub fn set_action_handler(&self, action: MediaSessionAction, handler: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `setCameraActive()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setCameraActive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl MediaSession {
    #[js::import = "$0.setCameraActive($1)"]
pub fn set_camera_active(&self, active: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `setMicrophoneActive()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setMicrophoneActive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl MediaSession {
    #[js::import = "$0.setMicrophoneActive($1)"]
pub fn set_microphone_active(&self, active: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `setPositionState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setPositionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl MediaSession {
    #[js::import = "$0.setPositionState()"]
pub fn set_position_state(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaPositionState")]
    #[doc = "The `setPositionState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSession/setPositionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`, `MediaSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl MediaSession {
    #[js::import = "$0.setPositionState($1)"]
pub fn set_position_state_with_state(&self, state: &MediaPositionState) { panic!("web-sys intrinsic") }
}

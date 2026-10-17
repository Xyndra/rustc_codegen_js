#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoPlaybackQuality` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VideoPlaybackQuality(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `creationTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/creationTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
impl VideoPlaybackQuality {
    #[js::import = "$0.creationTime"]
pub fn creation_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `totalVideoFrames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/totalVideoFrames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
impl VideoPlaybackQuality {
    #[js::import = "$0.totalVideoFrames"]
pub fn total_video_frames(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `droppedVideoFrames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/droppedVideoFrames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
impl VideoPlaybackQuality {
    #[js::import = "$0.droppedVideoFrames"]
pub fn dropped_video_frames(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `corruptedVideoFrames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoPlaybackQuality/corruptedVideoFrames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoPlaybackQuality`*"]
impl VideoPlaybackQuality {
    #[js::import = "$0.corruptedVideoFrames"]
pub fn corrupted_video_frames(&self) -> u32 { panic!("web-sys intrinsic") }
}

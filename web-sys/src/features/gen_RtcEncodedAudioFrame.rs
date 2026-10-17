#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcEncodedAudioFrame` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcEncodedAudioFrame(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedAudioFrame {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::js_sys::ArrayBuffer { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedAudioFrame {
    #[js::import = "$0.data = $1"]
pub fn set_data(&self, value: &::js_sys::ArrayBuffer) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new RtcEncodedAudioFrame(..)` constructor, creating a new instance of `RtcEncodedAudioFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/RTCEncodedAudioFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedAudioFrame {
    #[js::import = "new RTCEncodedAudioFrame($0)"]
pub fn new(original_frame: &RtcEncodedAudioFrame) -> Result<RtcEncodedAudioFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameOptions")]
    #[doc = "The `new RtcEncodedAudioFrame(..)` constructor, creating a new instance of `RtcEncodedAudioFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/RTCEncodedAudioFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`, `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedAudioFrame {
    #[js::import = "new RTCEncodedAudioFrame($0, $1)"]
pub fn new_with_options(original_frame: &RtcEncodedAudioFrame, options: &RtcEncodedAudioFrameOptions,) -> Result<RtcEncodedAudioFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameMetadata")]
    #[doc = "The `getMetadata()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedAudioFrame/getMetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrame`, `RtcEncodedAudioFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedAudioFrame {
    #[js::import = "$0.getMetadata()"]
pub fn get_metadata(&self) -> RtcEncodedAudioFrameMetadata { panic!("web-sys intrinsic") }
}

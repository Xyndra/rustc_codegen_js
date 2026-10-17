#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcEncodedVideoFrame` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcEncodedVideoFrame(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedVideoFrameType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrame`, `RtcEncodedVideoFrameType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedVideoFrame {
    #[js::import = "$0.type"]
pub fn type_(&self) -> RtcEncodedVideoFrameType { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedVideoFrame {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::js_sys::ArrayBuffer { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedVideoFrame {
    #[js::import = "$0.data = $1"]
pub fn set_data(&self, value: &::js_sys::ArrayBuffer) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new RtcEncodedVideoFrame(..)` constructor, creating a new instance of `RtcEncodedVideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/RTCEncodedVideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedVideoFrame {
    #[js::import = "new RTCEncodedVideoFrame($0)"]
pub fn new(original_frame: &RtcEncodedVideoFrame) -> Result<RtcEncodedVideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedVideoFrameOptions")]
    #[doc = "The `new RtcEncodedVideoFrame(..)` constructor, creating a new instance of `RtcEncodedVideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/RTCEncodedVideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrame`, `RtcEncodedVideoFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedVideoFrame {
    #[js::import = "new RTCEncodedVideoFrame($0, $1)"]
pub fn new_with_options(original_frame: &RtcEncodedVideoFrame, options: &RtcEncodedVideoFrameOptions,) -> Result<RtcEncodedVideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedVideoFrameMetadata")]
    #[doc = "The `getMetadata()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCEncodedVideoFrame/getMetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrame`, `RtcEncodedVideoFrameMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcEncodedVideoFrame {
    #[js::import = "$0.getMetadata()"]
pub fn get_metadata(&self) -> RtcEncodedVideoFrameMetadata { panic!("web-sys intrinsic") }
}

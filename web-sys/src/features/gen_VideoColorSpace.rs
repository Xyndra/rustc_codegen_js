#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoColorSpace` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct VideoColorSpace(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorPrimaries")]
    #[doc = "Getter for the `primaries` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/primaries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`, `VideoColorSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoColorSpace {
    #[js::import = "$0.primaries"]
pub fn primaries(&self) -> Option<VideoColorPrimaries> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[doc = "Getter for the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/transfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoTransferCharacteristics`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoColorSpace {
    #[js::import = "$0.transfer"]
pub fn transfer(&self) -> Option<VideoTransferCharacteristics> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[doc = "Getter for the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/matrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoMatrixCoefficients`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoColorSpace {
    #[js::import = "$0.matrix"]
pub fn matrix(&self) -> Option<VideoMatrixCoefficients> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `fullRange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/fullRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoColorSpace {
    #[js::import = "$0.fullRange"]
pub fn full_range(&self) -> Option<bool> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new VideoColorSpace(..)` constructor, creating a new instance of `VideoColorSpace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/VideoColorSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoColorSpace {
    #[js::import = "new VideoColorSpace()"]
pub fn new() -> Result<VideoColorSpace, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    #[doc = "The `new VideoColorSpace(..)` constructor, creating a new instance of `VideoColorSpace`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/VideoColorSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoColorSpace {
    #[js::import = "new VideoColorSpace($0)"]
pub fn new_with_init(init: &VideoColorSpaceInit) -> Result<VideoColorSpace, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpaceInit")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoColorSpace/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoColorSpace {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> VideoColorSpaceInit { panic!("web-sys intrinsic") }
}

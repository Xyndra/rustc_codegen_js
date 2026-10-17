#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrSubmitFrameResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrSubmitFrameResult(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `frameNum` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/frameNum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
impl VrSubmitFrameResult {
    #[js::import = "$0.frameNum"]
pub fn frame_num(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `base64Image` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/base64Image)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
impl VrSubmitFrameResult {
    #[js::import = "$0.base64Image"]
pub fn base64_image(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new VrSubmitFrameResult(..)` constructor, creating a new instance of `VrSubmitFrameResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/VRSubmitFrameResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
impl VrSubmitFrameResult {
    #[js::import = "new VRSubmitFrameResult()"]
pub fn new() -> Result<VrSubmitFrameResult, JsValue> { panic!("web-sys intrinsic") }
}

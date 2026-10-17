#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrStageParameters` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrStageParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrStageParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `sittingToStandingTransform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sittingToStandingTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrStageParameters`*"]
impl VrStageParameters {
    #[js::import = "$0.sittingToStandingTransform"]
pub fn sitting_to_standing_transform(&self) -> Result<::alloc::vec::Vec<f32>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sizeX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sizeX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrStageParameters`*"]
impl VrStageParameters {
    #[js::import = "$0.sizeX"]
pub fn size_x(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sizeZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sizeZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrStageParameters`*"]
impl VrStageParameters {
    #[js::import = "$0.sizeZ"]
pub fn size_z(&self) -> f32 { panic!("web-sys intrinsic") }
}

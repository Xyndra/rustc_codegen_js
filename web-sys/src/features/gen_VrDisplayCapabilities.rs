#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrDisplayCapabilities` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrDisplayCapabilities(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `hasPosition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
impl VrDisplayCapabilities {
    #[js::import = "$0.hasPosition"]
pub fn has_position(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hasOrientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
impl VrDisplayCapabilities {
    #[js::import = "$0.hasOrientation"]
pub fn has_orientation(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hasExternalDisplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasExternalDisplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
impl VrDisplayCapabilities {
    #[js::import = "$0.hasExternalDisplay"]
pub fn has_external_display(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `canPresent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/canPresent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
impl VrDisplayCapabilities {
    #[js::import = "$0.canPresent"]
pub fn can_present(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxLayers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/maxLayers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
impl VrDisplayCapabilities {
    #[js::import = "$0.maxLayers"]
pub fn max_layers(&self) -> u32 { panic!("web-sys intrinsic") }
}

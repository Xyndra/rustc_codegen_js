#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrEyeParameters` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrEyeParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`*"]
impl VrEyeParameters {
    #[js::import = "$0.offset"]
pub fn offset(&self) -> Result<::alloc::vec::Vec<f32>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VrFieldOfView")]
    #[doc = "Getter for the `fieldOfView` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/fieldOfView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`, `VrFieldOfView`*"]
impl VrEyeParameters {
    #[js::import = "$0.fieldOfView"]
pub fn field_of_view(&self) -> VrFieldOfView { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `renderWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/renderWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`*"]
impl VrEyeParameters {
    #[js::import = "$0.renderWidth"]
pub fn render_width(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `renderHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VREyeParameters/renderHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEyeParameters`*"]
impl VrEyeParameters {
    #[js::import = "$0.renderHeight"]
pub fn render_height(&self) -> u32 { panic!("web-sys intrinsic") }
}

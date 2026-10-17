#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrServiceTest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrServiceTest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrServiceTest(pub ::wasm_bindgen::JsValue);
    #[doc = "The `attachVRController()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest/attachVRController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrServiceTest`*"]
impl VrServiceTest {
    #[js::import = "$0.attachVRController($1)"]
pub fn attach_vr_controller(&self, id: &str) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `attachVRDisplay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRServiceTest/attachVRDisplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrServiceTest`*"]
impl VrServiceTest {
    #[js::import = "$0.attachVRDisplay($1)"]
pub fn attach_vr_display(&self, id: &str) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}

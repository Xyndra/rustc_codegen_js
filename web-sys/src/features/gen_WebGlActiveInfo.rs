#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlActiveInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebGlActiveInfo(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
impl WebGlActiveInfo {
    #[js::import = "$0.size"]
pub fn size(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
impl WebGlActiveInfo {
    #[js::import = "$0.type"]
pub fn type_(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
impl WebGlActiveInfo {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}

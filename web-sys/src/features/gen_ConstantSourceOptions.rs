#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstantSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConstantSourceOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
#[js::import = "None"]
pub fn get_offset(this: &ConstantSourceOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
#[js::import = "None = $1"]
pub fn set_offset(this: &ConstantSourceOptions, val: f32) { panic!("web-sys intrinsic") }
impl ConstantSourceOptions {
    #[doc = "Construct a new `ConstantSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_offset()` instead."]
#[js::import = "None"]
pub fn offset(&mut self, val: f32) -> &mut Self { self.set_offset(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConstantSourceOptions {
    fn default() -> Self {
        Self::new()
    }
}

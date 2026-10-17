#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ContextAttributes2d` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ContextAttributes2d(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
#[js::import = "None"]
pub fn get_alpha(this: &ContextAttributes2d) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
#[js::import = "None = $1"]
pub fn set_alpha(this: &ContextAttributes2d, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `willReadFrequently` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
#[js::import = "None"]
pub fn get_will_read_frequently(this: &ContextAttributes2d) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `willReadFrequently` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
#[js::import = "None = $1"]
pub fn set_will_read_frequently(this: &ContextAttributes2d, val: bool) { panic!("web-sys intrinsic") }
impl ContextAttributes2d {
    #[doc = "Construct a new `ContextAttributes2d`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_alpha()` instead."]
#[js::import = "None"]
pub fn alpha(&mut self, val: bool) -> &mut Self { self.set_alpha(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_will_read_frequently()` instead."]
#[js::import = "None"]
pub fn will_read_frequently(&mut self, val: bool) -> &mut Self { self.set_will_read_frequently(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ContextAttributes2d {
    fn default() -> Self {
        Self::new()
    }
}

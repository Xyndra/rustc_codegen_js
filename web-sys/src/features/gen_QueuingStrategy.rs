#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `QueuingStrategy` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
#[js::opaque]
#[derive(Copy)]
pub struct QueuingStrategy(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
#[js::import = "None"]
pub fn get_high_water_mark(this: &QueuingStrategy) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
#[js::import = "None = $1"]
pub fn set_high_water_mark(this: &QueuingStrategy, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
#[js::import = "None"]
pub fn get_size(this: &QueuingStrategy) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
#[js::import = "None = $1"]
pub fn set_size(this: &QueuingStrategy, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
impl QueuingStrategy {
    #[doc = "Construct a new `QueuingStrategy`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_high_water_mark()` instead."]
#[js::import = "None"]
pub fn high_water_mark(&mut self, val: f64) -> &mut Self { self.set_high_water_mark(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_size()` instead."]
#[js::import = "None"]
pub fn size(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_size(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for QueuingStrategy {
    fn default() -> Self {
        Self::new()
    }
}

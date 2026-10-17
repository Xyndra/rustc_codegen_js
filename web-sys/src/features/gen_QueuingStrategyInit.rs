#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `QueuingStrategyInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct QueuingStrategyInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
#[js::import = "None"]
pub fn get_high_water_mark(this: &QueuingStrategyInit) -> f64 { panic!("web-sys intrinsic") }
    #[doc = "Change the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
#[js::import = "None = $1"]
pub fn set_high_water_mark(this: &QueuingStrategyInit, val: f64) { panic!("web-sys intrinsic") }
impl QueuingStrategyInit {
    #[doc = "Construct a new `QueuingStrategyInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
#[js::import = "None"]
pub fn new(high_water_mark: f64) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_high_water_mark(high_water_mark);
        ret
    }
    #[deprecated = "Use `set_high_water_mark()` instead."]
#[js::import = "None"]
pub fn high_water_mark(&mut self, val: f64) -> &mut Self { self.set_high_water_mark(val) { panic!("web-sys intrinsic") }
        self
    }
}

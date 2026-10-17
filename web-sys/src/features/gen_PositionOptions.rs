#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PositionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PositionOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `enableHighAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
#[js::import = "None"]
pub fn get_enable_high_accuracy(this: &PositionOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `enableHighAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
#[js::import = "None = $1"]
pub fn set_enable_high_accuracy(this: &PositionOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `maximumAge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
#[js::import = "None"]
pub fn get_maximum_age(this: &PositionOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `maximumAge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
#[js::import = "None = $1"]
pub fn set_maximum_age(this: &PositionOptions, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
#[js::import = "None"]
pub fn get_timeout(this: &PositionOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
#[js::import = "None = $1"]
pub fn set_timeout(this: &PositionOptions, val: u32) { panic!("web-sys intrinsic") }
impl PositionOptions {
    #[doc = "Construct a new `PositionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_enable_high_accuracy()` instead."]
#[js::import = "None"]
pub fn enable_high_accuracy(&mut self, val: bool) -> &mut Self { self.set_enable_high_accuracy(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_maximum_age()` instead."]
#[js::import = "None"]
pub fn maximum_age(&mut self, val: u32) -> &mut Self { self.set_maximum_age(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_timeout()` instead."]
#[js::import = "None"]
pub fn timeout(&mut self, val: u32) -> &mut Self { self.set_timeout(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PositionOptions {
    fn default() -> Self {
        Self::new()
    }
}

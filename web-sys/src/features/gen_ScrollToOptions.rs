#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollToOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ScrollToOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Get the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollToOptions`*"]
#[js::import = "None"]
pub fn get_behavior(this: &ScrollToOptions) -> Option<ScrollBehavior> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollToOptions`*"]
#[js::import = "None = $1"]
pub fn set_behavior(this: &ScrollToOptions, val: ScrollBehavior) { panic!("web-sys intrinsic") }
    #[doc = "Get the `left` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
#[js::import = "None"]
pub fn get_left(this: &ScrollToOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `left` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
#[js::import = "None = $1"]
pub fn set_left(this: &ScrollToOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `top` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
#[js::import = "None"]
pub fn get_top(this: &ScrollToOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `top` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
#[js::import = "None = $1"]
pub fn set_top(this: &ScrollToOptions, val: f64) { panic!("web-sys intrinsic") }
impl ScrollToOptions {
    #[doc = "Construct a new `ScrollToOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "ScrollBehavior")]
    #[deprecated = "Use `set_behavior()` instead."]
#[js::import = "None"]
pub fn behavior(&mut self, val: ScrollBehavior) -> &mut Self { self.set_behavior(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_left()` instead."]
#[js::import = "None"]
pub fn left(&mut self, val: f64) -> &mut Self { self.set_left(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_top()` instead."]
#[js::import = "None"]
pub fn top(&mut self, val: f64) -> &mut Self { self.set_top(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ScrollToOptions {
    fn default() -> Self {
        Self::new()
    }
}

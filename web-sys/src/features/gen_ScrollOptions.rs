#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ScrollOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Get the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"]
#[js::import = "None"]
pub fn get_behavior(this: &ScrollOptions) -> Option<ScrollBehavior> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"]
#[js::import = "None = $1"]
pub fn set_behavior(this: &ScrollOptions, val: ScrollBehavior) { panic!("web-sys intrinsic") }
impl ScrollOptions {
    #[doc = "Construct a new `ScrollOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
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
}
impl Default for ScrollOptions {
    fn default() -> Self {
        Self::new()
    }
}

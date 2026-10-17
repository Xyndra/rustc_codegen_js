#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollIntoViewOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ScrollIntoViewOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Get the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollIntoViewOptions`*"]
#[js::import = "None"]
pub fn get_behavior(this: &ScrollIntoViewOptions) -> Option<ScrollBehavior> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollIntoViewOptions`*"]
#[js::import = "None = $1"]
pub fn set_behavior(this: &ScrollIntoViewOptions, val: ScrollBehavior) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Get the `block` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
#[js::import = "None"]
pub fn get_block(this: &ScrollIntoViewOptions) -> Option<ScrollLogicalPosition> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Change the `block` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
#[js::import = "None = $1"]
pub fn set_block(this: &ScrollIntoViewOptions, val: ScrollLogicalPosition) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Get the `inline` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
#[js::import = "None"]
pub fn get_inline(this: &ScrollIntoViewOptions) -> Option<ScrollLogicalPosition> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Change the `inline` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
#[js::import = "None = $1"]
pub fn set_inline(this: &ScrollIntoViewOptions, val: ScrollLogicalPosition) { panic!("web-sys intrinsic") }
impl ScrollIntoViewOptions {
    #[doc = "Construct a new `ScrollIntoViewOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"]
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
    #[cfg(feature = "ScrollLogicalPosition")]
    #[deprecated = "Use `set_block()` instead."]
#[js::import = "None"]
pub fn block(&mut self, val: ScrollLogicalPosition) -> &mut Self { self.set_block(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ScrollLogicalPosition")]
    #[deprecated = "Use `set_inline()` instead."]
#[js::import = "None"]
pub fn inline(&mut self, val: ScrollLogicalPosition) -> &mut Self { self.set_inline(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ScrollIntoViewOptions {
    fn default() -> Self {
        Self::new()
    }
}

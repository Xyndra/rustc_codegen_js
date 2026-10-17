#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomWindowResizeEventDetail` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomWindowResizeEventDetail(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
#[js::import = "None"]
pub fn get_height(this: &DomWindowResizeEventDetail) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
#[js::import = "None = $1"]
pub fn set_height(this: &DomWindowResizeEventDetail, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
#[js::import = "None"]
pub fn get_width(this: &DomWindowResizeEventDetail) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
#[js::import = "None = $1"]
pub fn set_width(this: &DomWindowResizeEventDetail, val: i32) { panic!("web-sys intrinsic") }
impl DomWindowResizeEventDetail {
    #[doc = "Construct a new `DomWindowResizeEventDetail`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_height()` instead."]
#[js::import = "None"]
pub fn height(&mut self, val: i32) -> &mut Self { self.set_height(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_width()` instead."]
#[js::import = "None"]
pub fn width(&mut self, val: i32) -> &mut Self { self.set_width(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DomWindowResizeEventDetail {
    fn default() -> Self {
        Self::new()
    }
}

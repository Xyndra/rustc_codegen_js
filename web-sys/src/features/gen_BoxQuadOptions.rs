#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BoxQuadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BoxQuadOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `CssBoxType`*"]
#[js::import = "None"]
pub fn get_box(this: &BoxQuadOptions) -> Option<CssBoxType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `CssBoxType`*"]
#[js::import = "None = $1"]
pub fn set_box(this: &BoxQuadOptions, val: CssBoxType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
#[js::import = "None"]
pub fn get_relative_to(this: &BoxQuadOptions) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
#[js::import = "None = $1"]
pub fn set_relative_to(this: &BoxQuadOptions, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
impl BoxQuadOptions {
    #[doc = "Construct a new `BoxQuadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "CssBoxType")]
    #[deprecated = "Use `set_box()` instead."]
#[js::import = "None"]
pub fn box_(&mut self, val: CssBoxType) -> &mut Self { self.set_box(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_relative_to()` instead."]
#[js::import = "None"]
pub fn relative_to(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_relative_to(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BoxQuadOptions {
    fn default() -> Self {
        Self::new()
    }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConvertCoordinateOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConvertCoordinateOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `fromBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
#[js::import = "None"]
pub fn get_from_box(this: &ConvertCoordinateOptions) -> Option<CssBoxType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `fromBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
#[js::import = "None = $1"]
pub fn set_from_box(this: &ConvertCoordinateOptions, val: CssBoxType) { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `toBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
#[js::import = "None"]
pub fn get_to_box(this: &ConvertCoordinateOptions) -> Option<CssBoxType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `toBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
#[js::import = "None = $1"]
pub fn set_to_box(this: &ConvertCoordinateOptions, val: CssBoxType) { panic!("web-sys intrinsic") }
impl ConvertCoordinateOptions {
    #[doc = "Construct a new `ConvertCoordinateOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "CssBoxType")]
    #[deprecated = "Use `set_from_box()` instead."]
#[js::import = "None"]
pub fn from_box(&mut self, val: CssBoxType) -> &mut Self { self.set_from_box(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "CssBoxType")]
    #[deprecated = "Use `set_to_box()` instead."]
#[js::import = "None"]
pub fn to_box(&mut self, val: CssBoxType) -> &mut Self { self.set_to_box(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConvertCoordinateOptions {
    fn default() -> Self {
        Self::new()
    }
}

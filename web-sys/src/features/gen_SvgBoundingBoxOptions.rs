#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgBoundingBoxOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgBoundingBoxOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `clipped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::import = "None"]
pub fn get_clipped(this: &SvgBoundingBoxOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `clipped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::import = "None = $1"]
pub fn set_clipped(this: &SvgBoundingBoxOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::import = "None"]
pub fn get_fill(this: &SvgBoundingBoxOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::import = "None = $1"]
pub fn set_fill(this: &SvgBoundingBoxOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `markers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::import = "None"]
pub fn get_markers(this: &SvgBoundingBoxOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `markers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::import = "None = $1"]
pub fn set_markers(this: &SvgBoundingBoxOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `stroke` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::import = "None"]
pub fn get_stroke(this: &SvgBoundingBoxOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `stroke` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::import = "None = $1"]
pub fn set_stroke(this: &SvgBoundingBoxOptions, val: bool) { panic!("web-sys intrinsic") }
impl SvgBoundingBoxOptions {
    #[doc = "Construct a new `SvgBoundingBoxOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_clipped()` instead."]
#[js::import = "None"]
pub fn clipped(&mut self, val: bool) -> &mut Self { self.set_clipped(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_fill()` instead."]
#[js::import = "None"]
pub fn fill(&mut self, val: bool) -> &mut Self { self.set_fill(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_markers()` instead."]
#[js::import = "None"]
pub fn markers(&mut self, val: bool) -> &mut Self { self.set_markers(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_stroke()` instead."]
#[js::import = "None"]
pub fn stroke(&mut self, val: bool) -> &mut Self { self.set_stroke(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for SvgBoundingBoxOptions {
    fn default() -> Self {
        Self::new()
    }
}

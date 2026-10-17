#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageEncodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ImageEncodeOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `quality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
#[js::import = "None"]
pub fn get_quality(this: &ImageEncodeOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `quality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_quality(this: &ImageEncodeOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
#[js::import = "None"]
pub fn get_type(this: &ImageEncodeOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &ImageEncodeOptions, val: &str) { panic!("web-sys intrinsic") }
impl ImageEncodeOptions {
    #[doc = "Construct a new `ImageEncodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_quality()` instead."]
#[js::import = "None"]
pub fn quality(&mut self, val: f64) -> &mut Self { self.set_quality(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: &str) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ImageEncodeOptions {
    fn default() -> Self {
        Self::new()
    }
}

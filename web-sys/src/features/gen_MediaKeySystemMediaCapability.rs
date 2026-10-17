#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemMediaCapability` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeySystemMediaCapability(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
#[js::import = "None"]
pub fn get_content_type(this: &MediaKeySystemMediaCapability,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
#[js::import = "None = $1"]
pub fn set_content_type(this: &MediaKeySystemMediaCapability, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `robustness` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
#[js::import = "None"]
pub fn get_robustness(this: &MediaKeySystemMediaCapability) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `robustness` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
#[js::import = "None = $1"]
pub fn set_robustness(this: &MediaKeySystemMediaCapability, val: &str) { panic!("web-sys intrinsic") }
impl MediaKeySystemMediaCapability {
    #[doc = "Construct a new `MediaKeySystemMediaCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_content_type()` instead."]
#[js::import = "None"]
pub fn content_type(&mut self, val: &str) -> &mut Self { self.set_content_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_robustness()` instead."]
#[js::import = "None"]
pub fn robustness(&mut self, val: &str) -> &mut Self { self.set_robustness(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaKeySystemMediaCapability {
    fn default() -> Self {
        Self::new()
    }
}

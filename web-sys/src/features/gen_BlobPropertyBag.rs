#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlobPropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BlobPropertyBag(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "EndingTypes")]
    #[doc = "Get the `endings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*"]
#[js::import = "None"]
pub fn get_endings(this: &BlobPropertyBag) -> Option<EndingTypes> { panic!("web-sys intrinsic") }
    #[cfg(feature = "EndingTypes")]
    #[doc = "Change the `endings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*"]
#[js::import = "None = $1"]
pub fn set_endings(this: &BlobPropertyBag, val: EndingTypes) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
#[js::import = "None"]
pub fn get_type(this: &BlobPropertyBag) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &BlobPropertyBag, val: &str) { panic!("web-sys intrinsic") }
impl BlobPropertyBag {
    #[doc = "Construct a new `BlobPropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "EndingTypes")]
    #[deprecated = "Use `set_endings()` instead."]
#[js::import = "None"]
pub fn endings(&mut self, val: EndingTypes) -> &mut Self { self.set_endings(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: &str) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BlobPropertyBag {
    fn default() -> Self {
        Self::new()
    }
}

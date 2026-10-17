#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OpenWindowEventDetail` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OpenWindowEventDetail(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `features` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
#[js::import = "None"]
pub fn get_features(this: &OpenWindowEventDetail) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `features` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
#[js::import = "None = $1"]
pub fn set_features(this: &OpenWindowEventDetail, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Node")]
    #[doc = "Get the `frameElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `OpenWindowEventDetail`*"]
#[js::import = "None"]
pub fn get_frame_element(this: &OpenWindowEventDetail) -> Option<Node> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Node")]
    #[doc = "Change the `frameElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `OpenWindowEventDetail`*"]
#[js::import = "None = $1"]
pub fn set_frame_element(this: &OpenWindowEventDetail, val: Option<&Node>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
#[js::import = "None"]
pub fn get_name(this: &OpenWindowEventDetail) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &OpenWindowEventDetail, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
#[js::import = "None"]
pub fn get_url(this: &OpenWindowEventDetail) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
#[js::import = "None = $1"]
pub fn set_url(this: &OpenWindowEventDetail, val: &str) { panic!("web-sys intrinsic") }
impl OpenWindowEventDetail {
    #[doc = "Construct a new `OpenWindowEventDetail`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_features()` instead."]
#[js::import = "None"]
pub fn features(&mut self, val: &str) -> &mut Self { self.set_features(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "Node")]
    #[deprecated = "Use `set_frame_element()` instead."]
#[js::import = "None"]
pub fn frame_element(&mut self, val: Option<&Node>) -> &mut Self { self.set_frame_element(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_url()` instead."]
#[js::import = "None"]
pub fn url(&mut self, val: &str) -> &mut Self { self.set_url(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for OpenWindowEventDetail {
    fn default() -> Self {
        Self::new()
    }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DocumentTimelineOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimelineOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DocumentTimelineOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `originTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimelineOptions`*"]
#[js::import = "None"]
pub fn get_origin_time(this: &DocumentTimelineOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `originTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimelineOptions`*"]
#[js::import = "None = $1"]
pub fn set_origin_time(this: &DocumentTimelineOptions, val: f64) { panic!("web-sys intrinsic") }
impl DocumentTimelineOptions {
    #[doc = "Construct a new `DocumentTimelineOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimelineOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_origin_time()` instead."]
#[js::import = "None"]
pub fn origin_time(&mut self, val: f64) -> &mut Self { self.set_origin_time(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DocumentTimelineOptions {
    fn default() -> Self {
        Self::new()
    }
}

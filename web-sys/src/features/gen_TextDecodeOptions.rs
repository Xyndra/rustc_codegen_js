#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextDecodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TextDecodeOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
#[js::import = "None"]
pub fn get_stream(this: &TextDecodeOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_stream(this: &TextDecodeOptions, val: bool) { panic!("web-sys intrinsic") }
impl TextDecodeOptions {
    #[doc = "Construct a new `TextDecodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_stream()` instead."]
#[js::import = "None"]
pub fn stream(&mut self, val: bool) -> &mut Self { self.set_stream(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for TextDecodeOptions {
    fn default() -> Self {
        Self::new()
    }
}

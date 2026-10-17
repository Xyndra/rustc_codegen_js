#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextDecoderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TextDecoderOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `fatal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
#[js::import = "None"]
pub fn get_fatal(this: &TextDecoderOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `fatal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
#[js::import = "None = $1"]
pub fn set_fatal(this: &TextDecoderOptions, val: bool) { panic!("web-sys intrinsic") }
impl TextDecoderOptions {
    #[doc = "Construct a new `TextDecoderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_fatal()` instead."]
#[js::import = "None"]
pub fn fatal(&mut self, val: bool) -> &mut Self { self.set_fatal(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for TextDecoderOptions {
    fn default() -> Self {
        Self::new()
    }
}

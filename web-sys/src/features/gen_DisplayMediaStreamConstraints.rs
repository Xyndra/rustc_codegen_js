#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayMediaStreamConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DisplayMediaStreamConstraints(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
#[js::import = "None"]
pub fn get_audio(this: &DisplayMediaStreamConstraints) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
#[js::import = "None = $1"]
pub fn set_audio(this: &DisplayMediaStreamConstraints, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
#[js::import = "None"]
pub fn get_video(this: &DisplayMediaStreamConstraints) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
#[js::import = "None = $1"]
pub fn set_video(this: &DisplayMediaStreamConstraints, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl DisplayMediaStreamConstraints {
    #[doc = "Construct a new `DisplayMediaStreamConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_audio()` instead."]
#[js::import = "None"]
pub fn audio(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_audio(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_video()` instead."]
#[js::import = "None"]
pub fn video(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_video(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DisplayMediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionErrorInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechRecognitionErrorInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &SpeechRecognitionErrorInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &SpeechRecognitionErrorInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &SpeechRecognitionErrorInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &SpeechRecognitionErrorInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &SpeechRecognitionErrorInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &SpeechRecognitionErrorInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorCode`, `SpeechRecognitionErrorInit`*"]
#[js::import = "None"]
pub fn get_error(this: &SpeechRecognitionErrorInit) -> Option<SpeechRecognitionErrorCode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorCode`, `SpeechRecognitionErrorInit`*"]
#[js::import = "None = $1"]
pub fn set_error(this: &SpeechRecognitionErrorInit, val: SpeechRecognitionErrorCode) { panic!("web-sys intrinsic") }
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::import = "None"]
pub fn get_message(this: &SpeechRecognitionErrorInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::import = "None = $1"]
pub fn set_message(this: &SpeechRecognitionErrorInit, val: &str) { panic!("web-sys intrinsic") }
impl SpeechRecognitionErrorInit {
    #[doc = "Construct a new `SpeechRecognitionErrorInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[deprecated = "Use `set_error()` instead."]
#[js::import = "None"]
pub fn error(&mut self, val: SpeechRecognitionErrorCode) -> &mut Self { self.set_error(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_message()` instead."]
#[js::import = "None"]
pub fn message(&mut self, val: &str) -> &mut Self { self.set_message(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for SpeechRecognitionErrorInit {
    fn default() -> Self {
        Self::new()
    }
}

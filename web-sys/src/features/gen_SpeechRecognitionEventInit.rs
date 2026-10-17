#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechRecognitionEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &SpeechRecognitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &SpeechRecognitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &SpeechRecognitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &SpeechRecognitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &SpeechRecognitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &SpeechRecognitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Document")]
    #[doc = "Get the `emma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEventInit`*"]
#[js::import = "None"]
pub fn get_emma(this: &SpeechRecognitionEventInit) -> Option<Document> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Document")]
    #[doc = "Change the `emma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_emma(this: &SpeechRecognitionEventInit, val: Option<&Document>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None"]
pub fn get_interpretation(this: &SpeechRecognitionEventInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_interpretation(this: &SpeechRecognitionEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None"]
pub fn get_result_index(this: &SpeechRecognitionEventInit) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_result_index(this: &SpeechRecognitionEventInit, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Get the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`, `SpeechRecognitionResultList`*"]
#[js::import = "None"]
pub fn get_results(this: &SpeechRecognitionEventInit) -> Option<SpeechRecognitionResultList> { panic!("web-sys intrinsic") }
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Change the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`, `SpeechRecognitionResultList`*"]
#[js::import = "None = $1"]
pub fn set_results(this: &SpeechRecognitionEventInit, val: Option<&SpeechRecognitionResultList>,) { panic!("web-sys intrinsic") }
impl SpeechRecognitionEventInit {
    #[doc = "Construct a new `SpeechRecognitionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
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
    #[cfg(feature = "Document")]
    #[deprecated = "Use `set_emma()` instead."]
#[js::import = "None"]
pub fn emma(&mut self, val: Option<&Document>) -> &mut Self { self.set_emma(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_interpretation()` instead."]
#[js::import = "None"]
pub fn interpretation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_interpretation(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_result_index()` instead."]
#[js::import = "None"]
pub fn result_index(&mut self, val: u32) -> &mut Self { self.set_result_index(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[deprecated = "Use `set_results()` instead."]
#[js::import = "None"]
pub fn results(&mut self, val: Option<&SpeechRecognitionResultList>) -> &mut Self { self.set_results(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for SpeechRecognitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}

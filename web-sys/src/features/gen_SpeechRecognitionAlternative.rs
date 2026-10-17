#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionAlternative` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechRecognitionAlternative(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `transcript` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/transcript)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
impl SpeechRecognitionAlternative {
    #[js::import = "$0.transcript"]
pub fn transcript(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `confidence` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/confidence)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
impl SpeechRecognitionAlternative {
    #[js::import = "$0.confidence"]
pub fn confidence(&self) -> f32 { panic!("web-sys intrinsic") }
}

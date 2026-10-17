#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResult`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechRecognitionResult(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResult`*"]
impl SpeechRecognitionResult {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isFinal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/isFinal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResult`*"]
impl SpeechRecognitionResult {
    #[js::import = "$0.isFinal"]
pub fn is_final(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechRecognitionAlternative")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`, `SpeechRecognitionResult`*"]
impl SpeechRecognitionResult {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> SpeechRecognitionAlternative { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechRecognitionAlternative")]
        method,
        structural,
        js_class = "SpeechRecognitionResult",
        indexing_getter
    )]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`, `SpeechRecognitionResult`*"]
#[js::import = "None"]
pub fn get(this: &SpeechRecognitionResult, index: u32) -> Option<SpeechRecognitionAlternative> { panic!("web-sys intrinsic") }

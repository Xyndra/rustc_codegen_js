#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionResultList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechRecognitionResultList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*"]
impl SpeechRecognitionResultList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechRecognitionResult")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*"]
impl SpeechRecognitionResultList {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> SpeechRecognitionResult { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechRecognitionResult")]
        method,
        structural,
        js_class = "SpeechRecognitionResultList",
        indexing_getter
    )]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*"]
#[js::import = "None"]
pub fn get(this: &SpeechRecognitionResultList, index: u32) -> Option<SpeechRecognitionResult> { panic!("web-sys intrinsic") }

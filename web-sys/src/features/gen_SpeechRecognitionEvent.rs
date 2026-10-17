#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechRecognitionEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/resultIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
impl SpeechRecognitionEvent {
    #[js::import = "$0.resultIndex"]
pub fn result_index(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Getter for the `results` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/results)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`, `SpeechRecognitionResultList`*"]
impl SpeechRecognitionEvent {
    #[js::import = "$0.results"]
pub fn results(&self) -> Option<SpeechRecognitionResultList> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/interpretation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
impl SpeechRecognitionEvent {
    #[js::import = "$0.interpretation"]
pub fn interpretation(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `emma` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/emma)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEvent`*"]
impl SpeechRecognitionEvent {
    #[js::import = "$0.emma"]
pub fn emma(&self) -> Option<Document> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SpeechRecognitionEvent(..)` constructor, creating a new instance of `SpeechRecognitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/SpeechRecognitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
impl SpeechRecognitionEvent {
    #[js::import = "new SpeechRecognitionEvent($0)"]
pub fn new(type_: &str) -> Result<SpeechRecognitionEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechRecognitionEventInit")]
    #[doc = "The `new SpeechRecognitionEvent(..)` constructor, creating a new instance of `SpeechRecognitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/SpeechRecognitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`, `SpeechRecognitionEventInit`*"]
impl SpeechRecognitionEvent {
    #[js::import = "new SpeechRecognitionEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &SpeechRecognitionEventInit,) -> Result<SpeechRecognitionEvent, JsValue> { panic!("web-sys intrinsic") }
}

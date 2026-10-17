#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechRecognitionError(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`, `SpeechRecognitionErrorCode`*"]
impl SpeechRecognitionError {
    #[js::import = "$0.error"]
pub fn error(&self) -> SpeechRecognitionErrorCode { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`*"]
impl SpeechRecognitionError {
    #[js::import = "$0.message"]
pub fn message(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SpeechRecognitionError(..)` constructor, creating a new instance of `SpeechRecognitionError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/SpeechRecognitionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`*"]
impl SpeechRecognitionError {
    #[js::import = "new SpeechRecognitionError($0)"]
pub fn new(type_: &str) -> Result<SpeechRecognitionError, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechRecognitionErrorInit")]
    #[doc = "The `new SpeechRecognitionError(..)` constructor, creating a new instance of `SpeechRecognitionError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionError/SpeechRecognitionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionError`, `SpeechRecognitionErrorInit`*"]
impl SpeechRecognitionError {
    #[js::import = "new SpeechRecognitionError($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &SpeechRecognitionErrorInit,) -> Result<SpeechRecognitionError, JsValue> { panic!("web-sys intrinsic") }
}

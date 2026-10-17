#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechSynthesisErrorEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SpeechSynthesisErrorCode")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEvent`*"]
impl SpeechSynthesisErrorEvent {
    #[js::import = "$0.error"]
pub fn error(&self) -> SpeechSynthesisErrorCode { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechSynthesisErrorEventInit")]
    #[doc = "The `new SpeechSynthesisErrorEvent(..)` constructor, creating a new instance of `SpeechSynthesisErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent/SpeechSynthesisErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisErrorEvent`, `SpeechSynthesisErrorEventInit`*"]
impl SpeechSynthesisErrorEvent {
    #[js::import = "new SpeechSynthesisErrorEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &SpeechSynthesisErrorEventInit,) -> Result<SpeechSynthesisErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechSynthesisEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Getter for the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/utterance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisEvent {
    #[js::import = "$0.utterance"]
pub fn utterance(&self) -> SpeechSynthesisUtterance { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
impl SpeechSynthesisEvent {
    #[js::import = "$0.charIndex"]
pub fn char_index(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
impl SpeechSynthesisEvent {
    #[js::import = "$0.charLength"]
pub fn char_length(&self) -> Option<u32> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/elapsedTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
impl SpeechSynthesisEvent {
    #[js::import = "$0.elapsedTime"]
pub fn elapsed_time(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
impl SpeechSynthesisEvent {
    #[js::import = "$0.name"]
pub fn name(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechSynthesisEventInit")]
    #[doc = "The `new SpeechSynthesisEvent(..)` constructor, creating a new instance of `SpeechSynthesisEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/SpeechSynthesisEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisEventInit`*"]
impl SpeechSynthesisEvent {
    #[js::import = "new SpeechSynthesisEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &SpeechSynthesisEventInit,) -> Result<SpeechSynthesisEvent, JsValue> { panic!("web-sys intrinsic") }
}

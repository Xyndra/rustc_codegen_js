#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechRecognition(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SpeechGrammarList")]
    #[doc = "Getter for the `grammars` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.grammars"]
pub fn grammars(&self) -> SpeechGrammarList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechGrammarList")]
    #[doc = "Setter for the `grammars` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.grammars = $1"]
pub fn set_grammars(&self, value: &SpeechGrammarList) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.lang"]
pub fn lang(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.lang = $1"]
pub fn set_lang(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `continuous` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.continuous"]
pub fn continuous(&self) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `continuous` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.continuous = $1"]
pub fn set_continuous(&self, value: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `interimResults` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.interimResults"]
pub fn interim_results(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `interimResults` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.interimResults = $1"]
pub fn set_interim_results(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxAlternatives` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.maxAlternatives"]
pub fn max_alternatives(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `maxAlternatives` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.maxAlternatives = $1"]
pub fn set_max_alternatives(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `serviceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.serviceURI"]
pub fn service_uri(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `serviceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.serviceURI = $1"]
pub fn set_service_uri(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onaudiostart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onaudiostart"]
pub fn onaudiostart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onaudiostart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onaudiostart = $1"]
pub fn set_onaudiostart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsoundstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onsoundstart"]
pub fn onsoundstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsoundstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onsoundstart = $1"]
pub fn set_onsoundstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onspeechstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onspeechstart"]
pub fn onspeechstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onspeechstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onspeechstart = $1"]
pub fn set_onspeechstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onspeechend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onspeechend"]
pub fn onspeechend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onspeechend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onspeechend = $1"]
pub fn set_onspeechend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsoundend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onsoundend"]
pub fn onsoundend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsoundend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onsoundend = $1"]
pub fn set_onsoundend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onaudioend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onaudioend"]
pub fn onaudioend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onaudioend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onaudioend = $1"]
pub fn set_onaudioend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onresult` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onresult"]
pub fn onresult(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onresult` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onresult = $1"]
pub fn set_onresult(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onnomatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onnomatch"]
pub fn onnomatch(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onnomatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onnomatch = $1"]
pub fn set_onnomatch(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onstart"]
pub fn onstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onstart = $1"]
pub fn set_onstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onend"]
pub fn onend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.onend = $1"]
pub fn set_onend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SpeechRecognition(..)` constructor, creating a new instance of `SpeechRecognition`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/SpeechRecognition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "new SpeechRecognition()"]
pub fn new() -> Result<SpeechRecognition, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.abort()"]
pub fn abort(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.start()"]
pub fn start(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStream")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.start($1)"]
pub fn start_with_stream(&self, stream: &MediaStream) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
impl SpeechRecognition {
    #[js::import = "$0.stop()"]
pub fn stop(&self) { panic!("web-sys intrinsic") }
}

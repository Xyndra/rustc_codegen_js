#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisVoice` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechSynthesisVoice(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `voiceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/voiceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
impl SpeechSynthesisVoice {
    #[js::import = "$0.voiceURI"]
pub fn voice_uri(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
impl SpeechSynthesisVoice {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
impl SpeechSynthesisVoice {
    #[js::import = "$0.lang"]
pub fn lang(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `localService` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/localService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
impl SpeechSynthesisVoice {
    #[js::import = "$0.localService"]
pub fn local_service(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `default` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/default)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisVoice`*"]
impl SpeechSynthesisVoice {
    #[js::import = "$0.default"]
pub fn default(&self) -> bool { panic!("web-sys intrinsic") }
}

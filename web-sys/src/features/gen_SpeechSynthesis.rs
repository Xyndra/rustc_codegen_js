#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesis` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechSynthesis(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `pending` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pending)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
impl SpeechSynthesis {
    #[js::import = "$0.pending"]
pub fn pending(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `speaking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speaking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
impl SpeechSynthesis {
    #[js::import = "$0.speaking"]
pub fn speaking(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `paused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/paused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
impl SpeechSynthesis {
    #[js::import = "$0.paused"]
pub fn paused(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvoiceschanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
impl SpeechSynthesis {
    #[js::import = "$0.onvoiceschanged"]
pub fn onvoiceschanged(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvoiceschanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
impl SpeechSynthesis {
    #[js::import = "$0.onvoiceschanged = $1"]
pub fn set_onvoiceschanged(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
impl SpeechSynthesis {
    #[js::import = "$0.cancel()"]
pub fn cancel(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getVoices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/getVoices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
impl SpeechSynthesis {
    #[js::import = "$0.getVoices()"]
pub fn get_voices(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `pause()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
impl SpeechSynthesis {
    #[js::import = "$0.pause()"]
pub fn pause(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
impl SpeechSynthesis {
    #[js::import = "$0.resume()"]
pub fn resume(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "The `speak()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speak)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`, `SpeechSynthesisUtterance`*"]
impl SpeechSynthesis {
    #[js::import = "$0.speak($1)"]
pub fn speak(&self, utterance: &SpeechSynthesisUtterance) { panic!("web-sys intrinsic") }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisUtterance` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechSynthesisUtterance(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.text"]
pub fn text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.text = $1"]
pub fn set_text(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.lang"]
pub fn lang(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.lang = $1"]
pub fn set_lang(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechSynthesisVoice")]
    #[doc = "Getter for the `voice` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`, `SpeechSynthesisVoice`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.voice"]
pub fn voice(&self) -> Option<SpeechSynthesisVoice> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechSynthesisVoice")]
    #[doc = "Setter for the `voice` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`, `SpeechSynthesisVoice`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.voice = $1"]
pub fn set_voice(&self, value: Option<&SpeechSynthesisVoice>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `volume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.volume"]
pub fn volume(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `volume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.volume = $1"]
pub fn set_volume(&self, value: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.rate"]
pub fn rate(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `rate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.rate = $1"]
pub fn set_rate(&self, value: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pitch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.pitch"]
pub fn pitch(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `pitch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.pitch = $1"]
pub fn set_pitch(&self, value: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onstart"]
pub fn onstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onstart = $1"]
pub fn set_onstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onend"]
pub fn onend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onend = $1"]
pub fn set_onend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onpause"]
pub fn onpause(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onpause = $1"]
pub fn set_onpause(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onresume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onresume"]
pub fn onresume(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onresume` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onresume = $1"]
pub fn set_onresume(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onmark"]
pub fn onmark(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onmark = $1"]
pub fn set_onmark(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onboundary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onboundary"]
pub fn onboundary(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onboundary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "$0.onboundary = $1"]
pub fn set_onboundary(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SpeechSynthesisUtterance(..)` constructor, creating a new instance of `SpeechSynthesisUtterance`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/SpeechSynthesisUtterance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "new SpeechSynthesisUtterance()"]
pub fn new() -> Result<SpeechSynthesisUtterance, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SpeechSynthesisUtterance(..)` constructor, creating a new instance of `SpeechSynthesisUtterance`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/SpeechSynthesisUtterance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisUtterance`*"]
impl SpeechSynthesisUtterance {
    #[js::import = "new SpeechSynthesisUtterance($0)"]
pub fn new_with_text(text: &str) -> Result<SpeechSynthesisUtterance, JsValue> { panic!("web-sys intrinsic") }
}

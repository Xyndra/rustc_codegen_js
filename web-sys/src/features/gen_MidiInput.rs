#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiInput` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInput`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MidiInput(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `onmidimessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInput`*"]
impl MidiInput {
    #[js::import = "$0.onmidimessage"]
pub fn onmidimessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmidimessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInput/onmidimessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInput`*"]
impl MidiInput {
    #[js::import = "$0.onmidimessage = $1"]
pub fn set_onmidimessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiAccess` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiAccess`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MidiAccess(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MidiInputMap")]
    #[doc = "Getter for the `inputs` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/inputs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiAccess`, `MidiInputMap`*"]
impl MidiAccess {
    #[js::import = "$0.inputs"]
pub fn inputs(&self) -> MidiInputMap { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MidiOutputMap")]
    #[doc = "Getter for the `outputs` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/outputs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiAccess`, `MidiOutputMap`*"]
impl MidiAccess {
    #[js::import = "$0.outputs"]
pub fn outputs(&self) -> MidiOutputMap { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiAccess`*"]
impl MidiAccess {
    #[js::import = "$0.onstatechange"]
pub fn onstatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiAccess`*"]
impl MidiAccess {
    #[js::import = "$0.onstatechange = $1"]
pub fn set_onstatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sysexEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIAccess/sysexEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiAccess`*"]
impl MidiAccess {
    #[js::import = "$0.sysexEnabled"]
pub fn sysex_enabled(&self) -> bool { panic!("web-sys intrinsic") }
}

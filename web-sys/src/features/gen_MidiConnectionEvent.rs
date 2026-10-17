#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiConnectionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MidiConnectionEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MidiPort")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiPort`*"]
impl MidiConnectionEvent {
    #[js::import = "$0.port"]
pub fn port(&self) -> Option<MidiPort> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MidiConnectionEvent(..)` constructor, creating a new instance of `MidiConnectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEvent`*"]
impl MidiConnectionEvent {
    #[js::import = "new MIDIConnectionEvent($0)"]
pub fn new(type_: &str) -> Result<MidiConnectionEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MidiConnectionEventInit")]
    #[doc = "The `new MidiConnectionEvent(..)` constructor, creating a new instance of `MidiConnectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIConnectionEvent/MIDIConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEvent`, `MidiConnectionEventInit`*"]
impl MidiConnectionEvent {
    #[js::import = "new MIDIConnectionEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &MidiConnectionEventInit,) -> Result<MidiConnectionEvent, JsValue> { panic!("web-sys intrinsic") }
}

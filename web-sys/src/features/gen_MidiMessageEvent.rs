#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiMessageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiMessageEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MidiMessageEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiMessageEvent`*"]
impl MidiMessageEvent {
    #[js::import = "$0.data"]
pub fn data(&self) -> Result<::alloc::vec::Vec<u8>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MidiMessageEvent(..)` constructor, creating a new instance of `MidiMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/MIDIMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiMessageEvent`*"]
impl MidiMessageEvent {
    #[js::import = "new MIDIMessageEvent($0)"]
pub fn new(type_: &str) -> Result<MidiMessageEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MidiMessageEventInit")]
    #[doc = "The `new MidiMessageEvent(..)` constructor, creating a new instance of `MidiMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIMessageEvent/MIDIMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiMessageEvent`, `MidiMessageEventInit`*"]
impl MidiMessageEvent {
    #[js::import = "new MIDIMessageEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &MidiMessageEventInit,) -> Result<MidiMessageEvent, JsValue> { panic!("web-sys intrinsic") }
}

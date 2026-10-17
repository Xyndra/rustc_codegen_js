#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiPort` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MidiPort(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
impl MidiPort {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `manufacturer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/manufacturer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
impl MidiPort {
    #[js::import = "$0.manufacturer"]
pub fn manufacturer(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
impl MidiPort {
    #[js::import = "$0.name"]
pub fn name(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
impl MidiPort {
    #[js::import = "$0.version"]
pub fn version(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MidiPortType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`, `MidiPortType`*"]
impl MidiPort {
    #[js::import = "$0.type"]
pub fn type_(&self) -> MidiPortType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MidiPortDeviceState")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`, `MidiPortDeviceState`*"]
impl MidiPort {
    #[js::import = "$0.state"]
pub fn state(&self) -> MidiPortDeviceState { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MidiPortConnectionState")]
    #[doc = "Getter for the `connection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/connection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`, `MidiPortConnectionState`*"]
impl MidiPort {
    #[js::import = "$0.connection"]
pub fn connection(&self) -> MidiPortConnectionState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
impl MidiPort {
    #[js::import = "$0.onstatechange"]
pub fn onstatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
impl MidiPort {
    #[js::import = "$0.onstatechange = $1"]
pub fn set_onstatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
impl MidiPort {
    #[js::import = "$0.close()"]
pub fn close(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIPort/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiPort`*"]
impl MidiPort {
    #[js::import = "$0.open()"]
pub fn open(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}

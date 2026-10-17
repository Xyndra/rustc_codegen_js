#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiInputMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MidiInputMap(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
impl MidiInputMap {
    #[js::import = "$0.size"]
pub fn size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
impl MidiInputMap {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
impl MidiInputMap {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MidiInput")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInput`, `MidiInputMap`*"]
impl MidiInputMap {
    #[js::import = "$0.get($1)"]
pub fn get(&self, key: &str) -> Option<MidiInput> { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
impl MidiInputMap {
    #[js::import = "$0.has($1)"]
pub fn has(&self, key: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
impl MidiInputMap {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIInputMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiInputMap`*"]
impl MidiInputMap {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}

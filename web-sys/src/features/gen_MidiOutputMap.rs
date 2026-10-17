#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiOutputMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MidiOutputMap(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
impl MidiOutputMap {
    #[js::import = "$0.size"]
pub fn size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
impl MidiOutputMap {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
impl MidiOutputMap {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MidiOutput")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`, `MidiOutputMap`*"]
impl MidiOutputMap {
    #[js::import = "$0.get($1)"]
pub fn get(&self, key: &str) -> Option<MidiOutput> { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
impl MidiOutputMap {
    #[js::import = "$0.has($1)"]
pub fn has(&self, key: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
impl MidiOutputMap {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutputMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutputMap`*"]
impl MidiOutputMap {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}

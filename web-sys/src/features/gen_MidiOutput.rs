#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiOutput` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MidiOutput(pub ::wasm_bindgen::JsValue);
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`*"]
impl MidiOutput {
    #[js::import = "$0.clear()"]
pub fn clear(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`*"]
impl MidiOutput {
    #[js::import = "$0.send($1)"]
pub fn send(&self, data: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MIDIOutput/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOutput`*"]
impl MidiOutput {
    #[js::import = "$0.send($1, $2)"]
pub fn send_with_timestamp(&self, data: &::wasm_bindgen::JsValue, timestamp: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}

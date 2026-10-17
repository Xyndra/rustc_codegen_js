#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MidiConnectionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MidiConnectionEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &MidiConnectionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &MidiConnectionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &MidiConnectionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &MidiConnectionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &MidiConnectionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &MidiConnectionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MidiPort")]
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`, `MidiPort`*"]
#[js::import = "None"]
pub fn get_port(this: &MidiConnectionEventInit) -> Option<MidiPort> { panic!("web-sys intrinsic") }
    #[cfg(feature = "MidiPort")]
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`, `MidiPort`*"]
#[js::import = "None = $1"]
pub fn set_port(this: &MidiConnectionEventInit, val: Option<&MidiPort>) { panic!("web-sys intrinsic") }
impl MidiConnectionEventInit {
    #[doc = "Construct a new `MidiConnectionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiConnectionEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "MidiPort")]
    #[deprecated = "Use `set_port()` instead."]
#[js::import = "None"]
pub fn port(&mut self, val: Option<&MidiPort>) -> &mut Self { self.set_port(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MidiConnectionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
